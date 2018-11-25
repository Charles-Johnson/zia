use constants::LABEL;
use utils::{ZiaError, ZiaResult};
use std::ops::Add;
use token::{parse_tokens, Token, parse_line};
use traits::{GetDefinition, Id, SyntaxFactory};

pub trait SyntaxConverter<T, U>
where
    Self: SyntaxFinder<T>,
    T: Clone + Id + GetDefinition<T> + Label<T>,
    U: SyntaxFactory<T> + Add<U, Output = ZiaResult<U>>,
{
    fn ast_from_expression(&mut self, s: &str) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::Syntax(
                "Parentheses need to contain an expression".to_string(),
            )),
            1 => self.ast_from_atom(&tokens[0]),
            2 => {
                let parsed_tokens = parse_tokens(&tokens);
                self.ast_from_pair(&parsed_tokens[0], &parsed_tokens[1])
            }
            _ => Err(ZiaError::Syntax(
                "Expression composed of more than 2 tokens has not been implemented yet"
                    .to_string(),
            )),
        }
    }
    fn ast_from_atom(&mut self, s: &str) -> ZiaResult<U> {
        let concept_if_exists = try!(self.concept_from_label(s));
        Ok(U::new(s, concept_if_exists))
    }
    fn ast_from_pair(&mut self, left: &Token, right: &Token) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token(left));
        let righthand = try!(self.ast_from_token(right));
        lefthand + righthand
    }
    fn ast_from_token(&mut self, t: &Token) -> ZiaResult<U> {
        match *t {
            Token::Atom(ref s) => self.ast_from_atom(s),
            Token::Expression(ref s) => self.ast_from_expression(s),
        }
    }
}

pub trait SyntaxFinder<T>
where
    T: Label<T> 
		+ GetDefinition<T> 
		+ Clone 
		+ Id,
{
    fn get_string_concept(&self, &str) -> Option<T>;
    fn concept_from_label(&self, s: &str) -> ZiaResult<Option<T>> {
        match self.get_string_concept(s) {
            None => Ok(None),
            Some(c) => c.get_labellee(),
        }
    }
}

pub trait Label<T>
where
    T: GetDefinition<T> + GetNormalFormOf<T> + Clone + Id,
    Self: GetNormalFormOf<T>,
{
    fn get_labellee(&self) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for label in self.get_normal_form_of() {
            match label.get_definition() {
                None => continue,
                Some((r, x)) => if r.get_id() == LABEL {
                    candidates.push(x)
                } else {
                    continue;
                },
            };
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple concepts are labelled with the same string".to_string(),
            )),
        }
    }
}

pub trait GetNormalFormOf<T> {
    fn get_normal_form_of(&self) -> Vec<T>;
}
