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
use concepts::{AbstractRef, ConceptRef};
use std::cell::RefCell;
use std::rc::Rc;
use traits::{Application, Id, Label, NormalForm, RefactorFrom};
use utils::{ZiaError, ZiaResult};

pub struct AbstractConcept {
    id: usize,
    definition: Option<(ConceptRef, ConceptRef)>,
    lefthand_of: Vec<ConceptRef>,
    righthand_of: Vec<ConceptRef>,
    normal_form: Option<ConceptRef>,
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
            normal_form: None,
            reduces_from: Vec::new(),
        }
    }
    pub fn set_id(&mut self, number: usize) {
        self.id = number;
    }
}

impl RefactorFrom<ConceptRef> for AbstractConcept {
    fn refactor_from(&mut self, other: &ConceptRef) -> ZiaResult<()> {
        // In order to compare `other` to `self`, `other` needs to be borrowed. If `other == self`,
        // then borrowing `other` will panic because `other` is already mutably borrowed.
        if other.check_borrow_err() {
            return Err(ZiaError::Redundancy(
                "Concept already has this definition".to_string(),
            ));
        }
        self.definition = other.get_definition();
        self.lefthand_of = other.get_lefthand_of();
        self.righthand_of = other.get_righthand_of();
        self.normal_form = try!(other.get_normal_form());
        self.reduces_from = other.get_reduces_from();
        Ok(())
    }
}

impl Application<ConceptRef> for AbstractConcept {
    fn get_lefthand_of(&self) -> Vec<ConceptRef> {
        self.lefthand_of.clone()
    }
    fn get_righthand_of(&self) -> Vec<ConceptRef> {
        self.righthand_of.clone()
    }
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        self.definition.clone()
    }
    fn set_definition(&mut self, lefthand: &ConceptRef, righthand: &ConceptRef) {
        self.definition = Some((lefthand.clone(), righthand.clone()));
    }
    fn add_lefthand_of(&mut self, lefthand: &ConceptRef) {
        self.lefthand_of.push(lefthand.clone());
    }
    fn add_righthand_of(&mut self, righthand: &ConceptRef) {
        self.righthand_of.push(righthand.clone());
    }
    fn delete_definition(&mut self) {
        self.definition = None
    }
    fn delete_lefthand_of(&mut self, definition: &ConceptRef) {
        self.lefthand_of.remove_item(definition);
    }
    fn delete_righthand_of(&mut self, definition: &ConceptRef) {
        self.righthand_of.remove_item(definition);
    }
}

impl Id for AbstractConcept {
    fn get_id(&self) -> usize {
        self.id
    }
}

impl NormalForm<ConceptRef> for AbstractConcept {
    fn get_normal_form(&self) -> ZiaResult<Option<ConceptRef>> {
        match self.normal_form {
            None => Ok(None),
            Some(ref n) => {
                if n.check_borrow_err() {
                    return Err(ZiaError::Borrow(
                        "Error while borrowing normal form".to_string(),
                    ));
                }
                match try!(n.get_normal_form()) {
                    None => Ok(Some(n.clone())),
                    Some(ref m) => Ok(Some(m.clone())),
                }
            }
        }
    }
    fn get_reduces_from(&self) -> Vec<ConceptRef> {
        let mut reduces_from: Vec<ConceptRef> = Vec::new();
        for concept in self.reduces_from.clone() {
            reduces_from.push(concept.clone());
            for concept2 in concept.get_reduces_from() {
                reduces_from.push(concept2);
            }
        }
        reduces_from
    }
    fn set_normal_form(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        // If `concept.get_normal_form() == self` then calling `concept.get_normal_form()` will
        // raise an error due to borrowing self which has already been mutably borrowed.
        if concept.get_normal_form().is_err() {
            return Err(ZiaError::Loop("Cannot create a reduction loop".to_string()));
        }
        if let Some(ref n) = try!(self.get_normal_form()) {
            if n == concept {
                return Err(ZiaError::Redundancy(
                    "Concept already has this normal form.".to_string(),
                ));
            }
        }
        self.normal_form = Some(concept.clone());
        Ok(())
    }
    fn add_reduces_from(&mut self, concept: &ConceptRef) {
        self.reduces_from.push(concept.clone());
    }
    fn remove_normal_form(&mut self) {
        self.normal_form = None;
    }
    fn remove_reduces_from(&mut self, concept: &ConceptRef) {
        self.reduces_from.remove_item(concept);
    }
}

impl Label<ConceptRef> for AbstractConcept {}
