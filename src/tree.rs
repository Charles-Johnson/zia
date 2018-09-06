/*  Library for the Zia programming language.
    Copyright (C) 2018 Charles Johnson

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
use db::{
    assign_new_id, find_definition, find_normal_form, id_from_label, insert_definition,
    insert_reduction3, label_from_id, label_id, select_definition, transfer_id, DBError,
    SqliteConnection, ZiaResult, DEFINE, REDUCTION,
};
use token::{parse_line, parse_tokens, Token};

///Data structure representing the (full binary) abstract syntax tree used to interpret expressions.
#[derive(Clone)]
pub struct Tree {
    id: i32,
    applicand: Option<Box<Tree>>,
    argument: Option<Box<Tree>>,
}

///A function to generate the syntax tree from an expression
pub fn extract_tree_from_expression(t: &str, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() {
        0 => Err(DBError::Syntax(
            "Parentheses need to contain an expression".to_string(),
        )),
        1 => extract_tree_from_atom(&tokens[0], conn),
        2 => {
            let parsed_tokens = parse_tokens(&tokens);
            extract_tree_from_monad(&parsed_tokens[0], &parsed_tokens[1], conn)
        }
        _ => Err(DBError::Syntax(
            "Expression composed of more than 2 tokens has not been implemented yet".to_string(),
        )),
    }
}

fn extract_tree_from_atom(t: &str, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let id_if_exists = try!(id_from_label(t, conn));
    match id_if_exists {
        None => {
            let id = try!(assign_new_id(conn));
            try!(label_id(id, t, conn));
            Tree::new_leaf(id)
        }
        Some(id) => Tree::new_leaf(id),
    }
}

fn extract_tree_from_monad(app: &Token, arg: &Token, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let applicand = try!(extract_tree_from_token(app, conn));
    let argument = try!(extract_tree_from_token(arg, conn));
    Ok(try!(Tree::new_definition(
        Box::new(applicand),
        Box::new(argument),
        conn
    )))
}

fn extract_tree_from_token(t: &Token, conn: &SqliteConnection) -> ZiaResult<Tree> {
    match t {
        Token::Atom(s) => extract_tree_from_atom(&s, conn),
        Token::Expression(s) => extract_tree_from_expression(&s, conn),
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
                    try!(transfer_id(arg.id, app2.id, conn));
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    ///assuming no errors, returns Ok(true) if self is mutated by this function, else Ok(false)
    fn reduce(&mut self, conn: &SqliteConnection) -> ZiaResult<bool> {
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
                *self = try!(Tree::new_leaf(n));
                try!(self.expand(conn));
                Ok(true)
            }
        }
    }

    fn expand_as_token(&mut self, conn: &SqliteConnection) -> ZiaResult<Token> {
        match (self.applicand.clone(), self.argument.clone()) {
            (Some(app), Some(arg)) => Tree::join_tokens(*app, *arg, conn),
            _ => self.as_token(conn),
        }
    }

    fn add_token(mut self, conn: &SqliteConnection, mut string: String) -> ZiaResult<String> {
        match try!(self.as_token(conn)) {
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
                    (Some(app), Some(arg)) => Ok(try!(app.join_tokens(*arg, conn))),
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
                        self.applicand = Some(Box::new(try!(Tree::new_leaf(def.0))));
                        self.argument = Some(Box::new(try!(Tree::new_leaf(def.1))));
                        Ok(())
                    }
                }
            }
        }
    }

    fn join_tokens(self, arg: Tree, conn: &SqliteConnection) -> ZiaResult<Token> {
        let mut string = String::new();
        string = try!(Tree::add_token(self, conn, string));
        string.push(' ');
        string = try!(Tree::add_token(arg, conn, string));
        Ok(Token::Expression(string))
    }

    fn new_leaf(id: i32) -> ZiaResult<Tree> {
        Ok(Tree {
            id,
            applicand: None,
            argument: None,
        })
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
