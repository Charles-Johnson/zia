/*  Library for the Zia programming language.
    Copyright (C) 2018  Charles Johnson

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
use std::ops::Add;

#[cfg(test)]
mod tokens {
    use super::parse_line;
    use super::parse_tokens;
    use super::Token;
    #[test]
    fn monad() {
        let parsed_line = parse_line("(not true)->");
        assert_eq!(parsed_line, ["not true", "->"].to_vec());
        assert_eq!(
            parse_tokens(&parsed_line),
            [
                Token::Expression("not true".to_string()),
                Token::Atom("->".to_string())
            ]
                .to_vec()
        );
    }
    #[test]
    fn diad() {
        assert_eq!(parse_line("(0 + 1)->"), ["0 + 1", "->"].to_vec());
    }
    #[test]
    fn lambda() {
        assert_eq!(
            parse_line("((lambda x_)(_f _x))_y ->"),
            ["(lambda x_)(_f _x)", "_y", "->"].to_vec()
        );
    }
}

pub fn parse_tokens(tokens: &[String]) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = [].to_vec();
    for token in tokens.iter() {
        if token.contains(' ') {
            new_tokens.push(Token::Expression(token[..].to_string()));
        } else {
            new_tokens.push(Token::Atom(token[..].to_string()));
        }
    }
    new_tokens
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Atom(String),
    Expression(String),
}

impl Token {
    pub fn as_string(&self) -> String {
        match *self {
            Token::Expression(ref s) => s.clone(),
            Token::Atom(ref s) => s.clone(),
        }
    }
}

impl Add<Token> for Token {
    type Output = Token;
    fn add(self, other: Token) -> Token {
        let app_string: String;
        match self {
            Token::Expression(s) => app_string = "(".to_string() + &s + ")",
            Token::Atom(s) => app_string = s,
        };
        let arg_string: String;
        match other {
            Token::Expression(s) => arg_string = "(".to_string() + &s + ")",
            Token::Atom(s) => arg_string = s,
        };
        Token::Expression(app_string + " " + &arg_string)
    }
}

pub fn parse_line(buffer: &str) -> Vec<String> {
    let mut tokens: Vec<String> = [].to_vec();
    let mut token = String::new();
    let mut parenthesis_level = 0;
    for letter in buffer.chars() {
        parse_letter(letter, &mut parenthesis_level, &mut token, &mut tokens);
    }
    if token != "" {
        tokens.push(token.clone());
    }
    tokens
}

fn parse_letter(
    letter: char,
    parenthesis_level: &mut i8,
    token: &mut String,
    tokens: &mut Vec<String>,
) {
    match letter {
        '(' => {
            push_token(letter, *parenthesis_level, token, tokens);
            *parenthesis_level += 1;
        }
        ')' => {
            *parenthesis_level -= 1;
            push_token(letter, *parenthesis_level, token, tokens);
        }
        ' ' => push_token(letter, *parenthesis_level, token, tokens),
        '\n' | '\r' => (),
        _ => token.push(letter),
    };
}

fn push_token(letter: char, parenthesis_level: i8, token: &mut String, tokens: &mut Vec<String>) {
    if (token != "") & (parenthesis_level == 0) {
        tokens.push(token.clone());
        *token = String::new();
    }
    if parenthesis_level != 0 {
        token.push(letter);
    }
}
