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
use concept::ConceptRef;
use std::rc::Rc;
use token::Token;
use traits::{Application, Definition};
use utils::ZiaResult;

pub struct AbstractSyntaxTree {
    token: Token,
    concept: Option<ConceptRef>,
    expansion: Option<(Rc<AbstractSyntaxTree>, Rc<AbstractSyntaxTree>)>,
}

impl AbstractSyntaxTree {
    pub fn from_token_and_concept(t: &Token, c: &ConceptRef) -> Rc<AbstractSyntaxTree> {
        Rc::new(AbstractSyntaxTree {
            token: t.clone(),
            concept: Some(c.clone()),
            expansion: None,
        })
    }
    pub fn from_atom(s: &str) -> Rc<AbstractSyntaxTree> {
        Rc::new(AbstractSyntaxTree {
            token: Token::Atom(s.to_string()),
            concept: None,
            expansion: None,
        })
    }
    pub fn from_monad(
        token: Token,
        applicand: &Rc<AbstractSyntaxTree>,
        argument: &Rc<AbstractSyntaxTree>,
    ) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        let mut concept: Option<ConceptRef> = None;
        if let Some(argc) = argument.get_concept() {
            if let Some(def) = try!(applicand.find_definition(&argc)) {
                concept = Some(def.clone());
            }
        }
        Ok(Rc::new(AbstractSyntaxTree {
            token,
            concept,
            expansion: Some((applicand.clone(), argument.clone())),
        }))
    }
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }
    pub fn get_concept(&self) -> Option<ConceptRef> {
        self.concept.clone()
    }
    pub fn get_expansion(&self) -> Option<(Rc<AbstractSyntaxTree>, Rc<AbstractSyntaxTree>)> {
        self.expansion.clone()
    }
    pub fn contains(&self, ast: &Rc<AbstractSyntaxTree>) -> bool {
        if let Some((ref app, ref arg)) = self.get_expansion() {
            app == ast || arg == ast || app.contains(ast) || arg.contains(ast)
        } else {
            false
        }
    }
}

impl Application<ConceptRef> for AbstractSyntaxTree {
    fn get_definition(&self) -> Option<(ConceptRef, ConceptRef)> {
        match self.get_concept() {
            None => match self.get_expansion() {
                Some((app, arg)) => {
                    if let (Some(appc), Some(argc)) = (app.get_concept(), arg.get_concept()) {
                        Some((appc, argc))
                    } else {
                        None
                    }
                }
                None => None,
            },
            Some(c) => c.get_definition(),
        }
    }
    fn get_applicand_of(&self) -> Vec<ConceptRef> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_applicand_of(),
        }
    }
    fn get_argument_of(&self) -> Vec<ConceptRef> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_argument_of(),
        }
    }
    fn set_definition(&mut self, applicand: &ConceptRef, argument: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.set_definition(applicand, argument)
        }
    }
    fn add_applicand_of(&mut self, concept: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.add_applicand_of(concept)
        }
    }
    fn add_argument_of(&mut self, concept: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.add_argument_of(concept)
        }
    }
    fn delete_definition(&mut self) {
        if let Some(mut c) = self.get_concept() {
            c.delete_definition()
        }
    }
    fn delete_applicand_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.delete_applicand_of(definition)
        }
    }
    fn delete_argument_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.delete_argument_of(definition)
        }
    }
}

impl Definition<ConceptRef> for AbstractSyntaxTree {}

impl PartialEq for AbstractSyntaxTree {
    fn eq(&self, other: &Self) -> bool {
        self.get_token() == other.get_token()
    }
}

impl Eq for AbstractSyntaxTree {}
