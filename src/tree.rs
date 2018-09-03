/*  Copyright (C) 2018 Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.*/
use super::token::{parse_line, parse_tokens, Token};
use db::{
    assign_new_id, find_definition, find_normal_form, id_from_label, insert_definition,
    insert_reduction3, label_from_id, label_id, refactor_id, select_definition, select_integer,
    unlabel, DBError, SqliteConnection, ZiaResult, DEFINE, LUID, REDUCTION,
};

#[derive(Clone)]
pub struct Tree {
    id: i32,
    applicand: Option<Box<Tree>>,
    argument: Option<Box<Tree>>,
}

pub fn extract_tree_from_token(token: &Token, conn: &SqliteConnection) -> ZiaResult<Tree> {
    match token {
        Token::Atom(t) => extract_tree_from_atom(t, conn),
        Token::Expression(t) => extract_tree_from_expression(t, conn),
    }
}

fn extract_tree_from_expression(t: &str, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() {
        0 | 1 => Err(DBError::Syntax(
            "Expression needs to be composed of multiple tokens".to_string(),
        )),
        2 => {
            let parsed_tokens = parse_tokens(&tokens);
            let applicand = try!(extract_tree_from_token(&parsed_tokens[0], conn));
            let argument = try!(extract_tree_from_token(&parsed_tokens[1], conn));
            Ok(try!(Tree::new_definition(
                Box::new(applicand),
                Box::new(argument),
                conn
            )))
        }
        _ => Err(DBError::Syntax(
            "Expression composed of more than 2 tokens has not been implemented yet".to_string(),
        )),
    }
}

fn extract_tree_from_atom(t: &str, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let id_if_exists = try!(id_from_label(&t, conn));
    match id_if_exists {
        None => {
            let id = try!(assign_new_id(conn));
            try!(label_id(id, &t, conn));
            Ok(Tree {
                id,
                applicand: None,
                argument: None,
            })
        }
        Some(id) => Ok(Tree {
            id,
            applicand: None,
            argument: None,
        }),
    }
}

impl Tree {
    pub fn call(&self, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
        match (self.applicand.clone(), self.argument.clone()) {
            (Some(mut app), Some(arg)) => match arg.id {
                REDUCTION => {
                    try!(app.reduce(conn));
                    match try!(app.as_token(conn)) {
                        Token::Expression(s) | Token::Atom(s) => Ok(Some(s)),
                    }
                }
                DEFINE => {
                    try!(app.expand(conn));
                    match try!(app.expand_as_token(conn)) {
                        Token::Expression(s) | Token::Atom(s) => Ok(Some(s)),
                    }
                }
                _ => app.call_as_applicand(&arg, conn),
            },
            _ => Ok(None),
        }
    }
    fn call_as_applicand(&self, arg: &Tree, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
        match (self.applicand.clone(), self.argument.clone()) {
            (Some(app2), Some(arg2)) => match arg2.id {
                REDUCTION => {
                    try!(insert_reduction3(app2.id, arg.id, conn));
                    Ok(None)
                }
                DEFINE => {
                    try!(Tree::transfer_id(arg.id, app2.id, conn));
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn reduce(&mut self, conn: &SqliteConnection) -> ZiaResult<bool> {
        //returns true if self is mutated by this function, else false
        let self_reduction = try!(find_normal_form(self.id, conn));
        match self_reduction {
            None => {
                let mut result = false;
                if let (Some(mut app), Some(mut arg)) =
                    (self.applicand.clone(), self.argument.clone())
                {
                    let app_result = try!(app.reduce(conn));
                    let arg_result = try!(arg.reduce(conn));
                    if app_result | arg_result {
                        *self = try!(Tree::new_definition(app, arg, conn));
                        try!(self.reduce(conn));
                        result = true;
                    }
                };
                Ok(result)
            }
            Some(n) => {
                self.id = n;
                self.applicand = None;
                self.argument = None;
                try!(self.expand(conn));
                Ok(true)
            }
        }
    }
    fn transfer_id(id_before: i32, id_after: i32, conn: &SqliteConnection) -> ZiaResult<()> {
        ///Need to delete label of arg if exists
        try!(unlabel(id_before, conn));
        let luid = try!(select_integer(LUID, conn));
        try!(refactor_id(id_before, id_after, luid, conn));
        Ok(())
    }

    fn expand_as_token(&mut self, conn: &SqliteConnection) -> ZiaResult<Token> {
        match (self.applicand.clone(), self.argument.clone()) {
            (Some(app), Some(arg)) => Tree::join_tokens(app, arg, conn),
            _ => self.as_token(conn),
        }
    }

    fn add_token(
        mut tree: Box<Tree>,
        conn: &SqliteConnection,
        mut string: String,
    ) -> ZiaResult<String> {
        match try!(tree.as_token(conn)) {
            Token::Atom(s) => {
                string.push_str(&s);
            }
            Token::Expression(s) => {
                string.push('(');
                string.push_str(&s);
                string.push(')');
            }
        }
        Ok(string)
    }

    fn as_token(&mut self, conn: &SqliteConnection) -> ZiaResult<Token> {
        match try!(label_from_id(self.id, conn)) {
            None => {
                try!(self.expand(conn));
                match (self.applicand.clone(), self.argument.clone()) {
                    (Some(app), Some(arg)) => Ok(try!(Tree::join_tokens(app, arg, conn))),
                    _ => Err(DBError::Absence(
                        "Unlabelled concept with no definition".to_string(),
                    )),
                }
            }
            Some(s) => Ok(Token::Atom(s)),
        }
    }

    fn expand(&mut self, conn: &SqliteConnection) -> ZiaResult<()> {
        match (self.applicand.clone(), self.argument.clone()) {
            (Some(mut app), Some(mut arg)) => {
                match try!(label_from_id(app.id, conn)) {
                    None => try!(app.expand(conn)),
                    Some(_) => (),
                };
                match try!(label_from_id(arg.id, conn)) {
                    None => arg.expand(conn),
                    Some(_) => Ok(()),
                }
            }
            _ => {
                let definition = try!(select_definition(self.id, conn));
                match definition {
                    None => Ok(()),
                    Some(def) => {
                        self.applicand = Tree::new_leaf(def.0);
                        self.argument = Tree::new_leaf(def.1);
                        Ok(())
                    }
                }
            }
        }
    }

    fn join_tokens(app: Box<Tree>, arg: Box<Tree>, conn: &SqliteConnection) -> ZiaResult<Token> {
        let mut string = String::new();
        string = try!(Tree::add_token(app, conn, string));
        string.push(' ');
        string = try!(Tree::add_token(arg, conn, string));
        Ok(Token::Expression(string))
    }

    fn new_leaf(id: i32) -> Option<Box<Tree>> {
        Some(Box::new(Tree {
            id,
            applicand: None,
            argument: None,
        }))
    }

    fn new_definition(
        applicand: Box<Tree>,
        argument: Box<Tree>,
        conn: &SqliteConnection,
    ) -> ZiaResult<Tree> {
        let id: i32;
        let app = applicand.id;
        let arg = argument.id;
        let application = try!(find_definition(app, arg, conn));
        match application {
            None => id = try!(insert_definition(app, arg, conn)),
            Some(def) => id = def,
        };
        Ok(Tree {
            id,
            applicand: Some(applicand),
            argument: Some(argument),
        })
    }
}
