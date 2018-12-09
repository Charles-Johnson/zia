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
use traits::call::label_getter::{GetDefinitionOf, LabelGetter, MaybeString};
use traits::call::right_hand_call::definer::delete_definition::RemoveDefinition;
use traits::call::right_hand_call::definer::labeller::{
    AbstractFactory, SetDefinition, SetReduction, StringFactory,
};
use traits::call::right_hand_call::definer::refactor::delete_normal_form::RemoveReduction;
use traits::call::{FindWhatReducesToIt, GetReduction};
use traits::{GetDefinition, GetId, SetId};

pub enum ConceptRef {
    Abstract(AbstractRef<ConceptRef>),
    String(StringRef<ConceptRef>),
}

impl SetId for ConceptRef {
    fn set_id(&mut self, number: usize) {
        match *self {
            ConceptRef::Abstract(ref mut r) => r.borrow_mut().set_id(number),
            ConceptRef::String(ref mut r) => r.borrow_mut().set_id(number),
        }
    }
}

pub trait ConvertTo<T> {
	fn convert(&self) -> Option<T>;
}

impl ConvertTo<StringRef<ConceptRef>> for ConceptRef {
	fn convert(&self) -> Option<StringRef<ConceptRef>> {
		match *self {
			ConceptRef::String(ref a) => Some(a.clone()),
			_ => None
		}
	}
}

impl From<StringRef<ConceptRef>> for ConceptRef {
	fn from(sr: StringRef<ConceptRef>) -> ConceptRef {
		ConceptRef::String(sr.clone())
	}
}

pub trait Display {
    fn to_string(&self) -> String;
}

impl<T: LabelGetter> Display for T {
    fn to_string(&self) -> String {
        match self.get_string() {
            Some(s) => "\"".to_string() + &s + "\"",
            None => match self.get_label() {
                Some(l) => l,
                None => match self.get_definition() {
                    Some((left, right)) => {
                        let mut left_string = left.to_string();
                        if left_string.contains(' ') {
                            left_string = "(".to_string() + &left_string;
                        }
                        let mut right_string = right.to_string();
                        if right_string.contains(' ') {
                            right_string += ")";
                        }
                        left_string + " " + &right_string
                    }
                    None => panic!("Unlabelled concept with no definition!"),
                },
            },
        }
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
    fn add_as_lefthand_of(&mut self, lefthand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_as_lefthand_of(lefthand),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_as_lefthand_of(lefthand),
        }
    }
    fn add_as_righthand_of(&mut self, righthand: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().add_as_righthand_of(righthand),
            ConceptRef::String(ref mut c) => c.borrow_mut().add_as_righthand_of(righthand),
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
    fn remove_as_lefthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_as_lefthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_as_lefthand_of(definition),
        }
    }
    fn remove_as_righthand_of(&mut self, definition: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().remove_as_righthand_of(definition),
            ConceptRef::String(ref mut c) => c.borrow_mut().remove_as_righthand_of(definition),
        }
    }
}

impl GetId for ConceptRef {
    fn get_id(&self) -> usize {
        match *self {
            ConceptRef::Abstract(ref r) => r.borrow().get_id(),
            ConceptRef::String(ref r) => r.borrow().get_id(),
        }
    }
}

impl GetReduction<ConceptRef> for ConceptRef {
    fn get_reduction(&self) -> Option<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().get_reduction(),
            ConceptRef::String(ref c) => c.borrow().get_reduction(),
        }
    }
}

impl FindWhatReducesToIt<ConceptRef> for ConceptRef {
    fn find_what_reduces_to_it(&self) -> Vec<ConceptRef> {
        match *self {
            ConceptRef::Abstract(ref c) => c.borrow().find_what_reduces_to_it(),
            ConceptRef::String(ref c) => c.borrow().find_what_reduces_to_it(),
        }
    }
}

impl SetReduction<ConceptRef> for ConceptRef {
    fn make_reduce_to(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().make_reduce_to(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().make_reduce_to(concept),
        }
    }
    fn make_reduce_from(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().make_reduce_from(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().make_reduce_from(concept),
        }
    }
}

impl RemoveReduction<ConceptRef> for ConceptRef {
    fn make_reduce_to_none(&mut self) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().make_reduce_to_none(),
            ConceptRef::String(ref mut c) => c.borrow_mut().make_reduce_to_none(),
        };
    }
    fn no_longer_reduces_from(&mut self, concept: &ConceptRef) {
        match *self {
            ConceptRef::Abstract(ref mut c) => c.borrow_mut().no_longer_reduces_from(concept),
            ConceptRef::String(ref mut c) => c.borrow_mut().no_longer_reduces_from(concept),
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

impl MaybeString for ConceptRef {
    fn get_string(&self) -> Option<String> {
        match *self {
            ConceptRef::String(ref s) => s.borrow().get_string(),
            _ => None,
        }
    }
}
