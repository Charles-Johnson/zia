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
use reading::{FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction, MaybeString};
use writing::{RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};

pub struct StringConcept<T> {
    abstract_concept: T,
    string: String,
}

impl<T> From<String> for StringConcept<T>
where
    T: Default,
{
    fn from(string: String) -> StringConcept<T> {
        StringConcept::<T> {
            string,
            abstract_concept: T::default(),
        }
    }
}

impl<T> MaybeString for StringConcept<T> {
    fn get_string(&self) -> Option<String> {
        Some(self.string.clone())
    }
}

impl<T> FindWhatReducesToIt for StringConcept<T>
where
    T: FindWhatReducesToIt,
{
    fn find_what_reduces_to_it(&self) -> Vec<usize> {
        self.abstract_concept.find_what_reduces_to_it()
    }
}

impl<T> GetDefinitionOf for StringConcept<T>
where
    T: GetDefinitionOf,
{
    fn get_lefthand_of(&self) -> Vec<usize> {
        self.abstract_concept.get_lefthand_of()
    }
    fn get_righthand_of(&self) -> Vec<usize> {
        self.abstract_concept.get_righthand_of()
    }
}

impl<T> GetDefinition for StringConcept<T>
where
    T: GetDefinition,
{
    fn get_definition(&self) -> Option<(usize, usize)> {
        self.abstract_concept.get_definition()
    }
}

impl<T> SetDefinition for StringConcept<T>
where
    T: SetDefinition,
{
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

impl<T> RemoveDefinition for StringConcept<T>
where
    T: RemoveDefinition,
{
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

impl<T> GetReduction for StringConcept<T>
where
    T: GetReduction,
{
    fn get_reduction(&self) -> Option<usize> {
        self.abstract_concept.get_reduction()
    }
}

impl<T> SetReduction for StringConcept<T>
where
    T: SetReduction,
{
    fn make_reduce_to(&mut self, _: usize) {
        panic!("Concept is a string so must be its own normal form",)
    }
    fn make_reduce_from(&mut self, concept: usize) {
        self.abstract_concept.make_reduce_from(concept);
    }
}

impl<T> RemoveReduction for StringConcept<T>
where
    T: RemoveReduction,
{
    fn make_reduce_to_none(&mut self) {
        panic!("Concept is a string so no need to remove reduction.")
    }
    fn no_longer_reduces_from(&mut self, concept: usize) {
        self.abstract_concept.no_longer_reduces_from(concept);
    }
}
