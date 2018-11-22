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
use token::{parse_line, parse_tokens, Token};
use traits::{
    AbstractMaker, Application, ConceptAdder, ConceptMaker, ConceptNumber, ConceptTidyer, Definer,
    Definer2, Definer3, Expander, HasToken, Id, LabelGetter, LabelledAbstractMaker, Labeller,
    LeftHandCall, MatchLeftRight, MaybeConcept, MightExpand, NormalForm, Refactor, RefactorId,
    StringMaker, SyntaxFactory, SyntaxFinder, SyntaxFromConcept, TokenHandler, Unlabeller,
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
    pub fn ast_from_expression(&mut self, s: &str) -> ZiaResult<AbstractSyntaxTree> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::Syntax(
                "Parentheses need to contain an expression".to_string(),
            )),
            1 => self.ast_from_atom(&tokens[0]),
            2 => {
                let parsed_tokens = parse_tokens(&tokens);
                self.ast_from_pair(&parsed_tokens[0], &parsed_tokens[1])
            }
            _ => Err(ZiaError::Syntax(
                "Expression composed of more than 2 tokens has not been implemented yet"
                    .to_string(),
            )),
        }
    }
    fn ast_from_atom(&mut self, s: &str) -> ZiaResult<AbstractSyntaxTree> {
        let concept_if_exists = try!(self.concept_from_label(s));
        Ok(AbstractSyntaxTree::new(s, concept_if_exists))
    }
    fn ast_from_pair(&mut self, left: &Token, right: &Token) -> ZiaResult<AbstractSyntaxTree> {
        let lefthand = try!(self.ast_from_token(left));
        let righthand = try!(self.ast_from_token(right));
        lefthand + righthand
    }
    fn ast_from_token(&mut self, t: &Token) -> ZiaResult<AbstractSyntaxTree> {
        match *t {
            Token::Atom(ref s) => self.ast_from_atom(s),
            Token::Expression(ref s) => self.ast_from_expression(s),
        }
    }
    fn recursively_reduce(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<AbstractSyntaxTree> {
        match try!(self.reduce(ast)) {
            Some(ref a) => self.recursively_reduce(a),
            None => Ok(ast.clone()),
        }
    }
    fn reduce(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<Option<AbstractSyntaxTree>> {
        match ast.get_concept() {
            Some(ref c) => self.reduce_concept(c),
            None => match ast.get_expansion() {
                None => Ok(None),
                Some((left, right)) => AbstractSyntaxTree::match_left_right(
                    try!(self.reduce(&left)),
                    try!(self.reduce(&right)),
                    &left,
                    &right,
                ),
            },
        }
    }
    fn reduce_concept(&mut self, c: &ConceptRef) -> ZiaResult<Option<AbstractSyntaxTree>> {
        match try!(c.get_normal_form()) {
            None => match c.get_definition() {
                Some((mut left, mut right)) => {
                    let left_result = try!(self.reduce_concept(&left));
                    let right_result = try!(self.reduce_concept(&right));
                    AbstractSyntaxTree::match_left_right(
                        left_result,
                        right_result,
                        &try!(self.ast_from_concept(&left)),
                        &try!(self.ast_from_concept(&right)),
                    )
                }
                None => Ok(None),
            },
            Some(n) => Ok(Some(try!(self.ast_from_concept(&n)))),
        }
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

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new().unwrap();
    }
}
