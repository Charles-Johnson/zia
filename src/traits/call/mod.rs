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
mod expander;
pub mod label_getter;
pub mod left_hand_call;
mod reduce;

use self::expander::Expander;
use self::label_getter::LabelGetter;
use self::left_hand_call::definer3::definer2::delete_normal_form::DeleteNormalForm;
use self::left_hand_call::definer3::definer2::refactor_id::RefactorFrom;
use self::left_hand_call::definer3::delete_definition::DeleteDefinition;
use self::left_hand_call::definer3::labeller::{
    AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm,
};
use self::left_hand_call::definer3::Pair;
use self::left_hand_call::{Container, LeftHandCall};
pub use self::reduce::{Reduce, SyntaxFromConcept};
use constants::{DEFINE, REDUCTION};
use std::marker;
use std::ops::Add;
use token::Token;
use traits::SyntaxFactory;
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
    Self: LeftHandCall<T, U>,
    T: RefactorFrom<T>
        + StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + LabelGetter,
    U: Reduce<T>
        + Expander<T>
        + HasToken
        + Pair
        + Container
        + MaybeConcept<T>
        + SyntaxFactory<T>
        + Add<U, Output = ZiaResult<U>>,
{
    fn call(&mut self, ast: &U) -> ZiaResult<String> {
        println!("Calling AST: {:?}", ast.get_token());
        match ast.get_expansion() {
            Some((ref left, ref right)) => if let Some(c) = right.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(left.recursively_reduce()).get_token().as_string()),
                    DEFINE => Ok(try!(left.expand_ast_token()).as_string()),
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

impl<S, T, U> Call<T, U> for S
where
    S: LeftHandCall<T, U>,
    T: RefactorFrom<T>
        + StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + LabelGetter,
    U: HasToken
        + Expander<T>
        + Reduce<T>
        + Pair
        + Container
        + MaybeConcept<T>
        + SyntaxFactory<T>
        + Add<U, Output = ZiaResult<U>>,
{}
