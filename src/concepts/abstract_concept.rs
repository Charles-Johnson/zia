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

use reading::{ConcreteReader, GetDefinition, GetReduction};
use writing::{ConcreteWriter, RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};

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

impl<T> ConcreteReader for AbstractConcept<T> {
    type C = T;
    fn read_concrete(&self) -> &T {
        &self.concrete_concept
    }
}

impl<T> ConcreteWriter for AbstractConcept<T> {
    type C = T;
    fn write_concrete(&mut self) -> &mut T {
        &mut self.concrete_concept
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

impl<T> RemoveDefinition for AbstractConcept<T> {
    fn remove_definition(&mut self) {
        self.definition = None
    }
}

impl<T> GetReduction for AbstractConcept<T> {
    fn get_reduction(&self) -> Option<usize> {
        self.reduces_to
    }
}

impl<T> SetReduction for AbstractConcept<T> {
    fn make_reduce_to(&mut self, concept: usize) {
        self.reduces_to = Some(concept);
    }
}

impl<T> RemoveReduction for AbstractConcept<T> {
    fn make_reduce_to_none(&mut self) {
        self.reduces_to = None;
    }
}
