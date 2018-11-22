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
use constants::{DEFINE, LABEL, REDUCTION};
use std::collections::HashMap;
use traits::{
    AbstractMaker, ConceptAdder, ConceptMaker, ConceptNumber, ConceptTidyer, Definer, Definer2,
    Definer3, Expander, HasToken, Id, LabelGetter, LabelledAbstractMaker, Labeller, LeftHandCall,
    MaybeConcept, MightExpand, Reduce, Refactor, RefactorId, StringMaker, SyntaxConverter,
    SyntaxFinder, SyntaxFromConcept, TokenHandler, Unlabeller,
};
use utils::{ZiaError, ZiaResult};

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
    pub fn call(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref left, ref right)) => if let Some(c) = right.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(self.recursively_reduce(left)).get_token().as_string()),
                    DEFINE => Ok(try!(self.expand_ast_token(left)).as_string()),
                    _ => self.call_as_lefthand(left, right),
                }
            } else {
                self.call_as_lefthand(left, right)
            },
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
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

impl LabelledAbstractMaker<ConceptRef> for Context {}

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

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new().unwrap();
    }
}
