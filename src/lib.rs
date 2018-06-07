extern crate zia2sql;

pub use zia2sql::{memory_database, SqliteConnection, id_from_label, assign_new_id, assign_new_variable_id, insert_definition, REDUCTION, find_application, insert_reduction2, label_of_reduction_of_id};

pub fn oracle(buffer: &str, conn: &SqliteConnection)-> String{
    let expression_id = extract_id_from_token(&Token::Expression(buffer.to_string()), conn);
    let application_if_found = find_application(expression_id, conn);
    let mut string = String::new();
    scan_application_further(expression_id, &scan_application, &insert_reduction3, conn, &mut string);
    scan_application(expression_id, 0, &find_normal_form2, conn, &mut string);
    string
}

fn scan_application_further(id: i32, f: &Fn(i32,i32,&Fn(i32,i32,i32,&SqliteConnection,&mut String),&SqliteConnection,&mut String), g: &Fn(i32,i32,i32,&SqliteConnection,&mut String), conn: &SqliteConnection, string: &mut String) {
    let application_if_found = find_application(id, conn);
    match application_if_found {None => (),
                                Some((appl1,arg1)) => f(appl1,arg1,g,conn,string)};
}

fn scan_application(appl1:i32,arg1:i32,g:&Fn(i32,i32,i32,&SqliteConnection,&mut String),conn: &SqliteConnection,string: &mut String) {
    let application_if_found = find_application(appl1, conn);
    match application_if_found {None => (),
                                Some((appl2,arg2)) => g(appl2, arg1, arg2, conn, string)};
}

fn insert_reduction3(appl2:i32,arg1:i32,arg2:i32, conn: &SqliteConnection, string: &mut String) {
    if appl2 == REDUCTION {insert_reduction2(arg1,arg2,conn);}
}

fn find_normal_form2(appl2:i32,arg1:i32,arg2:i32, conn: &SqliteConnection, string: &mut String) {
    if arg2 == REDUCTION {let label = label_of_reduction_of_id(appl2,conn);
                          match label {None => (),
                                       Some(s) => *string = s};}
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
        _ => token.push(letter)
    }
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

fn extract_id_from_atom(t: String, conn: &SqliteConnection) -> i32 {
    let id_if_exists = id_from_label(&t,conn);
    match id_if_exists {
        None => assign_new_id(conn),
        Some(id) => id
    }
}

fn extract_id_from_expression(t: String, conn: &SqliteConnection) -> i32 {
    let tokens: Vec<String> = parse_line(&t);
    match tokens.len() {0|1 => panic!("Expression needs to be composed of multiple tokens"),
                        2 => {let parsed_tokens = parse_tokens(&tokens);
                              let applicant = extract_id_from_token(&parsed_tokens[0], conn);
                              let argument = extract_id_from_token(&parsed_tokens[1], conn);
                              insert_definition(applicant, argument, conn)},
                        _ => panic!("Expression composed of more than 2 tokens has not been implemented yet")
    }
}

fn extract_id_from_free(t: String, conn: &SqliteConnection) -> i32 {
    assign_new_variable_id(conn)
}

fn extract_id_from_dummy(t: String, conn: &SqliteConnection) -> i32 {
    assign_new_variable_id(conn)
}

fn extract_id_from_token(token: &Token, conn: &SqliteConnection) -> i32 {
    match token {
        Token::Atom(t) => extract_id_from_atom(t.to_string(), conn),
        Token::Expression(t) => extract_id_from_expression(t.to_string(), conn),
        Token::Free(t) => extract_id_from_free(t.to_string(), conn),
        Token::Dummy(t) => extract_id_from_dummy(t.to_string(), conn)
    }
}

#[derive(Debug,PartialEq,Clone)]
enum Token {
    Atom(String),
    Expression(String),
    Free(String),
    Dummy(String),
}


#[cfg(test)]
mod reductions {
    use {oracle, memory_database};
    #[test]
    fn monad() {
        let conn = memory_database();
        assert_eq!(oracle("(-> b)a", &conn),"");
        assert_eq!(oracle("a ->", &conn),"b");
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
