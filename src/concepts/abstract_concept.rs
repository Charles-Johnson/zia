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

use reading::{FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction};
use std::collections::HashSet;
use writing::{
    MakeReduceFrom, NoLongerReducesFrom, RemoveAsDefinitionOf, RemoveDefinition, RemoveReduction,
    SetAsDefinitionOf, SetDefinition, SetReduction,
};

pub struct AbstractConcept<T> {
    concrete_concept: T,
    definition: Option<(usize, usize)>,
    reduces_to: Option<usize>,
}

impl<T> Default for AbstractConcept<T>
where
    T: Default,
{
    fn default() -> AbstractConcept<T> {
        AbstractConcept::<T> {
            concrete_concept: T::default(),
            definition: None,
            reduces_to: None,
        }
    }
}

impl<T> GetDefinitionOf for AbstractConcept<T>
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

impl<T> GetDefinition for AbstractConcept<T> {
    fn get_definition(&self) -> Option<(usize, usize)> {
        self.definition
    }
}

impl<T> SetDefinition for AbstractConcept<T> {
    fn set_definition(&mut self, lefthand: usize, righthand: usize) {
        self.definition = Some((lefthand, righthand));
    }
}

impl<T> SetAsDefinitionOf for AbstractConcept<T>
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

impl<T> RemoveDefinition for AbstractConcept<T> {
    fn remove_definition(&mut self) {
        self.definition = None
    }
}

impl<T> RemoveAsDefinitionOf for AbstractConcept<T>
where
    T: RemoveAsDefinitionOf,
{
    fn remove_as_lefthand_of(&mut self, definition: usize) {
        self.concrete_concept.remove_as_lefthand_of(definition);
    }
    fn remove_as_righthand_of(&mut self, definition: usize) {
        self.concrete_concept.remove_as_righthand_of(definition);
    }
}

impl<T> GetReduction for AbstractConcept<T> {
    fn get_reduction(&self) -> Option<usize> {
        self.reduces_to
    }
}

impl<T> FindWhatReducesToIt for AbstractConcept<T>
where
    T: FindWhatReducesToIt,
{
    fn find_what_reduces_to_it(&self) -> HashSet<usize> {
        self.concrete_concept.find_what_reduces_to_it()
    }
}

impl<T> SetReduction for AbstractConcept<T> {
    fn make_reduce_to(&mut self, concept: usize) {
        self.reduces_to = Some(concept);
    }
}

impl<T> MakeReduceFrom for AbstractConcept<T>
where
    T: MakeReduceFrom,
{
    fn make_reduce_from(&mut self, concept: usize) {
        self.concrete_concept.make_reduce_from(concept);
    }
}

impl<T> RemoveReduction for AbstractConcept<T> {
    fn make_reduce_to_none(&mut self) {
        self.reduces_to = None;
    }
}

impl<T> NoLongerReducesFrom for AbstractConcept<T>
where
    T: NoLongerReducesFrom,
{
    fn no_longer_reduces_from(&mut self, concept: usize) {
        self.concrete_concept.no_longer_reduces_from(concept);
    }
}
