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
use ast::AbstractSyntaxTree;
use concepts::string_concept::StringRef;
use concepts::ConceptRef;
use constants::LABEL;
use std::collections::HashMap;
use traits::call::right_hand_call::definer::concept_maker::ConceptMaker;
use traits::call::right_hand_call::definer::labeller::{ConceptAdder, LabelConcept, Labeller};
use traits::call::right_hand_call::definer::refactor::refactor_id::ConceptTidyer;
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::call::Call;
use traits::syntax_converter::{SyntaxConverter, SyntaxFinder};
use traits::Id;

#[derive(Default)]
pub struct Context {
    string_map: HashMap<String, StringRef>,
    concepts: Vec<ConceptRef>,
}

impl Context {
    pub fn new() -> Context {
        let mut cont = Context::default();
        cont.setup().unwrap();
        cont
    }
    pub fn execute(&mut self, command: &str) -> String {
        let ast = match self.ast_from_expression(command) {
            Ok(a) => a,
            Err(e) => return e.to_string(),
        };
        match self.call(&ast) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
    fn add_string(&mut self, string_ref: &StringRef) {
        self.string_map
            .insert(string_ref.borrow().to_string(), string_ref.clone());
    }
}

impl ConceptTidyer<ConceptRef> for Context {
    fn remove_concept(&mut self, concept: &ConceptRef) {
        self.concepts.remove(concept.get_id());
    }
    fn correct_id(&mut self, id: usize) {
        self.concepts[id].set_id(id);
    }
}

impl ConceptNumber for Context {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

impl ConceptAdder<ConceptRef> for Context {
    fn add_concept(&mut self, concept: &ConceptRef) {
        self.concepts.push(concept.clone());
        if let ConceptRef::String(ref s) = concept {
            self.add_string(s);
        }
    }
}

impl SyntaxFinder<ConceptRef> for Context {
    fn get_string_concept(&self, s: &str) -> Option<ConceptRef> {
        match self.string_map.get(s) {
            None => None,
            Some(sc) => Some(ConceptRef::String(sc.clone())),
        }
    }
}

impl LabelConcept<ConceptRef> for Context {
    fn get_label_concept(&self) -> ConceptRef {
        self.concepts[LABEL].clone()
    }
}

impl ConceptMaker<ConceptRef, AbstractSyntaxTree> for Context {}

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new();
    }
}
