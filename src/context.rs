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
use concepts::{ConceptRef, StringRef};
use constants::LABEL;
use std::collections::HashMap;
use traits::call::expander::{Expander, TokenHandler};
use traits::call::label_getter::LabelGetter;
use traits::call::left_hand_call::definer3::labeller::{
    AbstractMaker, Definer, Labeller, StringMaker,
};
use traits::call::left_hand_call::definer3::{
    ConceptMaker, ConceptNumber, ConceptTidyer, Definer2, Definer3, Refactor, RefactorId,
    Unlabeller,
};
use traits::call::left_hand_call::{ConceptAdder, LeftHandCall};
use traits::call::reduce::Reduce;
use traits::call::{Call, SyntaxFromConcept};
use traits::syntax_converter::{SyntaxConverter, SyntaxFinder};
use traits::Id;
use utils::ZiaResult;

pub struct Context {
    string_map: HashMap<String, StringRef>,
    concepts: Vec<ConceptRef>,
}

impl Context {
    pub fn new() -> ZiaResult<Context> {
        let mut cont = Context {
            string_map: HashMap::new(),
            concepts: Vec::new(),
        };
        try!(cont.setup());
        Ok(cont)
    }
    pub fn execute(&mut self, command: &str) -> ZiaResult<String> {
        let ast = try!(self.ast_from_expression(command));
        self.call(&ast)
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

impl RefactorId<ConceptRef> for Context {}

impl LabelGetter<ConceptRef> for Context {
    fn get_label_concept(&self) -> ConceptRef {
        self.concepts[LABEL].clone()
    }
}

impl Unlabeller<ConceptRef> for Context {}

impl Refactor<ConceptRef> for Context {}

impl ConceptAdder<ConceptRef> for Context {
    fn add_concept(&mut self, concept: &ConceptRef) {
        self.concepts.push(concept.clone());
        if let ConceptRef::String(ref s) = concept {
            self.add_string(s);
        }
    }
}

impl StringMaker<ConceptRef> for Context {}

impl AbstractMaker<ConceptRef> for Context {}

impl Definer<ConceptRef> for Context {}

impl Labeller<ConceptRef> for Context {}

impl SyntaxFinder<ConceptRef> for Context {
    fn get_string_concept(&self, s: &str) -> Option<ConceptRef> {
        match self.string_map.get(s) {
            None => None,
            Some(sc) => Some(ConceptRef::String(sc.clone())),
        }
    }
}

impl Definer2<ConceptRef, AbstractSyntaxTree> for Context {}

impl Definer3<ConceptRef, AbstractSyntaxTree> for Context {}

impl TokenHandler<ConceptRef> for Context {}

impl Expander<ConceptRef, AbstractSyntaxTree> for Context {}

impl ConceptMaker<ConceptRef, AbstractSyntaxTree> for Context {}

impl LeftHandCall<ConceptRef, AbstractSyntaxTree> for Context {}

impl SyntaxFromConcept<ConceptRef, AbstractSyntaxTree> for Context {}

impl Reduce<ConceptRef, AbstractSyntaxTree> for Context {}

impl SyntaxConverter<ConceptRef, AbstractSyntaxTree> for Context {}

impl Call<ConceptRef, AbstractSyntaxTree> for Context {}

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new().unwrap();
    }
}
