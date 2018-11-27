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
use concepts::abstract_concept::AbstractConcept;
use concepts::{ConceptRef, StringRef};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::left_hand_call::definer3::definer2::delete_normal_form::RemoveNormalForm;
use traits::call::left_hand_call::definer3::definer2::refactor_id::RefactorFrom;
use traits::call::left_hand_call::definer3::delete_definition::RemoveDefinition;
use traits::call::left_hand_call::definer3::labeller::{SetDefinition, SetNormalForm};
use traits::call::GetNormalForm;
use traits::syntax_converter::label::GetNormalFormOf;
use traits::{GetDefinition, Id};
use utils::ZiaResult;

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
    pub fn set_id(&mut self, number: usize) {
        self.abstract_concept.set_id(number);
    }
}

impl RefactorFrom<ConceptRef> for StringConcept {
    fn refactor_from(&mut self, other: &ConceptRef) -> ZiaResult<()> {
        self.abstract_concept.refactor_from(other)
    }
}

impl GetDefinitionOf<ConceptRef> for StringConcept {
    fn get_lefthand_of(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_lefthand_of()
    }
    fn get_righthand_of(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_righthand_of()
    }
}

impl GetDefinition<ConceptRef> for StringConcept {
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        self.abstract_concept.get_definition()
    }
}

impl SetDefinition<ConceptRef> for StringConcept {
    fn set_definition(&mut self, lefthand: &ConceptRef, righthand: &ConceptRef) {
        self.abstract_concept.set_definition(lefthand, righthand);
    }
    fn add_lefthand_of(&mut self, lefthand: &ConceptRef) {
        self.abstract_concept.add_lefthand_of(lefthand);
    }
    fn add_righthand_of(&mut self, righthand: &ConceptRef) {
        self.abstract_concept.add_righthand_of(righthand);
    }
}

impl RemoveDefinition<ConceptRef> for StringConcept {
    fn remove_definition(&mut self) {
        self.abstract_concept.remove_definition();
    }
    fn remove_lefthand_of(&mut self, definition: &ConceptRef) {
        self.abstract_concept.remove_lefthand_of(definition)
    }
    fn remove_righthand_of(&mut self, definition: &ConceptRef) {
        self.abstract_concept.remove_righthand_of(definition)
    }
}

impl Id for StringConcept {
    fn get_id(&self) -> usize {
        self.abstract_concept.get_id()
    }
}

impl GetNormalForm<ConceptRef> for StringConcept {
    fn get_normal_form(&self) -> ZiaResult<Option<ConceptRef>> {
        self.abstract_concept.get_normal_form()
    }
}

impl GetNormalFormOf<ConceptRef> for StringConcept {
    fn get_normal_form_of(&self) -> Vec<ConceptRef> {
        self.abstract_concept.get_normal_form_of()
    }
}

impl SetNormalForm<ConceptRef> for StringConcept {
    fn set_normal_form(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        self.abstract_concept.set_normal_form(concept)
    }
    fn add_normal_form_of(&mut self, concept: &ConceptRef) {
        self.abstract_concept.add_normal_form_of(concept);
    }
}

impl RemoveNormalForm<ConceptRef> for StringConcept {
    fn remove_normal_form(&mut self) {
        self.abstract_concept.remove_normal_form();
    }
    fn remove_normal_form_of(&mut self, concept: &ConceptRef) {
        self.abstract_concept.remove_normal_form_of(concept);
    }
}

impl fmt::Display for StringConcept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
