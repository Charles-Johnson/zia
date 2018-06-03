pub fn oracle(buffer: &str)->&str{
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
    if (*parenthesis_level !=0) {token.push(letter);}
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
    use oracle;
    #[test]
    fn monad() {
        assert_eq!(oracle("(not true)->"),"false");
    }
    #[test]
    fn diad() {
        assert_eq!(oracle("(0 + 1)->"), "1");
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
}
