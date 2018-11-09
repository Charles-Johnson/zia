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
use std::fmt;
use std::rc::Rc;
use traits::{Application, Definition, Id, Label, ModifyNormalForm, NormalForm, RefactorFrom};
use utils::{ZiaError, ZiaResult};

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
    pub fn check_borrow_err(&self) -> bool {
        match *self {
            ConceptRef::Abstract(ref r) => r.try_borrow().is_err(),
            ConceptRef::String(ref r) => r.try_borrow().is_err(),
        }
    }
    pub fn insert_definition(&mut self, applicand: &mut ConceptRef, argument: &mut ConceptRef) {
        self.set_definition(applicand, argument);
        applicand.add_applicand_of(self);
        argument.add_argument_of(self);
    }
    pub fn remove_definition(&mut self) {
        match self.get_definition() {
            None => panic!("No definition to remove!"),
            Some((mut app, mut arg)) => {
                app.delete_applicand_of(self);
                arg.delete_argument_of(self);
                self.delete_definition();
            }
        };
    }
}

impl RefactorFrom<ConceptRef> for ConceptRef {
    fn refactor_from(&mut self, other: &ConceptRef) -> ZiaResult<()> {
        match *self {
            ConceptRef::Abstract(ref mut r) => r.borrow_mut().refactor_from(other),
            ConceptRef::String(ref mut r) => r.borrow_mut().refactor_from(other),
        }
    }
}

impl fmt::Display for ConceptRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ConceptRef::String(ref s) => s.borrow().to_string(),
                _ => "".to_string(),
            },
        )
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
    fn delete_definition(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_definition(),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_definition(),
        }
    }
    fn delete_applicand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_applicand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_applicand_of(definition),
        }
    }
    fn delete_argument_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_argument_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_argument_of(definition),
        }
    }
}

impl Definition<ConceptRef> for ConceptRef {}

impl Id for ConceptRef {
	fn get_id(&self) -> usize {
        match *self {
            ConceptRef::Abstract(ref r) => r.borrow().get_id(),
            ConceptRef::String(ref r) => r.borrow().get_id(),
        }
    }
}

impl NormalForm<ConceptRef> for ConceptRef {
    fn get_normal_form(&self) -> ZiaResult<Option<ConceptRef>> {
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
    fn set_normal_form(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
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

impl ModifyNormalForm for ConceptRef {}

impl Label<ConceptRef> for ConceptRef {}

impl PartialEq for ConceptRef {
    fn eq(&self, other: &ConceptRef) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for ConceptRef {}

pub struct StringConcept {
    abstract_concept: AbstractConcept,
    string: String,
}

impl StringConcept {
    pub fn new_ref(id: usize, string: &str) -> StringRef {
        Rc::new(RefCell::new(StringConcept {
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

impl RefactorFrom<ConceptRef> for StringConcept {
    fn refactor_from(&mut self, other: &ConceptRef) -> ZiaResult<()> {
        self.abstract_concept.refactor_from(other)
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
    fn delete_definition(&mut self) {
        self.abstract_concept.delete_definition();
    }
    fn delete_applicand_of(&mut self, definition: &ConceptRef) {
        self.abstract_concept.delete_applicand_of(definition)
    }
    fn delete_argument_of(&mut self, definition: &ConceptRef) {
        self.abstract_concept.delete_argument_of(definition)
    }
}

impl Id for StringConcept {
	fn get_id(&self) -> usize {
        self.abstract_concept.get_id()
    }
}

impl NormalForm<ConceptRef> for StringConcept {
    fn get_normal_form(&self) -> ZiaResult<Option<ConceptRef>> {
        self.abstract_concept.get_normal_form()
    }
    fn get_reduces_from(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_reduces_from()
    }
    fn set_normal_form(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        self.abstract_concept.set_normal_form(concept)
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

impl fmt::Display for StringConcept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

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
        AbstractConcept {
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
        self.applicand_of = other.get_applicand_of();
        self.argument_of = other.get_argument_of();
        self.normal_form = try!(other.get_normal_form());
        self.reduces_from = other.get_reduces_from();
        Ok(())
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
    fn delete_definition(&mut self) {
        self.definition = None
    }
    fn delete_applicand_of(&mut self, definition: &ConceptRef) {
        self.applicand_of.remove_item(definition);
    }
    fn delete_argument_of(&mut self, definition: &ConceptRef) {
        self.argument_of.remove_item(definition);
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
