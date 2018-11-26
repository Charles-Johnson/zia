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
pub mod expander;
pub mod label_getter;
pub mod left_hand_call;
pub mod reduce;

use self::expander::Expander;
use self::label_getter::FindDefinition;
use self::left_hand_call::definer3::definer2::{DeleteNormalForm, RefactorFrom};
use self::left_hand_call::definer3::delete_definition::DeleteDefinition;
use self::left_hand_call::definer3::labeller::{
    AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm,
};
use self::left_hand_call::definer3::Pair;
use self::left_hand_call::{Container, LeftHandCall};
pub use self::reduce::{Reduce, SyntaxFromConcept};
use constants::{DEFINE, REDUCTION};
use std::ops::Add;
use std::{fmt, marker};
use token::Token;
use traits::SyntaxFactory;
use traits::{GetDefinition, Id};
use utils::{ZiaError, ZiaResult};

pub trait MightExpand
where
    Self: marker::Sized,
{
    fn get_expansion(&self) -> Option<(Self, Self)>;
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait HasToken {
    fn get_token(&self) -> Token;
}

pub trait GetNormalForm<T>
where
    Self: marker::Sized,
{
    fn get_normal_form(&self) -> ZiaResult<Option<T>>;
}

pub trait Call<T, U>
where
    Self: Reduce<T, U> + LeftHandCall<T, U> + Expander<T, U>,
    T: RefactorFrom<T>
        + StringFactory
        + AbstractFactory
        + Id
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + fmt::Display
        + GetDefinition<T>
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: HasToken
        + Pair
        + Container
        + MaybeConcept<T>
        + SyntaxFactory<T>
        + Add<U, Output = ZiaResult<U>>,
{
    fn call(&mut self, ast: &U) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref left, ref right)) => if let Some(c) = right.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(self.recursively_reduce(left)).get_token().as_string()),
                    DEFINE => Ok(try!(self.expand_ast_token(left)).as_string()),
                    _ => self.call_as_lefthand(left, right),
                }
            } else {
                self.call_as_lefthand(left, right)
            },
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
}
