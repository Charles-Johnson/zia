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

use context::StringConcept;
use reading::{
    Combine, DisplayJoint, FindWhatReducesToIt, GetDefinition, GetDefinitionOf, Label,
    MaybeConcept, Pair, SyntaxFactory,
};
use token::parse_line;
use utils::{ZiaError, ZiaResult};

pub trait SyntaxConverter<T>
where
    Self: SyntaxFinder<T> + Combine<T>,
    T: GetDefinitionOf + GetDefinition + FindWhatReducesToIt,
{
    fn ast_from_expression<U: SyntaxFactory + Pair<U> + MaybeConcept + DisplayJoint>(
        &self,
        s: &str,
    ) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::EmptyParentheses),
            1 => self.ast_from_token::<U>(&tokens[0]),
            2 => self.ast_from_pair::<U>(&tokens[0], &tokens[1]),
            _ => Err(ZiaError::AmbiguousExpression),
        }
    }
    fn ast_from_pair<U: SyntaxFactory + DisplayJoint + MaybeConcept + Pair<U>>(
        &self,
        left: &str,
        right: &str,
    ) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token::<U>(left));
        let righthand = try!(self.ast_from_token::<U>(right));
        Ok(self.combine(&lefthand, &righthand))
    }
    fn ast_from_token<U: SyntaxFactory + MaybeConcept + DisplayJoint + Pair<U>>(
        &self,
        t: &str,
    ) -> ZiaResult<U> {
        if t.contains(' ') {
            self.ast_from_expression::<U>(t)
        } else {
            Ok(self.ast_from_symbol::<U>(t))
        }
    }
}

impl<S, T> SyntaxConverter<T> for S
where
    S: SyntaxFinder<T> + Combine<T>,
    T: GetDefinitionOf + GetDefinition + FindWhatReducesToIt,
{
}

pub trait SyntaxFinder<T>
where
    Self: StringConcept + Label<T>,
    T: FindWhatReducesToIt + GetDefinition,
{
    fn concept_from_label(&self, s: &str) -> Option<usize> {
        match self.get_string_concept(s) {
            None => None,
            Some(c) => self.get_labellee(c),
        }
    }
    fn ast_from_symbol<U: SyntaxFactory>(&self, s: &str) -> U {
        let concept_if_exists = self.concept_from_label(s);
        U::new(s, concept_if_exists)
    }
}

impl<S, T> SyntaxFinder<T> for S
where
    S: StringConcept + Label<T>,
    T: FindWhatReducesToIt + GetDefinition,
{
}
