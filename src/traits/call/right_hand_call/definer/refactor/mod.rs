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
pub mod delete_normal_form;
pub mod refactor_id;

use self::delete_normal_form::DeleteReduction;
use self::refactor_id::{RefactorFrom, RefactorId};
use traits::call::label_getter::LabelGetter;

pub trait Refactor<T>
where
    T: RefactorFrom + Unlabeller,
    Self: RefactorId<T>,
{
    fn refactor(&mut self, before: &mut T, after: &mut T) {
        if before.get_label().is_some() {
            before.unlabel();
        }
        self.refactor_id(before, after)
    }
}

impl<S, T> Refactor<T> for S
where
    T: RefactorFrom + Unlabeller,
    S: RefactorId<T>,
{
}

pub trait Unlabeller
where
    Self: LabelGetter + DeleteReduction,
{
    fn unlabel(&mut self) {
        match self.get_concept_of_label() {
            None => panic!("No label to remove"),
            Some(mut d) => d.delete_reduction(),
        }
    }
}

impl<S> Unlabeller for S where S: LabelGetter + DeleteReduction {}
