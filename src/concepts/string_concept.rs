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
use traits::{Application, Id, Label, NormalForm, RefactorFrom};
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
