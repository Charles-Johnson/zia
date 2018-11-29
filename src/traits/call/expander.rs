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
use token::Token;
use traits::call::label_getter::LabelGetter;
use traits::call::{HasToken, MaybeConcept, MightExpand};
use utils::ZiaResult;

pub trait Expander<T>
where
    T: LabelGetter,
    Self: MaybeConcept<T> + HasToken + MightExpand,
{
    fn expand_ast_token(&self) -> ZiaResult<Token> {
        if let Some(ref con) = self.get_concept() {
            con.expand_as_token()
        } else if let Some((ref left, ref right)) = self.get_expansion() {
            Ok(try!(left.expand_ast_token()) + try!(right.expand_ast_token()))
        } else {
            Ok(self.get_token())
        }
    }
}

impl<S, T> Expander<T> for S
where
    T: LabelGetter,
    S: MaybeConcept<T> + HasToken + MightExpand,
{}
