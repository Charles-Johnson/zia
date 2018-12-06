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
pub mod reduce;
pub mod right_hand_call;

use self::expander::Expander;
use self::label_getter::LabelGetter;
pub use self::reduce::{Reduce, SyntaxFromConcept};
use self::right_hand_call::definer::delete_definition::DeleteDefinition;
use self::right_hand_call::definer::labeller::{
    AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm,
};
use self::right_hand_call::definer::refactor::delete_normal_form::DeleteNormalForm;
use self::right_hand_call::definer::refactor::refactor_id::RefactorFrom;
use self::right_hand_call::definer::{MaybeDisconnected, Pair};
use self::right_hand_call::{Container, MaybeId, RightHandCall};
use constants::{DEFINE, REDUCTION};
use std::fmt::Display;
use std::marker;
use std::ops::Add;
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

pub trait GetNormalForm<T>
where
    Self: marker::Sized,
{
    fn get_normal_form(&self) -> ZiaResult<Option<T>>;
}

pub trait GetReduction<T>
where
{
    fn get_reduction(&self) -> Option<T>;
}

impl<T, U> GetNormalForm<T> for U
where
    T: GetNormalForm<T>,
    U: MaybeConcept<T>,
{
    fn get_normal_form(&self) -> ZiaResult<Option<T>> {
        match self.get_concept() {
            None => Ok(None),
            Some(c) => c.get_normal_form(),
        }
    }
}

pub trait Call<T, U>
where
    Self: RightHandCall<T, U>,
    T: RefactorFrom
        + StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + LabelGetter
        + MaybeDisconnected
        + Display,
    U: Reduce<T>
        + Expander<T>
        + Pair<U>
        + Container
        + MaybeId<T>
        + SyntaxFactory<T>
        + Add<U, Output = U>
        + Clone
        + Display,
{
    fn call(&mut self, ast: &U) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref mut left, ref right)) => match right.get_id() {
                Some(id) => match id {
                    REDUCTION => Ok(try!(left.recursively_reduce()).to_string()),
                    DEFINE => Ok(try!(left.expand()).to_string()),
                    _ => self.call_as_righthand(left, right),
                },
                None => self.call_as_righthand(left, right),
            },
            None => Err(ZiaError:: NotAProgram),
        }
    }
}

impl<S, T, U> Call<T, U> for S
where
    S: RightHandCall<T, U>,
    T: RefactorFrom
        + StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + LabelGetter
        + MaybeDisconnected
        + Display,
    U: Expander<T>
        + Reduce<T>
        + Pair<U>
        + Container
        + MaybeId<T>
        + SyntaxFactory<T>
        + Add<U, Output = U>
        + Display,
{}
