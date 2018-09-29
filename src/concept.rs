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
use utils::ZiaResult;
use traits::{Application, Definition, NormalForm, Reduction, Label};

pub enum ConceptRef {
    Abstract(AbstractRef),
    String(StringRef),
}

pub type AbstractRef = Rc<RefCell<AbstractConcept>>;
pub type StringRef = Rc<RefCell<StringConcept>>;

impl ConceptRef {
    pub fn set_id(&mut self, number: usize) {
        match *self {
            ConceptRef::Abstract(ref mut r) => r.borrow_mut().set_id(number),
            ConceptRef::String(ref mut r) => r.borrow_mut().set_id(number),
        }
    }
    pub fn delete_normal_form(&mut self) -> ZiaResult<()> {
        match self.get_normal_form() {
            None => (),
            Some(mut n) => {
                n.remove_reduces_from(self);
                self.remove_normal_form();
            }
        };
        Ok(())
    }
}

impl Clone for ConceptRef {
    fn clone(&self) -> Self {
        match *self {
            ConceptRef::Abstract(ref r) => ConceptRef::Abstract(r.clone()),
            ConceptRef::String(ref r) => ConceptRef::String(r.clone()),
        }
    }
}

impl Application<ConceptRef> for ConceptRef {
    fn get_argument_of(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_argument_of(),
            ConceptRef::String(ref c) => c.borrow().get_argument_of(),
        }
    }
    fn get_applicand_of(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_applicand_of(),
            ConceptRef::String(ref c) => c.borrow().get_applicand_of(),
        }
    }
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_definition(),
            ConceptRef::String(ref c) => c.borrow().get_definition(),
        }
    }
    fn set_definition(&mut self, applicand: &ConceptRef, argument: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().set_definition(applicand, argument),
            ConceptRef::String(ref mut c) => c.borrow_mut().set_definition(applicand, argument),
        }
    }
    fn add_applicand_of(&mut self, applicand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_applicand_of(applicand),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_applicand_of(applicand),
        }
    }
    fn add_argument_of(&mut self, argument: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_argument_of(argument),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_argument_of(argument),
        }
    }
}

impl Definition<ConceptRef> for ConceptRef {}

impl NormalForm<ConceptRef> for ConceptRef {
    fn get_id(&self) -> usize {
        match *self {
            ConceptRef::Abstract(ref r) => 
                r.borrow().get_id(),
            ConceptRef::String(ref r) => 
                r.borrow().get_id(),
        }
    }
    fn get_normal_form(&self) -> Option<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_normal_form(),
            ConceptRef::String(ref c) => c.borrow().get_normal_form(),
        }
    }
    fn get_reduces_from(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_reduces_from(),
            ConceptRef::String(ref c) => c.borrow().get_reduces_from(),
        }
    }
    fn set_normal_form(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().set_normal_form(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().set_normal_form(concept),
        }
    }
    fn add_reduces_from(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_reduces_from(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_reduces_from(concept),
        }
    }
    fn remove_normal_form(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_normal_form(),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_normal_form(),
        }
    }
    fn remove_reduces_from(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_reduces_from(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_reduces_from(concept),
        }
    }
}

impl Reduction for ConceptRef {}

impl Label<ConceptRef> for ConceptRef {}

impl PartialEq for ConceptRef {
    fn eq(&self, other: &ConceptRef) -> bool {
         self.get_id() == other.get_id()
    }
}

pub struct StringConcept {
    abstract_concept: AbstractConcept,
    string: String,
}

impl StringConcept {
    pub fn new_ref(id: usize, string: &str) -> StringRef {
        Rc::new(RefCell::new(StringConcept{
            string: string.to_string(),
            abstract_concept: AbstractConcept::new(id),
        }))
    }
    fn set_id(&mut self, number: usize) {
        self.abstract_concept.set_id(number);
    }
    pub fn get_string(&self) -> String {
        self.string.clone()
    }
}

impl Application<ConceptRef> for StringConcept {
    fn get_applicand_of(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_applicand_of()
    }
    fn get_argument_of(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_argument_of()
    }
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        self.abstract_concept.get_definition()
    }
    fn set_definition(&mut self, applicand: &ConceptRef, argument: &ConceptRef) {
        self.abstract_concept.set_definition(applicand, argument);
    }
    fn add_applicand_of(&mut self, applicand: &ConceptRef) {
        self.abstract_concept.add_applicand_of(applicand);
    }
    fn add_argument_of(&mut self, argument: &ConceptRef) {
        self.abstract_concept.add_argument_of(argument);
    }
}

impl NormalForm<ConceptRef> for StringConcept {
    fn get_id(&self) -> usize {
        self.abstract_concept.get_id()
    }
    fn get_normal_form(&self) -> Option<ConceptRef> {
        self.abstract_concept.get_normal_form()
    }
    fn get_reduces_from(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_reduces_from()
    }
    fn set_normal_form(&mut self, concept: &ConceptRef) {
        self.abstract_concept.set_normal_form(concept);
    }
    fn add_reduces_from(&mut self, concept: &ConceptRef) {
        self.abstract_concept.add_reduces_from(concept);
    }
    fn remove_normal_form(&mut self) {
        self.abstract_concept.remove_normal_form();
    }
    fn remove_reduces_from(&mut self, concept: &ConceptRef) {
        self.abstract_concept.remove_reduces_from(concept);
    }
}

impl Label<ConceptRef> for StringConcept {}

pub struct AbstractConcept {
    id: usize,
    definition: Option<(ConceptRef, ConceptRef)>,
    applicand_of: Vec<ConceptRef>,
    argument_of: Vec<ConceptRef>,
    normal_form: Option<ConceptRef>,
    reduces_from: Vec<ConceptRef>,
}

impl AbstractConcept {
    pub fn new_ref(id: usize) -> AbstractRef {
        Rc::new(RefCell::new(AbstractConcept::new(id)))
    }
    fn new(id: usize) -> AbstractConcept {
        AbstractConcept{
            id,
            definition: None,
            applicand_of: Vec::new(),
            argument_of: Vec::new(),
            normal_form: None,
            reduces_from: Vec::new(),
        }
    }
    fn set_id(&mut self, number: usize) {
        self.id = number;
    }
}

impl Application<ConceptRef> for AbstractConcept {
    fn get_applicand_of(&self) -> Vec<ConceptRef> {
        self.applicand_of.clone()
    }
    fn get_argument_of(&self) -> Vec<ConceptRef> {
        self.argument_of.clone()
    }
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        self.definition.clone()
    }
    fn set_definition(&mut self, applicand: &ConceptRef, argument: &ConceptRef) {
        self.definition = Some((applicand.clone(), argument.clone()));
    }
    fn add_applicand_of(&mut self, applicand: &ConceptRef) {
        self.applicand_of.push(applicand.clone());
    }
    fn add_argument_of(&mut self, argument: &ConceptRef) {
        self.argument_of.push(argument.clone());
    }
}

impl NormalForm<ConceptRef> for AbstractConcept {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_normal_form(&self) -> Option<ConceptRef> {
        self.normal_form.clone()
    }
    fn get_reduces_from(&self) -> Vec<ConceptRef> {
        self.reduces_from.clone()
    }
    fn set_normal_form(&mut self, concept: &ConceptRef) {
        self.normal_form = Some(concept.clone());
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


