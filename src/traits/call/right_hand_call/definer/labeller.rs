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
pub use context::traits::{BlindConceptAdder, LabelConcept};
use std::marker;
use traits::call::right_hand_call::Container;
use traits::call::{GetNormalForm, GetReduction};
use utils::{ZiaError, ZiaResult};
pub use concepts::traits::{SetDefinition, SetReduction, AbstractFactory, StringFactory};



pub trait UpdateNormalForm
where
    Self: GetNormalForm + SetReduction<Self> + PartialEq,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        if let Some(n) = normal_form.get_normal_form() {
            if *self == n {
                return Err(ZiaError::CyclicReduction);
            }
        }
        if let Some(ref n) = self.get_reduction() {
            if n == normal_form {
                return Err(ZiaError::RedundantReduction);
            }
        }
        self.make_reduce_to(normal_form);
        normal_form.make_reduce_from(self);
        Ok(())
    }
}

impl<T> UpdateNormalForm for T where T: GetNormalForm + SetReduction<Self> + PartialEq {}

pub trait InsertDefinition
where
    Self: SetDefinition<Self> + marker::Sized + Container + GetReduction<Self>,
{
    fn insert_definition(&mut self, lefthand: &mut Self, righthand: &mut Self) -> ZiaResult<()> {
        if lefthand.contains(self) || righthand.contains(self) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.check_reductions(lefthand));
            try!(self.check_reductions(righthand));
            self.set_definition(lefthand, righthand);
            lefthand.add_as_lefthand_of(self);
            righthand.add_as_righthand_of(self);
            Ok(())
        }
    }
    fn check_reductions(&self, concept: &Self) -> ZiaResult<()> {
        if let Some(ref r) = concept.get_reduction() {
            if r == self || r.contains(self) {
                Err(ZiaError::ExpandingReduction)
            } else {
                self.check_reductions(r)
            }
        } else {
            Ok(())
        }
    }
}

impl<T> InsertDefinition for T where
    T: SetDefinition<T> + marker::Sized + Container + GetReduction<Self>
{
}
