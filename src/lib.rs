extern crate zia2sql;

pub use zia2sql::{memory_database, SqliteConnection, id_from_label, assign_new_id, assign_new_variable_id, insert_definition, REDUCTION, DEFINE, find_application, insert_reduction2, label_of_reduction_of_id, label_id, find_definitions, refactor_id, select_integer, LUID};

pub fn oracle(buffer: &str, conn: &SqliteConnection)-> String{
    let application_tree = extract_tree_from_token(&Token::Expression(buffer.to_string()), conn);
    application_tree.call_reduction_rule(conn);
    application_tree.call_definition(conn);
    match application_tree.call_normal_form(conn) {None => "".to_string(),
                                                   Some(s) => s}
    
}

fn parse_line(buffer: &str)->Vec<String>{
    let mut tokens: Vec<String> = [].to_vec();
    let mut token = String::new();
    let mut parenthesis_level = 0;
    for letter in buffer.chars() {
        parse_letter(letter, &mut parenthesis_level, &mut token, &mut tokens);
    }
    if token != "" {tokens.push(token.clone());}
    tokens
}

fn push_token(letter: char, parenthesis_level: &i8, token: &mut String,tokens: &mut Vec<String>) {
    if (token != "")&(*parenthesis_level==0) {
        tokens.push(token.clone());
        *token = String::new();
        }
    if *parenthesis_level !=0 {token.push(letter);}
}


fn parse_letter(letter: char, parenthesis_level: &mut i8, token: &mut String, tokens: &mut Vec<String>) {
    match letter {
        '(' => {push_token(letter,parenthesis_level,token,tokens); *parenthesis_level += 1;},
        ')' => {*parenthesis_level -= 1; push_token(letter,parenthesis_level,token,tokens);},
        ' ' => push_token(letter,parenthesis_level,token,tokens),
        '\n'|'\r' => (),
        _ => token.push(letter),
    };
}

fn parse_tokens(tokens: &Vec<String>) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = [].to_vec();
    for token in tokens {
        if token.contains(" ") {new_tokens.push(Token::Expression(token[..].to_string()));}
        else if token.starts_with("_") {new_tokens.push(Token::Free(token[1..].to_string()));}
        else if token.ends_with("_") {new_tokens.push(Token::Dummy(token[..token.len()-2].to_string()));}
        else {new_tokens.push(Token::Atom(token[..].to_string()));}
    }
    new_tokens
}




fn extract_tree_from_atom(t: String, conn: &SqliteConnection) -> ApplicationTree {
    let id_if_exists = id_from_label(&t,conn);
    match id_if_exists {
        None => {let id = assign_new_id(conn);
                 label_id(id, &t,conn);
                 ApplicationTree{id: Some(id), applicant: None, argument: None}},
        Some(id) => ApplicationTree{id: Some(id), applicant: None, argument: None}
    }
}


fn extract_tree_from_expression(t: String, conn: &SqliteConnection) -> ApplicationTree {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() {0|1 => panic!("Expression needs to be composed of multiple tokens"),
                        2 => {let parsed_tokens = parse_tokens(&tokens);
                              let applicant = extract_tree_from_token(&parsed_tokens[0], conn);
                              let argument = extract_tree_from_token(&parsed_tokens[1], conn);
                              let mut id: Option<i32>;
                              match (applicant.id, argument.id) {(Some(app), Some(arg)) => {let definitions = find_definitions(app, arg, conn);
                                       match definitions.len() {0 => id = Some(insert_definition(app, arg, conn)),
                                                                1 => id = Some(definitions[0]),
                                                                _ => panic!("There are multiple ids for the application of the same applicant and argument pair."),}}, 
                                                              _ => id = None};
                              ApplicationTree{id, applicant: Some(Box::new(applicant)), argument: Some(Box::new(argument))}},
                        _ => panic!("Expression composed of more than 2 tokens has not been implemented yet")
    }
}


fn extract_tree_from_free(_t: String, conn: &SqliteConnection) -> ApplicationTree {
    ApplicationTree{id: Some(assign_new_variable_id(conn)), applicant: None, argument: None}
}



fn extract_tree_from_dummy(_t: String, conn: &SqliteConnection) -> ApplicationTree {
    ApplicationTree{id: Some(assign_new_variable_id(conn)), applicant: None, argument: None}
}


fn extract_tree_from_token(token: &Token, conn: &SqliteConnection) -> ApplicationTree {
    match token {
        Token::Atom(t) => extract_tree_from_atom(t.to_string(), conn),
        Token::Expression(t) => extract_tree_from_expression(t.to_string(), conn),
        Token::Free(t) => extract_tree_from_free(t.to_string(), conn),
        Token::Dummy(t) => extract_tree_from_dummy(t.to_string(), conn)
    }
}

#[derive(Debug,PartialEq,Clone)]
enum Token {
    Atom(String),
    Expression(String),
    Free(String),
    Dummy(String),
}

#[derive(Clone)]
struct ApplicationTree {
    id: Option<i32>,
    applicant: Option<Box<ApplicationTree>>,
    argument: Option<Box<ApplicationTree>>, 
}


impl ApplicationTree {
    fn call_reduction_rule(&self, conn: &SqliteConnection) {
        match (self.applicant.clone(), self.argument.clone()) {(Some(app),Some(arg)) => match (app.applicant.clone(), app.argument.clone()) {(Some(app2),Some(arg2)) => if app2.id == Some(REDUCTION) {println!("the application is REDUCTION"); ApplicationTree::insert_reduction(&arg, &arg2, conn)}, _ => ()}, _ => ()};
    }
    fn insert_reduction(tree: &ApplicationTree, normal_form: &ApplicationTree, conn: &SqliteConnection) {
        match (tree.id, normal_form.id) {(Some(t),Some(n)) => insert_reduction2(t,n,conn), _ => ()};
    }
    fn call_normal_form(&self, conn: &SqliteConnection) -> Option<String> {
        match (self.applicant.clone(), self.argument.clone()) {(Some(app),Some(arg)) => {println!("applicant and argument both exist");
                     println!("{:?}",(app.id,arg.id)); if arg.id == Some(REDUCTION) {println!("the argument is REDUCTION"); ApplicationTree::find_normal_form(&app, conn)} else {None}}, _ => None}
    }
    fn find_normal_form(tree: &ApplicationTree, conn: &SqliteConnection) -> Option<String>{
        match tree.id.clone() {None => None,
                               Some(id) => label_of_reduction_of_id(id,conn)}
    }
    fn call_definition(&self, conn: &SqliteConnection) {
        match (self.applicant.clone(), self.argument.clone()) 
            {(Some(app),Some(arg)) => match (app.applicant.clone(), app.argument.clone())
                                          {(Some(app2), Some(arg2)) => if app2.id == Some(DEFINE)
                                              {ApplicationTree::label_application(&arg, &arg2, conn)},
                                                                  _ => ()}, 
                                 _ => ()};
    }
    fn label_application(arg: &ApplicationTree, arg2: &ApplicationTree, conn: &SqliteConnection) {
        match (arg.id, arg2.id) {(Some(id),Some(id2)) => {let luid = select_integer(LUID, conn);
                                                          refactor_id(id, id2, luid, conn)},
                                 _ => panic!("id missing for one of the concepts in definition statement")}
        println!("Not yet implemented");
    }
}


#[cfg(test)]
mod reductions {
    use {oracle, memory_database};
    #[test]
    fn monad() {
        let conn = memory_database();
        assert_eq!(oracle("(-> b)a", &conn),"");
        assert_eq!(oracle("a ->", &conn),"b");
        assert_eq!(oracle("(-> false)(not true)", &conn), "");
        assert_eq!(oracle("(not true)->", &conn),"false");
    }
    #[test]
    fn diad() {
        let conn = memory_database();
        assert_eq!(oracle("(0 + 1)->", &conn), "1");
    }
    #[test]
    fn lambda() {
        let conn = memory_database();
        assert_eq!(oracle("((lambda x_)(_f _x))_y ->", &conn),"_f _y");
    }
    #[test]
    fn wrong_variable() {
        let conn = memory_database();
        assert_eq!(oracle("_x -> _y", &conn), "Error! Variable _y does not appear in the expression '_x'.");
    }
    #[test]
    fn labelling_a_variable() {
        let conn = memory_database();
        assert_eq!(oracle("a := _x", &conn),"Error! Cannot label variable expression '_x'.");
        assert_eq!(oracle("a := x_", &conn),"Error! Cannot label dummy expression 'x_'.");
    }
    #[test]
    fn variable_label() {
        let conn = memory_database();
        assert_eq!(oracle("_x := a", &conn), "Error! Cannot use '_x' as a label.");
        assert_eq!(oracle("x_ := a", &conn), "Error! Cannot use 'x_' as a label.");
    }
    #[test]
    fn variable_reduction() {
        let conn = memory_database();
        assert_eq!(oracle("_x and false ->", &conn), "false");
    }
}

mod definitions {
    use oracle;
    use memory_database;
    #[test]
    fn monad() {
        let conn = memory_database();
        assert_eq!(oracle("(:= (repeated +))*", &conn), "");
        assert_eq!(oracle("* :=", &conn), "repeated +");
    }
}

mod tokens {
    use parse_line;
    use parse_tokens;
    use Token::Atom;
    use Token::Expression;
    #[test]
    fn monad() {
        let parsed_line = parse_line("(not true)->");
        assert_eq!(parsed_line,["not true", "->"].to_vec());
        assert_eq!(parse_tokens(&parsed_line),[Expression("not true".to_string()),Atom("->".to_string())].to_vec());
    }
    #[test]
    fn diad() {
        assert_eq!(parse_line("(0 + 1)->"), ["0 + 1", "->"].to_vec());
    }
    #[test]
    fn lambda() {
        assert_eq!(parse_line("((lambda x_)(_f _x))_y ->"),["(lambda x_)(_f _x)", "_y", "->"].to_vec());
    }
}
