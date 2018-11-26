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
	along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
use std::fmt;
use token::Token;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, HasToken, LabelGetter, MaybeConcept, MightExpand,
};
use utils::{ZiaError, ZiaResult};

pub trait Expander<T, U>
where
    T: GetNormalForm<T> + FindDefinition<T> + Clone + PartialEq + fmt::Display + GetDefinition<T>,
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
    T: GetNormalForm<T> + FindDefinition<T> + Clone + PartialEq + fmt::Display + GetDefinition<T>,
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
