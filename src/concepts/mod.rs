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
mod abstract_concept;
mod string_concept;

pub use self::abstract_concept::AbstractConcept;
pub use self::string_concept::StringConcept;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use traits::{
    AbstractFactory, Application, Definition, DefinitionModifier, Id, Label, NormalForm,
    NormalFormModifier, RefactorFrom, StringFactory,
};
use utils::ZiaResult;

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
}

impl DefinitionModifier for ConceptRef {}

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
    fn get_righthand_of(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_righthand_of(),
            ConceptRef::String(ref c) => c.borrow().get_righthand_of(),
        }
    }
    fn get_lefthand_of(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_lefthand_of(),
            ConceptRef::String(ref c) => c.borrow().get_lefthand_of(),
        }
    }
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_definition(),
            ConceptRef::String(ref c) => c.borrow().get_definition(),
        }
    }
    fn set_definition(&mut self, lefthand: &ConceptRef, righthand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().set_definition(lefthand, righthand),
            ConceptRef::String(ref mut c) => c.borrow_mut().set_definition(lefthand, righthand),
        }
    }
    fn add_lefthand_of(&mut self, lefthand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_lefthand_of(lefthand),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_lefthand_of(lefthand),
        }
    }
    fn add_righthand_of(&mut self, righthand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_righthand_of(righthand),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_righthand_of(righthand),
        }
    }
    fn delete_definition(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_definition(),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_definition(),
        }
    }
    fn delete_lefthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_lefthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_lefthand_of(definition),
        }
    }
    fn delete_righthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().delete_righthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().delete_righthand_of(definition),
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

impl NormalFormModifier for ConceptRef {}

impl Label<ConceptRef> for ConceptRef {}

impl PartialEq for ConceptRef {
    fn eq(&self, other: &ConceptRef) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for ConceptRef {}

impl StringFactory for ConceptRef {
    fn new_string(id: usize, string: &str) -> ConceptRef {
        ConceptRef::String(StringConcept::new_ref(id, string))
    }
}

impl AbstractFactory for ConceptRef {
    fn new_abstract(id: usize) -> ConceptRef {
        ConceptRef::Abstract(AbstractConcept::new_ref(id))
    }
}
