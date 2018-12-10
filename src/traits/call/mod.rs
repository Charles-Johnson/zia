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
pub mod reduce;
pub mod right_hand_call;

use self::expander::Expander;
pub use self::reduce::{Reduce, SyntaxFromConcept};
use self::right_hand_call::definer::delete_definition::DeleteDefinition;
use self::right_hand_call::definer::labeller::{
    AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm,
};
use self::right_hand_call::definer::refactor::delete_normal_form::DeleteReduction;
use self::right_hand_call::definer::MaybeDisconnected;
use self::right_hand_call::{Container, RightHandCall};
use concepts::Display;
use constants::{DEFINE, REDUCTION};
use std::marker::Sized;
use traits::GetDefinition;
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

pub trait Call<T>
where
    Self: RightHandCall<T>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display,
{
    fn call<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref mut left, ref right)) => self.call_pair(left, right),
            None => {
                match self.try_expanding_then_call(ast) {
                    Ok(s) => return Ok(s),
                    Err(e) => {
                        if let ZiaError::NotAProgram = e {
                        } else {
                            return Err(e);
                        }
                    }
                };
                self.try_reducing_then_call(ast)
            }
        }
    }
    fn call_pair<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        left: &mut U,
        right: &U,
    ) -> ZiaResult<String> {
        match right.get_concept() {
            Some(c) => match c.get_id() {
                REDUCTION => Ok(left.recursively_reduce().to_string()),
                DEFINE => Ok(left.expand().to_string()),
                _ => {
                    let right_reduction = c.get_reduction();
                    if let Some(r) = right_reduction {
                        self.call_pair(left, &r.to_ast())
                    } else {
                        self.call_as_righthand(left, right)
                    }
                }
            },
            None => self.call_as_righthand(left, right),
        }
    }
    fn try_expanding_then_call<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let expansion = &ast.expand();
        if expansion != ast {
            self.call(expansion)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
    fn try_reducing_then_call<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let normal_form = &ast.recursively_reduce();
        if normal_form != ast {
            self.call(normal_form)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
}

impl<S, T> Call<T> for S
where
    S: RightHandCall<T>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display,
{
}
