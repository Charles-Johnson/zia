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

use std::cell::RefCell;
use std::rc::Rc;
use super::traits::{GetDefinition, GetId, SetId, FindWhatReducesToIt, GetReduction, RemoveReduction, SetDefinition, SetReduction, RemoveDefinition, GetDefinitionOf};

pub type AbstractRef<T> = Rc<RefCell<AbstractConcept<T>>>;

pub struct AbstractConcept<T> {
    id: usize,
    definition: Option<(T, T)>,
    lefthand_of: Vec<T>,
    righthand_of: Vec<T>,
    reduces_to: Option<T>,
    reduces_from: Vec<T>,
}

impl<T> AbstractConcept<T> {
    pub fn new_ref(id: usize) -> AbstractRef<T> {
        Rc::new(RefCell::new(AbstractConcept::new(id)))
    }
    pub fn new(id: usize) -> AbstractConcept<T> {
        AbstractConcept::<T> {
            id,
            definition: None,
            lefthand_of: Vec::new(),
            righthand_of: Vec::new(),
            reduces_to: None,
            reduces_from: Vec::new(),
        }
    }
}

impl<T> SetId for AbstractConcept<T> {
    fn set_id(&mut self, number: usize) {
        self.id = number;
    }
}

impl<T: Clone> GetDefinitionOf<T> for AbstractConcept<T> {
    fn get_lefthand_of(&self) -> Vec<T> {
        self.lefthand_of.clone()
    }
    fn get_righthand_of(&self) -> Vec<T> {
        self.righthand_of.clone()
    }
}

impl<T: Clone> GetDefinition<T> for AbstractConcept<T> {
    fn get_definition(&self) -> Option<(T, T)> {
        self.definition.clone()
    }
}

impl<T: Clone> SetDefinition<T> for AbstractConcept<T> {
    fn set_definition(&mut self, lefthand: &T, righthand: &T) {
        self.definition = Some((lefthand.clone(), righthand.clone()));
    }
    fn add_as_lefthand_of(&mut self, lefthand: &T) {
        self.lefthand_of.push(lefthand.clone());
    }
    fn add_as_righthand_of(&mut self, righthand: &T) {
        self.righthand_of.push(righthand.clone());
    }
}

impl<T: GetId + PartialEq> RemoveDefinition<T> for AbstractConcept<T> {
    fn remove_definition(&mut self) {
        self.definition = None
    }
    fn remove_as_lefthand_of(&mut self, definition: &T) {
        if let Some(pos) = self.lefthand_of.iter().position(|x| *x == *definition) {
            self.lefthand_of.remove(pos);
        } else {
            panic!(
                "Concept number {} does not exist in lefthand_of concept number {}",
                self.get_id(),
                definition.get_id()
            );
        }
    }
    fn remove_as_righthand_of(&mut self, definition: &T) {
        if let Some(pos) = self.righthand_of.iter().position(|x| *x == *definition) {
            self.righthand_of.remove(pos);
        } else {
            panic!(
                "Concept number {} does not exist in righthand_of concept number {}",
                self.get_id(),
                definition.get_id()
            );
        }
    }
}

impl<T> GetId for AbstractConcept<T> {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl<T: Clone> GetReduction<T> for AbstractConcept<T> {
    fn get_reduction(&self) -> Option<T> {
        self.reduces_to.clone()
    }
}

impl<T: Clone> FindWhatReducesToIt<T> for AbstractConcept<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T> {
        self.reduces_from.clone()
    }
}

impl<T: Clone> SetReduction<T> for AbstractConcept<T> {
    fn make_reduce_to(&mut self, concept: &T) {
        self.reduces_to = Some(concept.clone());
    }
    fn make_reduce_from(&mut self, concept: &T) {
        self.reduces_from.push(concept.clone());
    }
}

impl<T: GetId + PartialEq> RemoveReduction<T> for AbstractConcept<T> {
    fn make_reduce_to_none(&mut self) {
        self.reduces_to = None;
    }
    fn no_longer_reduces_from(&mut self, concept: &T) {
        if let Some(pos) = self.reduces_from.iter().position(|x| *x == *concept) {
            self.reduces_from.remove(pos);
        } else {
            panic!(
                "Concept number {} does not think it reduces from concept number {}",
                self.get_id(),
                concept.get_id()
            );
        }
    }
}
