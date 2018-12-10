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
pub mod concept_maker;
pub mod delete_definition;
pub mod labeller;
pub mod refactor;

pub use context::traits::ConceptNumber;
use constants::LABEL;
use std::marker::Sized;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::{FindWhatReducesToIt, GetReduction};
use traits::{GetDefinition, GetId};
pub use ast::traits::Pair;


pub trait MaybeDisconnected
where
    Self: GetReduction<Self>
        + FindWhatReducesToIt<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + GetId
        + Sized,
{
    fn is_disconnected(&self) -> bool {
        self.get_reduction().is_none()
            && self.get_definition().is_none()
            && self.get_lefthand_of().is_empty()
            && self.righthand_of_without_label_is_empty()
            && self.find_what_reduces_to_it().is_empty()
    }
    fn righthand_of_without_label_is_empty(&self) -> bool {
        for concept in self.get_righthand_of() {
            if let Some((left, _)) = concept.get_definition() {
                if left.get_id() != LABEL {
                    return false;
                }
            }
        }
        true
    }
}

impl<T> MaybeDisconnected for T where
    T: GetReduction<T> + FindWhatReducesToIt<T> + GetDefinition<T> + GetDefinitionOf<T> + GetId
{
}
