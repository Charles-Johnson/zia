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
use concepts::ConceptRef;
use std::borrow::Borrow;
use token::Token;
use traits::{Application, Definition, HasToken, MaybeConcept, MightExpand};
use utils::ZiaResult;

pub struct AbstractSyntaxTree {
    token: Token,
    concept: Option<ConceptRef>,
    expansion: Option<(Box<AbstractSyntaxTree>, Box<AbstractSyntaxTree>)>,
}

impl AbstractSyntaxTree {
    pub fn from_token_and_concept(t: &Token, c: &ConceptRef) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            token: t.clone(),
            concept: Some(c.clone()),
            expansion: None,
        }
    }
    pub fn from_atom(s: &str) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            token: Token::Atom(s.to_string()),
            concept: None,
            expansion: None,
        }
    }
    pub fn from_pair(
        token: Token,
        lefthand: AbstractSyntaxTree,
        righthand: AbstractSyntaxTree,
    ) -> ZiaResult<AbstractSyntaxTree> {
        let mut concept: Option<ConceptRef> = None;
        if let Some(argc) = righthand.get_concept() {
            if let Some(def) = try!(lefthand.find_definition(&argc)) {
                concept = Some(def.clone());
            }
        }
        Ok(AbstractSyntaxTree {
            token,
            concept,
            expansion: Some((Box::new(lefthand), Box::new(righthand))),
        })
    }
    pub fn contains(&self, ast: &AbstractSyntaxTree) -> bool {
        if let Some((ref app, ref arg)) = self.get_expansion() {
            app == ast || arg == ast || app.contains(ast) || arg.contains(ast)
        } else {
            false
        }
    }
}

impl HasToken for AbstractSyntaxTree {
    fn get_token(&self) -> Token {
        self.token.clone()
    }
}

impl MaybeConcept<ConceptRef> for AbstractSyntaxTree {
    fn get_concept(&self) -> Option<ConceptRef> {
        self.concept.clone()
    }
}

impl MightExpand<AbstractSyntaxTree> for AbstractSyntaxTree {
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree, AbstractSyntaxTree)> {
        match self.expansion {
            None => None,
            Some((ref app, ref arg)) => {
                let borrowed_app: &AbstractSyntaxTree = app.borrow();
                let borrowed_arg: &AbstractSyntaxTree = arg.borrow();
                Some((borrowed_app.clone(), borrowed_arg.clone()))
            }
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
    fn get_lefthand_of(&self) -> Vec<ConceptRef> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_lefthand_of(),
        }
    }
    fn get_righthand_of(&self) -> Vec<ConceptRef> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_righthand_of(),
        }
    }
    fn set_definition(&mut self, lefthand: &ConceptRef, righthand: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.set_definition(lefthand, righthand)
        }
    }
    fn add_lefthand_of(&mut self, concept: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.add_lefthand_of(concept)
        }
    }
    fn add_righthand_of(&mut self, concept: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.add_righthand_of(concept)
        }
    }
    fn delete_definition(&mut self) {
        if let Some(mut c) = self.get_concept() {
            c.delete_definition()
        }
    }
    fn delete_lefthand_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.delete_lefthand_of(definition)
        }
    }
    fn delete_righthand_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.delete_righthand_of(definition)
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

impl Clone for AbstractSyntaxTree {
    fn clone(&self) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            token: self.get_token(),
            concept: self.get_concept(),
            expansion: match self.get_expansion() {
                None => None,
                Some((app, arg)) => Some((Box::new(app), Box::new(arg))),
            },
        }
    }
}
