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

use self::delete_normal_form::DeleteNormalForm;
use self::refactor_id::{RefactorFrom, RefactorId};
use traits::call::label_getter::LabelGetter;
use utils::ZiaResult;

pub trait Refactor<T>
where
    T: RefactorFrom + DeleteNormalForm + LabelGetter,
    Self: RefactorId<T> + Unlabeller<T>,
{
    fn refactor(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        try!(self.unlabel(before));
        self.refactor_id(before, after)
    }
}

impl<S, T> Refactor<T> for S
where
    T: RefactorFrom + DeleteNormalForm + LabelGetter,
    S: RefactorId<T> + Unlabeller<T>,
{}

pub trait Unlabeller<T>
where
    T: LabelGetter + DeleteNormalForm,
{
    fn unlabel(&mut self, concept: &T) -> ZiaResult<()> {
        match concept.get_concept_of_label() {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
}

impl<S, T> Unlabeller<T> for S where T: LabelGetter + DeleteNormalForm {}
