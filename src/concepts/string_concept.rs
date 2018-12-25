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
use reading::{FindWhatReducesToIt, GetDefinitionOf, MaybeString};
use std::collections::HashSet;
use writing::{MakeReduceFrom, NoLongerReducesFrom, RemoveAsDefinitionOf, SetAsDefinitionOf};

pub struct StringConcept<T> {
    concrete_concept: T,
    string: String,
}

impl<T> From<String> for StringConcept<T>
where
    T: Default,
{
    fn from(string: String) -> StringConcept<T> {
        StringConcept::<T> {
            string,
            concrete_concept: T::default(),
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
    fn find_what_reduces_to_it(&self) -> HashSet<usize> {
        self.concrete_concept.find_what_reduces_to_it()
    }
}

impl<T> GetDefinitionOf for StringConcept<T>
where
    T: GetDefinitionOf,
{
    fn get_lefthand_of(&self) -> HashSet<usize> {
        self.concrete_concept.get_lefthand_of()
    }
    fn get_righthand_of(&self) -> HashSet<usize> {
        self.concrete_concept.get_righthand_of()
    }
}

impl<T> SetAsDefinitionOf for StringConcept<T>
where
    T: SetAsDefinitionOf,
{
    fn add_as_lefthand_of(&mut self, lefthand: usize) {
        self.concrete_concept.add_as_lefthand_of(lefthand);
    }
    fn add_as_righthand_of(&mut self, righthand: usize) {
        self.concrete_concept.add_as_righthand_of(righthand);
    }
}

impl<T> RemoveAsDefinitionOf for StringConcept<T>
where
    T: RemoveAsDefinitionOf,
{
    fn remove_as_lefthand_of(&mut self, definition: usize) {
        self.concrete_concept.remove_as_lefthand_of(definition)
    }
    fn remove_as_righthand_of(&mut self, definition: usize) {
        self.concrete_concept.remove_as_righthand_of(definition)
    }
}

impl<T> MakeReduceFrom for StringConcept<T>
where
    T: MakeReduceFrom,
{
    fn make_reduce_from(&mut self, concept: usize) {
        self.concrete_concept.make_reduce_from(concept);
    }
}

impl<T> NoLongerReducesFrom for StringConcept<T>
where
    T: NoLongerReducesFrom,
{
    fn no_longer_reduces_from(&mut self, concept: usize) {
        self.concrete_concept.no_longer_reduces_from(concept);
    }
}
