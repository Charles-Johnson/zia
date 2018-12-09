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
use concepts::abstract_concept::AbstractConcept;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use traits::call::label_getter::{GetDefinitionOf, MaybeString};
use traits::call::right_hand_call::definer::delete_definition::RemoveDefinition;
use traits::call::right_hand_call::definer::labeller::{SetDefinition, SetReduction};
use traits::call::right_hand_call::definer::refactor::delete_normal_form::RemoveReduction;
use traits::call::{FindWhatReducesToIt, GetReduction};
use traits::{GetDefinition, Id};

pub type StringRef<T> = Rc<RefCell<StringConcept<T>>>;

pub struct StringConcept<T> {
    abstract_concept: AbstractConcept<T>,
    string: String,
}

impl<T> StringConcept<T> {
    pub fn new_ref(id: usize, string: &str) -> StringRef<T> {
        Rc::new(RefCell::new(StringConcept::<T> {
            string: string.to_string(),
            abstract_concept: AbstractConcept::new(id),
        }))
    }
    pub fn set_id(&mut self, number: usize) {
        self.abstract_concept.set_id(number);
    }
}

impl<T> MaybeString for StringConcept<T> {
    fn get_string(&self) -> Option<String> {
        Some(self.string.clone())
    }
}

impl<T: Clone> FindWhatReducesToIt<T> for StringConcept<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T> {
        self.abstract_concept.find_what_reduces_to_it()
    }
}

impl<T: Clone> GetDefinitionOf<T> for StringConcept<T> {
    fn get_lefthand_of(&self) -> Vec<T> {
        self.abstract_concept.get_lefthand_of()
    }
    fn get_righthand_of(&self) -> Vec<T> {
        self.abstract_concept.get_righthand_of()
    }
}

impl<T: Clone> GetDefinition<T> for StringConcept<T> {
    fn get_definition(&self) -> Option<(T, T)> {
        self.abstract_concept.get_definition()
    }
}

impl<T: Clone> SetDefinition<T> for StringConcept<T> {
    fn set_definition(&mut self, lefthand: &T, righthand: &T) {
        self.abstract_concept.set_definition(lefthand, righthand);
    }
    fn add_as_lefthand_of(&mut self, lefthand: &T) {
        self.abstract_concept.add_as_lefthand_of(lefthand);
    }
    fn add_as_righthand_of(&mut self, righthand: &T) {
        self.abstract_concept.add_as_righthand_of(righthand);
    }
}

impl<T: Id + PartialEq> RemoveDefinition<T> for StringConcept<T> {
    fn remove_definition(&mut self) {
        self.abstract_concept.remove_definition();
    }
    fn remove_as_lefthand_of(&mut self, definition: &T) {
        self.abstract_concept.remove_as_lefthand_of(definition)
    }
    fn remove_as_righthand_of(&mut self, definition: &T) {
        self.abstract_concept.remove_as_righthand_of(definition)
    }
}

impl<T> Id for StringConcept<T> {
    fn get_id(&self) -> usize {
        self.abstract_concept.get_id()
    }
}

impl<T: Clone> GetReduction<T> for StringConcept<T> {
    fn get_reduction(&self) -> Option<T> {
        self.abstract_concept.get_reduction()
    }
}

impl<T: Clone> SetReduction<T> for StringConcept<T> {
    fn make_reduce_to(&mut self, _: &T) {
        panic!(
            "Concept number {} is a string so must be its own normal form",
            self.get_id()
        )
    }
    fn make_reduce_from(&mut self, concept: &T) {
        self.abstract_concept.make_reduce_from(concept);
    }
}

impl<T: Id + PartialEq> RemoveReduction<T> for StringConcept<T> {
    fn make_reduce_to_none(&mut self) {
        panic!(
            "Concept number {} is a string so no need to remove reduction.",
            self.get_id()
        )
    }
    fn no_longer_reduces_from(&mut self, concept: &T) {
        self.abstract_concept.no_longer_reduces_from(concept);
    }
}

impl<T> fmt::Display for StringConcept<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
