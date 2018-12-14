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
use super::abstract_concept::AbstractConcept;
use super::traits::{
    FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction, MaybeString, Refresh,
    RemoveDefinition, RemoveReduction, SetDefinition, SetReduction,
};

pub struct StringConcept {
    abstract_concept: AbstractConcept,
    string: String,
}

impl StringConcept {
    pub fn new(string: &str) -> StringConcept {
        StringConcept {
            string: string.to_string(),
            abstract_concept: AbstractConcept::new(),
        }
    }
}

impl Refresh for StringConcept {
    fn refresh(&mut self, removed_concept: usize) {
        self.abstract_concept.refresh(removed_concept);
    }
}

impl MaybeString for StringConcept {
    fn get_string(&self) -> Option<String> {
        Some(self.string.clone())
    }
}

impl FindWhatReducesToIt for StringConcept {
    fn find_what_reduces_to_it(&self) -> Vec<usize> {
        self.abstract_concept.find_what_reduces_to_it()
    }
}

impl GetDefinitionOf for StringConcept {
    fn get_lefthand_of(&self) -> Vec<usize> {
        self.abstract_concept.get_lefthand_of()
    }
    fn get_righthand_of(&self) -> Vec<usize> {
        self.abstract_concept.get_righthand_of()
    }
}

impl GetDefinition for StringConcept {
    fn get_definition(&self) -> Option<(usize, usize)> {
        self.abstract_concept.get_definition()
    }
}

impl SetDefinition for StringConcept {
    fn set_definition(&mut self, lefthand: usize, righthand: usize) {
        self.abstract_concept.set_definition(lefthand, righthand);
    }
    fn add_as_lefthand_of(&mut self, lefthand: usize) {
        self.abstract_concept.add_as_lefthand_of(lefthand);
    }
    fn add_as_righthand_of(&mut self, righthand: usize) {
        self.abstract_concept.add_as_righthand_of(righthand);
    }
}

impl RemoveDefinition for StringConcept {
    fn remove_definition(&mut self) {
        self.abstract_concept.remove_definition();
    }
    fn remove_as_lefthand_of(&mut self, definition: usize) {
        self.abstract_concept.remove_as_lefthand_of(definition)
    }
    fn remove_as_righthand_of(&mut self, definition: usize) {
        self.abstract_concept.remove_as_righthand_of(definition)
    }
}

impl GetReduction for StringConcept {
    fn get_reduction(&self) -> Option<usize> {
        self.abstract_concept.get_reduction()
    }
}

impl SetReduction for StringConcept {
    fn make_reduce_to(&mut self, _: usize) {
        panic!("Concept is a string so must be its own normal form",)
    }
    fn make_reduce_from(&mut self, concept: usize) {
        self.abstract_concept.make_reduce_from(concept);
    }
}

impl RemoveReduction for StringConcept {
    fn make_reduce_to_none(&mut self) {
        panic!("Concept number {} is a string so no need to remove reduction.")
    }
    fn no_longer_reduces_from(&mut self, concept: usize) {
        self.abstract_concept.no_longer_reduces_from(concept);
    }
}
