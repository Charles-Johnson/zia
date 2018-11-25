use std::fmt;
use token::Token;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, HasToken, LabelGetter,
    MaybeConcept, MightExpand,
};
use utils::{ZiaError, ZiaResult};

pub trait Expander<T, U>
where
    T: GetNormalForm<T>
        + FindDefinition<T>
        + Clone
        + PartialEq
        + fmt::Display
        + GetDefinition<T>,
    U: MaybeConcept<T> + HasToken + MightExpand,
    Self: TokenHandler<T>,
{
    fn expand_ast_token(&self, ast: &U) -> ZiaResult<Token> {
        if let Some(con) = ast.get_concept() {
            self.expand_as_token(&con)
        } else if let Some((ref app2, ref arg2)) = ast.get_expansion() {
            Ok(try!(self.expand_ast_token(app2)) + try!(self.expand_ast_token(arg2)))
        } else {
            Ok(ast.get_token())
        }
    }
}

pub trait TokenHandler<T>
where
    T:  GetNormalForm<T>
        + FindDefinition<T>
        + Clone
        + PartialEq
        + fmt::Display
        + GetDefinition<T>,
    Self: LabelGetter<T>,
{
    fn get_token(&self, c: &T) -> ZiaResult<Token> {
        match try!(self.get_label(c)) {
            None => match c.get_definition() {
                Some((ref left, ref right)) => self.join_tokens(left, right),
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
            Some(s) => Ok(Token::Atom(s)),
        }
    }
    fn join_tokens(&self, app: &T, arg: &T) -> ZiaResult<Token> {
        Ok(try!(self.get_token(&app)) + try!(self.get_token(&arg)))
    }
    fn expand_as_token(&self, c: &T) -> ZiaResult<Token> {
        match c.get_definition() {
            Some((app, arg)) => self.join_tokens(&app, &arg),
            None => self.get_token(c),
        }
    }
}
