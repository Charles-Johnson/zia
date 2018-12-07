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
use concepts::ConceptRef;
use std::cell::RefCell;
use std::rc::Rc;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::right_hand_call::definer::delete_definition::RemoveDefinition;
use traits::call::right_hand_call::definer::labeller::{SetDefinition, SetReduction};
use traits::call::right_hand_call::definer::refactor::delete_normal_form::RemoveReduction;
use traits::call::{FindWhatReducesToIt, GetReduction};
use traits::{GetDefinition, Id};

pub type AbstractRef = Rc<RefCell<AbstractConcept>>;

pub struct AbstractConcept {
    id: usize,
    definition: Option<(ConceptRef, ConceptRef)>,
    lefthand_of: Vec<ConceptRef>,
    righthand_of: Vec<ConceptRef>,
    reduces_to: Option<ConceptRef>,
    reduces_from: Vec<ConceptRef>,
}

impl AbstractConcept {
    pub fn new_ref(id: usize) -> AbstractRef {
        Rc::new(RefCell::new(AbstractConcept::new(id)))
    }
    pub fn new(id: usize) -> AbstractConcept {
        AbstractConcept {
            id,
            definition: None,
            lefthand_of: Vec::new(),
            righthand_of: Vec::new(),
            reduces_to: None,
            reduces_from: Vec::new(),
        }
    }
    pub fn set_id(&mut self, number: usize) {
        self.id = number;
    }
}

impl GetDefinitionOf<ConceptRef> for AbstractConcept {
    fn get_lefthand_of(&self) -> Vec<ConceptRef> {
        self.lefthand_of.clone()
    }
    fn get_righthand_of(&self) -> Vec<ConceptRef> {
        self.righthand_of.clone()
    }
}

impl GetDefinition<ConceptRef> for AbstractConcept {
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        self.definition.clone()
    }
}

impl SetDefinition<ConceptRef> for AbstractConcept {
    fn set_definition(&mut self, lefthand: &ConceptRef, righthand: &ConceptRef) {
        self.definition = Some((lefthand.clone(), righthand.clone()));
    }
    fn add_as_lefthand_of(&mut self, lefthand: &ConceptRef) {
        self.lefthand_of.push(lefthand.clone());
    }
    fn add_as_righthand_of(&mut self, righthand: &ConceptRef) {
        self.righthand_of.push(righthand.clone());
    }
}

impl RemoveDefinition<ConceptRef> for AbstractConcept {
    fn remove_definition(&mut self) {
        self.definition = None
    }
    fn remove_as_lefthand_of(&mut self, definition: &ConceptRef) {
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
    fn remove_as_righthand_of(&mut self, definition: &ConceptRef) {
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

impl Id for AbstractConcept {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl GetReduction<ConceptRef> for AbstractConcept {
    fn get_reduction(&self) -> Option<ConceptRef> {
        self.reduces_to.clone()
    }
}

impl FindWhatReducesToIt<ConceptRef> for AbstractConcept {
    fn find_what_reduces_to_it(&self) -> Vec<ConceptRef> {
        self.reduces_from.clone()
    }
}

impl SetReduction<ConceptRef> for AbstractConcept {
    fn make_reduce_to(&mut self, concept: &ConceptRef) {
        self.reduces_to = Some(concept.clone());
    }
    fn make_reduce_from(&mut self, concept: &ConceptRef) {
        self.reduces_from.push(concept.clone());
    }
}

impl RemoveReduction<ConceptRef> for AbstractConcept {
    fn make_reduce_to_none(&mut self) {
        self.reduces_to = None;
    }
    fn no_longer_reduces_from(&mut self, concept: &ConceptRef) {
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
