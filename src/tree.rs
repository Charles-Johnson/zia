use zia2sql::{SqliteConnection, id_from_label, assign_new_id, insert_definition, REDUCTION, DEFINE, insert_reduction3, label_id, find_definition, refactor_id, select_integer, LUID, label_from_id, select_definition, find_normal_form, PRECEDENCE, unlabel, ZiaResult, DBError};
use super::token::{Token, parse_tokens, parse_line};
use super::precedence::set_precedence;

#[derive(Clone)]
pub struct Tree {
    id: i32,
    applicant: Option<Box<Tree>>,
    argument: Option<Box<Tree>>, 
}

pub fn extract_tree_from_token(token: &Token, conn: &SqliteConnection) -> ZiaResult<Tree> {
    match token {
        Token::Atom(t) => extract_tree_from_atom(t.to_string(), conn),
        Token::Expression(t) => extract_tree_from_expression(t.to_string(), conn)
    }
}

fn extract_tree_from_expression(t: String, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() 
        {0|1 => Err(DBError::Syntax("Expression needs to be composed of multiple tokens".to_string())),
           2 => {let parsed_tokens = parse_tokens(&tokens);
                 let applicant = try!(extract_tree_from_token(&parsed_tokens[0], conn));
                 let argument = try!(extract_tree_from_token(&parsed_tokens[1], conn));
                 Ok(try!(Tree::new_definition(Box::new(applicant), Box::new(argument), conn)))},
           _ => Err(DBError::Syntax("Expression composed of more than 2 tokens has not been implemented yet".to_string()))
    }
}

fn extract_tree_from_atom(t: String, conn: &SqliteConnection) -> ZiaResult<Tree> {
    let id_if_exists = try!(id_from_label(&t,conn));
    match id_if_exists {
        None => {let id = try!(assign_new_id(conn));
                 try!(label_id(id, &t,conn));
                 Ok(Tree{id, applicant: None, argument: None})},
        Some(id) => Ok(Tree{id, applicant: None, argument: None})
    }
}

impl Tree {
    pub fn call(&self, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
        match (self.applicant.clone(), self.argument.clone())
            {(Some(mut app),Some(arg)) =>
                  match arg.id
                      {REDUCTION => 
                           {try!(app.reduce(conn));
                            match try!(app.as_token(conn))
                                {Token::Expression(s)|Token::Atom(s) => Ok(Some(s))}
                            },
                          DEFINE =>
                           {try!(app.expand(conn));
                            match try!(app.expand_as_token(conn))
                                {Token::Expression(s)|Token::Atom(s) => Ok(Some(s))}
                            },
                               _ => 
                           app.call_as_applicant(&arg, conn)
                       },
                                     _ => Ok(None)
             }
    }
    fn call_as_applicant(&self, arg: &Tree, conn: &SqliteConnection) -> ZiaResult<Option<String>> {
        match (self.applicant.clone(), self.argument.clone())
            {(Some(app2),Some(arg2)) => 
                 match app2.id
                     { REDUCTION =>
                          {try!(insert_reduction3(arg.id, arg2.id, conn));
                           Ok(None)},
                          DEFINE => 
                          {try!(Tree::transfer_id(arg2.id, arg.id, conn));
                           Ok(None)},
                      PRECEDENCE =>
                          {try!(set_precedence(arg.id, arg2.id, conn));
                           Ok(None)},
                               _ => Ok(None)
                      },
                                   _ => Ok(None)
             }
    }
    
    fn reduce(&mut self, conn: &SqliteConnection) -> ZiaResult<bool> {
        //returns true if self is mutated by this function, else false
        let self_reduction = try!(find_normal_form(self.id, conn));
        match self_reduction 
            {None => {let mut result = false;
                      match (self.applicant.clone(), self.argument.clone()) 
                          {(Some(mut app),Some(mut arg)) => 
                               {let app_result = try!(app.reduce(conn));
                                let arg_result = try!(arg.reduce(conn));
                                if app_result | arg_result 
                                    {*self = try!(Tree::new_definition(app, arg, conn));
                                     try!(self.reduce(conn));
                                     result = true;}   
                                },                       
                                                       _ => ()
                           };
                      Ok(result)
                      },
             Some(n) => {self.id = n;
                         self.applicant = None;
                         self.argument = None;
                         try!(self.expand(conn));
                         Ok(true)}
             } 
    }
    fn transfer_id(id_before: i32, id_after: i32, conn: &SqliteConnection) -> ZiaResult<()>{
        ///Need to delete label of arg if exists
        try!(unlabel(id_before,conn));
        let luid = try!(select_integer(LUID, conn));
        try!(refactor_id(id_before, id_after, luid, conn));
        Ok(())
    }

    fn expand_as_token(&mut self, conn: &SqliteConnection) -> ZiaResult<Token> {
        match (self.applicant.clone(), self.argument.clone()) 
            {(Some(app), Some(arg)) => Tree::join_tokens(app, arg, conn),
                                  _ => self.as_token(conn)}
    }

    fn add_token(mut tree: Box<Tree>, conn: &SqliteConnection, mut string: String) -> ZiaResult<String> {
        match try!(tree.as_token(conn)) 
            {      Token::Atom(s) => {string.push_str(&s);},
             Token::Expression(s) => {string.push('(');
                                      string.push_str(&s);
                                      string.push(')');}
             }
        Ok(string)
    }

    fn as_token(&mut self, conn: &SqliteConnection) -> ZiaResult<Token> {
        match try!(label_from_id(self.id, conn)) 
            {   None => {try!(self.expand(conn));
                         match (self.applicant.clone(), self.argument.clone())
                             {(Some(app),Some(arg)) => Ok(try!(Tree::join_tokens(app, arg, conn))),
                                                  _ => Err(DBError::Absence("Unlabelled concept with no definition".to_string()))
                              }
                         },    
             Some(s) => Ok(Token::Atom(s))
             }
    }

    fn expand(&mut self, conn: &SqliteConnection) -> ZiaResult<()> {
        match (self.applicant.clone(), self.argument.clone())  
            {(Some(mut app),Some(mut arg)) => 
                {match try!(label_from_id(app.id, conn)) 
                     {       None => try!(app.expand(conn)),
                          Some(_) => ()};
                 match try!(label_from_id(arg.id, conn))
                     {       None => arg.expand(conn),
                          Some(_) => Ok(())}
                 },
                                         _ => 
                {let definition = try!(select_definition(self.id, conn));
                 match definition 
                     {     None => Ok(()),
                      Some(def) => {self.applicant = Tree::new_leaf(def.0);
                                    self.argument = Tree::new_leaf(def.1);
                                    Ok(())}
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

    fn new_leaf(id: i32) -> Option<Box<Tree>>{
        Some(Box::new(Tree{id,applicant:None,argument:None}))
    }

    fn new_definition(applicant: Box<Tree>, argument: Box<Tree>, conn: &SqliteConnection) -> ZiaResult<Tree> {
        let id: i32;
        let app = applicant.id;
        let arg = argument.id;
        let application = try!(find_definition(app, arg, conn));
        match application {None => id = try!(insert_definition(app, arg, conn)),
                           Some(def) => id = def
                           };
        Ok(Tree{id, applicant: Some(applicant), argument: Some(argument)})
    }
}

