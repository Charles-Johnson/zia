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
pub mod string_concept;

use self::abstract_concept::{AbstractConcept, AbstractRef};
use self::string_concept::{StringConcept, StringRef};
use std::fmt;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::left_hand_call::definer3::definer2::delete_normal_form::RemoveNormalForm;
use traits::call::left_hand_call::definer3::definer2::refactor_id::RefactorFrom;
use traits::call::left_hand_call::definer3::delete_definition::RemoveDefinition;
use traits::call::left_hand_call::definer3::labeller::{
    AbstractFactory, SetDefinition, SetNormalForm, StringFactory,
};
use traits::call::GetNormalForm;
use traits::syntax_converter::label::GetNormalFormOf;
use traits::{GetDefinition, Id};
use utils::{ZiaError, ZiaResult};

pub enum ConceptRef {
    Abstract(AbstractRef),
    String(StringRef),
}

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

impl RefactorFrom for ConceptRef {
    fn refactor_from(&mut self, other: &ConceptRef) -> ZiaResult<()> {
        match (self.clone(), other.clone()) {
            (ConceptRef::Abstract(ref mut r), ConceptRef::Abstract(ref o)) => {
				let mut r_borrowed = r.borrow_mut();
				// In order to compare `other` to `self`, `other` needs to be borrowed. If `other == self`,
        		// then borrowing `other` will panic because `other` is already mutably borrowed.
        		if other.check_borrow_err() {
            		return Err(ZiaError::Redundancy(
                		"Concept already has this definition".to_string(),
            		));
				}
				r_borrowed.refactor_from(&o.borrow())
			},
            (ConceptRef::String(ref mut r), ConceptRef::String(ref o)) => {
				let mut r_borrowed = r.borrow_mut();
				// In order to compare `other` to `self`, `other` needs to be borrowed. If `other == self`,
        		// then borrowing `other` will panic because `other` is already mutably borrowed.
        		if other.check_borrow_err() {
            		return Err(ZiaError::Redundancy(
                		"Concept already has this definition".to_string(),
            		));
				}
				r_borrowed.refactor_from(&o.borrow())
			},
			(ConceptRef::Abstract(ref r), ConceptRef::String(ref o)) => {
				*self = ConceptRef::new_string(r.borrow().get_id(), &o.borrow().to_string());
				self.refactor_from(other)
			},
			(ConceptRef::String(ref r), ConceptRef::Abstract(_)) => {
				*self = ConceptRef::new_abstract(r.borrow().get_id());
				self.refactor_from(other)
			},
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

impl GetDefinition<ConceptRef> for ConceptRef {
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_definition(),
            ConceptRef::String(ref c) => c.borrow().get_definition(),
        }
    }
}

impl GetDefinitionOf<ConceptRef> for ConceptRef {
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
}

impl SetDefinition<ConceptRef> for ConceptRef {
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
}

impl RemoveDefinition<ConceptRef> for ConceptRef {
    fn remove_definition(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_definition(),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_definition(),
        }
    }
    fn remove_lefthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_lefthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_lefthand_of(definition),
        }
    }
    fn remove_righthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_righthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_righthand_of(definition),
        }
    }
}

impl Id for ConceptRef {
    fn get_id(&self) -> usize {
        match *self {
            ConceptRef::Abstract(ref r) => r.borrow().get_id(),
            ConceptRef::String(ref r) => r.borrow().get_id(),
        }
    }
}

impl GetNormalForm<ConceptRef> for ConceptRef {
    fn get_normal_form(&self) -> ZiaResult<Option<ConceptRef>> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_normal_form(),
            ConceptRef::String(ref c) => c.borrow().get_normal_form(),
        }
    }
}

impl GetNormalFormOf<ConceptRef> for ConceptRef {
    fn get_normal_form_of(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_normal_form_of(),
            ConceptRef::String(ref c) => c.borrow().get_normal_form_of(),
        }
    }
}

impl SetNormalForm<ConceptRef> for ConceptRef {
    fn set_normal_form(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().set_normal_form(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().set_normal_form(concept),
        }
    }
    fn add_normal_form_of(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_normal_form_of(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_normal_form_of(concept),
        }
    }
}

impl RemoveNormalForm<ConceptRef> for ConceptRef {
    fn remove_normal_form(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_normal_form(),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_normal_form(),
        };
    }
    fn remove_normal_form_of(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_normal_form_of(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_normal_form_of(concept),
        };
    }
}

impl PartialEq for ConceptRef {
    fn eq(&self, other: &ConceptRef) -> bool {
        self.get_id() == other.get_id()
    }
}

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
