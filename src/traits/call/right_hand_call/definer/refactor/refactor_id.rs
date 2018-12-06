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
use std::marker::Sized;
use traits::call::{GetReduction, FindWhatReducesToIt};
use traits::call::label_getter::GetDefinitionOf;
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::call::right_hand_call::definer::labeller::{SetReduction, SetDefinition};
use traits::call::right_hand_call::definer::refactor::delete_normal_form::RemoveReduction;
use traits::call::right_hand_call::definer::delete_definition::RemoveDefinition;
use traits::{Id, GetDefinition};

pub trait RefactorId<T>
where
    T: Id + RefactorFrom,
    Self: ConceptCleaner<T>,
{
    fn refactor_id(&mut self, before: &mut T, after: &mut T) {
        after.refactor_from(before);
        self.cleanly_remove_concept(before);
    }
}

impl<S, T> RefactorId<T> for S
where
    T: Id + RefactorFrom,
    S: ConceptCleaner<T>,
{
}

pub trait RefactorFrom
where
    Self: Sized 
		+ GetDefinition<Self> 
		+ SetDefinition<Self> 
		+ GetDefinitionOf<Self> 
		+ GetReduction<Self>
		+ FindWhatReducesToIt<Self>
		+ SetReduction<Self>
		+ RemoveReduction<Self>
		+ RemoveDefinition<Self>,
{
    fn refactor_from(&mut self, other: &Self) {
		match other.get_definition() {
			Some((ref left, ref right)) => self.set_definition(left, right),
			None => self.remove_definition(),
		};
		for concept in self.get_lefthand_of() {
			self.remove_as_lefthand_of(&concept);
		}
        for concept in other.get_lefthand_of() {
			self.add_as_lefthand_of(&concept); 
		}
		for concept in self.get_righthand_of() {
			self.remove_as_righthand_of(&concept);
		}
        for concept in other.get_righthand_of() {
			self.add_as_righthand_of(&concept); 
		}
		match other.get_reduction() {
			Some(concept) => self.make_reduce_to(&concept),
			None => self.make_reduce_to_none(),
		};
		for concept in self.find_what_reduces_to_it() {
			self.no_longer_reduces_from(&concept);
		}
		for concept in other.find_what_reduces_to_it() {
			self.make_reduce_from(&concept);
		}
	}
}

impl<T> RefactorFrom for T 
where
    T: Sized 
		+ GetDefinition<T> 
		+ SetDefinition<T> 
		+ GetDefinitionOf<T> 
		+ GetReduction<T>
		+ FindWhatReducesToIt<T>
		+ SetReduction<T>
		+ RemoveReduction<T>
		+ RemoveDefinition<T>,
{
}

pub trait ConceptTidyer<T> {
    fn remove_concept(&mut self, &T);
    fn correct_id(&mut self, usize);
}

pub trait ConceptCleaner<T>
where
    Self: ConceptTidyer<T> + ConceptNumber,
    T: Id,
{
    fn cleanly_remove_concept(&mut self, concept: &T) {
        self.remove_concept(concept);
        for id in concept.get_id()..self.number_of_concepts() {
            self.correct_id(id);
        }
    }
}

impl<S, T> ConceptCleaner<T> for S
where
    S: ConceptTidyer<T> + ConceptNumber,
    T: Id,
{
}
