use zia2sql::{SqliteConnection, id_from_label, assign_new_id, assign_new_variable_id, insert_definition, REDUCTION, DEFINE, insert_reduction3, label_id, find_definition, refactor_id, select_integer, LUID, label_from_id, select_definition, find_normal_form};
use super::token::{Token, parse_tokens, parse_line};

#[derive(Clone)]
pub struct Tree {
    id: i32,
    applicant: Option<Box<Tree>>,
    argument: Option<Box<Tree>>, 
}

fn extract_tree_from_atom(t: String, conn: &SqliteConnection) -> Tree {
    let id_if_exists = id_from_label(&t,conn);
    match id_if_exists {
        None => {let id = assign_new_id(conn);
                 label_id(id, &t,conn);
                 Tree{id, applicant: None, argument: None}},
        Some(id) => Tree{id, applicant: None, argument: None}
    }
}


fn extract_tree_from_expression(t: String, conn: &SqliteConnection) -> Tree {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() {0|1 => panic!("Expression needs to be composed of multiple tokens"),
                        2 => {let parsed_tokens = parse_tokens(&tokens);
                              let applicant = extract_tree_from_token(&parsed_tokens[0], conn);
                              let argument = extract_tree_from_token(&parsed_tokens[1], conn);
                              Tree::new_definition(Box::new(applicant), Box::new(argument), conn)},
                        _ => panic!("Expression composed of more than 2 tokens has not been implemented yet")
    }
}


fn extract_tree_from_free(_t: String, conn: &SqliteConnection) -> Tree {
    Tree{id: assign_new_variable_id(conn), applicant: None, argument: None}
}



fn extract_tree_from_dummy(_t: String, conn: &SqliteConnection) -> Tree {
    Tree{id: assign_new_variable_id(conn), applicant: None, argument: None}
}


pub fn extract_tree_from_token(token: &Token, conn: &SqliteConnection) -> Tree {
    match token {
        Token::Atom(t) => extract_tree_from_atom(t.to_string(), conn),
        Token::Expression(t) => extract_tree_from_expression(t.to_string(), conn),
        Token::Free(t) => extract_tree_from_free(t.to_string(), conn),
        Token::Dummy(t) => extract_tree_from_dummy(t.to_string(), conn)
    }
}

impl Tree {
    pub fn call_reduction_rule(&self, conn: &SqliteConnection) {
        match (self.applicant.clone(), self.argument.clone())
            {(Some(app),Some(arg)) => 
                match (app.applicant.clone(), app.argument.clone())
                    {(Some(app2),Some(arg2)) => 
                        if app2.id == REDUCTION
                            {insert_reduction3(arg.id, arg2.id, conn)},
                                           _ => ()},
                                 _ => ()};
    }
    pub fn call_normal_form(&mut self, conn: &SqliteConnection) -> Option<String> {
        match (self.applicant.clone(), self.argument.clone())
            {(Some(mut app),Some(arg)) => 
                {if arg.id == REDUCTION 
                     {app.reduce(conn);
                      match app.as_token(conn) 
                          {Token::Expression(s)|Token::Atom(s) => Some(s),
                                                             _ => None}
                      }
                 else {None}
                 },
                                     _ => None
             }
    }
    fn reduce(&mut self, conn: &SqliteConnection) -> bool {
        //returns true if self is mutated by this function, else false
        let self_reduction = find_normal_form(self.id, conn);
        match self_reduction 
            {None => {let mut result = false;
                      match (self.applicant.clone(), self.argument.clone()) 
                          {(Some(mut app),Some(mut arg)) => 
                               {let app_result = app.reduce(conn);
                                let arg_result = arg.reduce(conn);
                                if app_result | arg_result 
                                    {*self = Tree::new_definition(app, arg, conn);
                                     self.reduce(conn);
                                     result = true;}   
                                },                       
                                                       _ => ()
                           };
                      result
                      },
             Some(n) => {self.id = n;
                         self.applicant = None;
                         self.argument = None;
                         self.expand(conn);
                         true}
             } 
    }
    pub fn call_definition(&self, conn: &SqliteConnection) {
        match (self.applicant.clone(), self.argument.clone()) 
            {(Some(app),Some(arg)) => match (app.applicant.clone(), app.argument.clone())
                                          {(Some(app2), Some(arg2)) => if app2.id == DEFINE
                                              {Tree::label_application(&arg, &arg2, conn)},
                                                                  _ => ()}, 
                                 _ => ()};
    }
    fn label_application(arg: &Tree, arg2: &Tree, conn: &SqliteConnection) {
        let luid = select_integer(LUID, conn);
        refactor_id(arg.id, arg2.id, luid, conn);
    }
    /// A method that checks whether the tree is an expansion statement e.g. (x :=).
    /// If the tree is an expansion statement then the applicant is expanded one 
    /// level and the method returns its token string.  
    pub fn call_expansion(&mut self, conn: &SqliteConnection) -> Option<String> {
        match (self.applicant.clone(), self.argument.clone()) 
            {(Some(mut app), Some(arg)) => if arg.id == DEFINE 
                                           {app.expand(conn);
                                            match app.expand_as_token(conn)
                                                {Token::Expression(s)|
                                                       Token::Atom(s) => Some(s),
                                                                    _ => None}
                                            } else {None},
                                      _ => None
             }
    }
    fn expand(&mut self, conn: &SqliteConnection) {
        match (self.applicant.clone(), self.argument.clone())  
            {(Some(mut app),Some(mut arg)) => 
                {match label_from_id(app.id, conn) 
                     {       None => app.expand(conn),
                          Some(_) => ()};
                 match label_from_id(arg.id, conn)
                     {       None => {arg.expand(conn)},
                          Some(_) => ()};
                 },
                                         _ => 
                {let definition = select_definition(self.id, conn);
                 match definition {     None => (),
                                   Some(def) => {self.applicant = Tree::new_leaf(def.0);
                                                 self.argument = Tree::new_leaf(def.1);
                                                 }
                                   };
                 }  
             };
    }
    fn as_token(&mut self, conn: &SqliteConnection) -> Token {
        match label_from_id(self.id, conn) 
            {   None => {self.expand(conn);
                         match (self.applicant.clone(), self.argument.clone())
                             {(Some(app),Some(arg)) => Tree::join_tokens(app, arg, conn),
                                                  _ => panic!("Unlabelled concept with no definition")
                              }
                         },    
             Some(s) => Token::Atom(s)
             }
    }
    fn expand_as_token(&mut self, conn: &SqliteConnection) -> Token {
        match (self.applicant.clone(), self.argument.clone()) 
            {(Some(app), Some(arg)) => Tree::join_tokens(app, arg, conn),
                                  _ => self.as_token(conn)}
    }
    fn join_tokens(app: Box<Tree>, arg: Box<Tree>, conn: &SqliteConnection) -> Token {
        let mut string = String::new();
        string = Tree::add_token(app, conn, string);
        string.push(' ');
        string = Tree::add_token(arg, conn, string);
        Token::Expression(string)
    }
    fn add_token(mut tree: Box<Tree>, conn: &SqliteConnection, mut string: String) -> String {
        match tree.as_token(conn) 
            {      Token::Atom(s) => {string.push_str(&s);},
             Token::Expression(s) => {string.push('(');
                                      string.push_str(&s);
                                      string.push(')');},
                   Token::Free(s) => {string.push('_');
                                      string.push_str(&s);},
                  Token::Dummy(s) => {string.push_str(&s);
                                      string.push('_');}
             }
        string
    }
    fn new_leaf(id: i32) -> Option<Box<Tree>>{
        Some(Box::new(Tree{id,applicant:None,argument:None}))
    }
    fn new_definition(applicant: Box<Tree>, argument: Box<Tree>, conn: &SqliteConnection) -> Tree{
        let id: i32;
        let app = applicant.id;
        let arg = argument.id;
        let application = find_definition(app, arg, conn);
        match application {None => id = insert_definition(app, arg, conn),
                           Some(def) => id = def
                           };
        Tree{id, applicant: Some(applicant), argument: Some(argument)}
    }
}

