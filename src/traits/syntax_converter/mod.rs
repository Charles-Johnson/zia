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
pub mod label;

use self::label::Label;
use std::ops::Add;
use token::parse_line;
use traits::{GetDefinition, Id, SyntaxFactory};
use utils::{ZiaError, ZiaResult};

pub trait SyntaxConverter<T, U>
where
    Self: SyntaxFinder<T>,
    T: Clone + Id + GetDefinition<T> + Label<T>,
    U: SyntaxFactory<T> + Add<U, Output = U>,
{
    fn ast_from_expression(&mut self, s: &str) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::EmptyParentheses),
            1 => Ok(self.ast_from_atom(&tokens[0])),
            2 => self.ast_from_pair(&tokens[0], &tokens[1]),
            _ => Err(ZiaError::AmbiguousExpression),
        }
    }
    fn ast_from_atom(&mut self, s: &str) -> U {
        let concept_if_exists = self.concept_from_label(s);
        U::new(s, concept_if_exists)
    }
    fn ast_from_pair(&mut self, left: &str, right: &str) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token(left));
        let righthand = try!(self.ast_from_token(right));
        Ok(lefthand + righthand)
    }
    fn ast_from_token(&mut self, t: &str) -> ZiaResult<U> {
        if t.contains(' ') {
            self.ast_from_expression(t)
        } else {
            Ok(self.ast_from_atom(t))
        }
    }
}

impl<S, T, U> SyntaxConverter<T, U> for S
where
    S: SyntaxFinder<T>,
    T: Clone + Id + GetDefinition<T> + Label<T>,
    U: SyntaxFactory<T> + Add<U, Output = U>,
{
}

pub trait SyntaxFinder<T>
where
    T: Label<T> + GetDefinition<T> + Clone + Id,
{
    fn get_string_concept(&self, &str) -> Option<T>;
    fn concept_from_label(&self, s: &str) -> Option<T> {
        match self.get_string_concept(s) {
            None => None,
            Some(c) => c.get_labellee(),
        }
    }
}
