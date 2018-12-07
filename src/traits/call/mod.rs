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
use self::right_hand_call::definer::refactor::delete_normal_form::DeleteReduction;
use self::right_hand_call::definer::{MaybeDisconnected, Pair};
use self::right_hand_call::{Container, MaybeId, RightHandCall};
use constants::{DEFINE, REDUCTION};
use std::fmt::Display;
use std::marker::Sized;
use std::ops::Add;
use traits::{GetDefinition, SyntaxFactory};
use utils::{ZiaError, ZiaResult};

pub trait FindWhatReducesToIt<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T>;
}

pub trait MightExpand
where
    Self: Sized,
{
    fn get_expansion(&self) -> Option<(Self, Self)>;
}

impl<T> MightExpand for T
where
    T: GetDefinition<T>,
{
    fn get_expansion(&self) -> Option<(T, T)> {
        self.get_definition()
    }
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait GetNormalForm
where
    Self: GetReduction<Self> + Sized + Clone,
{
    fn get_normal_form(&self) -> Option<Self> {
        match self.get_reduction() {
            None => None,
            Some(ref n) => match n.get_normal_form() {
                None => Some(n.clone()),
                Some(ref m) => Some(m.clone()),
            },
        }
    }
}

impl<S> GetNormalForm for S where S: GetReduction<S> + Sized + Clone {}

pub trait GetReduction<T> {
    fn get_reduction(&self) -> Option<T>;
}

impl<T, U> GetReduction<T> for U
where
    U: MaybeConcept<T>,
    T: GetReduction<T>,
{
    fn get_reduction(&self) -> Option<T> {
        match self.get_concept() {
            None => None,
            Some(c) => c.get_reduction(),
        }
    }
}

pub trait Call<T, U>
where
    Self: RightHandCall<T, U>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
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
                    REDUCTION => Ok(left.recursively_reduce().to_string()),
                    DEFINE => Ok(left.expand().to_string()),
                    _ => self.call_as_righthand(left, right),
                },
                None => self.call_as_righthand(left, right),
            },
            None => Err(ZiaError::NotAProgram),
        }
    }
}

impl<S, T, U> Call<T, U> for S
where
    S: RightHandCall<T, U>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
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
{
}
