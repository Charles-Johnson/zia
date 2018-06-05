extern crate zia2sql;

pub use zia2sql::{memory_database, SqliteConnection};

pub fn oracle<'a>(buffer: &'a str, conn: &'a SqliteConnection)->&'a str{
    let tokens = parse_line(buffer);
    buffer
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

#[cfg(test)]
mod reductions {
    use {oracle, memory_database};
    #[test]
    fn monad() {
        let conn = memory_database();
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
    #[test]
    fn monad() {
        assert_eq!(parse_line("(not true)->"),["not true", "->"].to_vec());
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
