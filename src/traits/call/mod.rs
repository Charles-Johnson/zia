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

pub use self::reduce::{Reduce, SyntaxFromConcept};
pub use concepts::traits::{GetReduction, FindWhatReducesToIt};
use std::marker::Sized;
use traits::GetDefinition;
pub use ast::traits::{MightExpand, MaybeConcept};

impl<T> MightExpand for T
where
    T: GetDefinition<T>,
{
    fn get_expansion(&self) -> Option<(T, T)> {
        self.get_definition()
    }
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
