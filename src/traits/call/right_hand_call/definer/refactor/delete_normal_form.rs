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
use traits::call::{GetNormalForm, MaybeConcept};
use utils::{ZiaError, ZiaResult};

pub trait DeleteNormalForm
where
    Self: GetNormalForm<Self> + RemoveNormalForm<Self>,
{
    fn delete_normal_form(&mut self) -> ZiaResult<()> {
        match try!(self.get_normal_form()) {
            None => Ok(()),
            Some(mut n) => {
                n.remove_normal_form_of(self);
                self.remove_normal_form();
                Ok(())
            }
        }
    }
}

impl<T> DeleteNormalForm for T where T: GetNormalForm<T> + RemoveNormalForm<T> {}

pub trait DeleteReduction<T>
where
    Self: MaybeConcept<T>,
    T: DeleteNormalForm,
{
    fn delete_reduction(&mut self) -> ZiaResult<()> {
        if let Some(mut concept) = self.get_concept() {
            concept.delete_normal_form()
        } else {
            Err(ZiaError::Redundancy(
                "Removing the normal form of a symbol that was never previously used \
                 is redundant"
                    .to_string(),
            ))
        }
    }
}

impl<T, U> DeleteReduction<T> for U
where
    U: MaybeConcept<T>,
    T: DeleteNormalForm,
{
}

pub trait RemoveNormalForm<T> {
    fn remove_normal_form(&mut self);
    fn remove_normal_form_of(&mut self, &T);
}
