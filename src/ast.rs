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
use std::ops::Add;
use token::Token;
use traits::call::label_getter::{FindDefinition, GetDefinitionOf};
use traits::call::left_hand_call::definer3::delete_definition::RemoveDefinition;
use traits::call::left_hand_call::definer3::labeller::SetDefinition;
use traits::call::left_hand_call::definer3::Pair;
use traits::call::{HasToken, MaybeConcept, MightExpand};
use traits::SyntaxFactory;
use utils::ZiaResult;

pub struct AbstractSyntaxTree {
    token: Token,
    concept: Option<ConceptRef>,
    expansion: Option<(Box<AbstractSyntaxTree>, Box<AbstractSyntaxTree>)>,
}

impl SyntaxFactory<ConceptRef> for AbstractSyntaxTree {
    fn new(s: &str, concept: Option<ConceptRef>) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            token: Token::Atom(s.to_string()),
            concept,
            expansion: None,
        }
    }
}

impl Pair for AbstractSyntaxTree {
    fn from_pair(
        token: Token,
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> ZiaResult<AbstractSyntaxTree> {
        let mut concept: Option<ConceptRef> = None;
        if let Some(rc) = righthand.get_concept() {
            if let Some(def) = try!(lefthand.find_definition(&rc)) {
                concept = Some(def.clone());
            }
        }
        Ok(AbstractSyntaxTree {
            token,
            concept,
            expansion: Some((Box::new(lefthand.clone()), Box::new(righthand.clone()))),
        })
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

impl MightExpand for AbstractSyntaxTree {
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree, AbstractSyntaxTree)> {
        match self.expansion {
            None => None,
            Some((ref left, ref right)) => {
                let borrowed_left: &AbstractSyntaxTree = left.borrow();
                let borrowed_right: &AbstractSyntaxTree = right.borrow();
                Some((borrowed_left.clone(), borrowed_right.clone()))
            }
        }
    }
}

impl GetDefinitionOf<ConceptRef> for AbstractSyntaxTree {
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
}

impl SetDefinition<ConceptRef> for AbstractSyntaxTree {
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
}

impl RemoveDefinition<ConceptRef> for AbstractSyntaxTree {
    fn remove_definition(&mut self) {
        if let Some(mut c) = self.get_concept() {
            c.remove_definition()
        }
    }
    fn remove_lefthand_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.remove_lefthand_of(definition)
        }
    }
    fn remove_righthand_of(&mut self, definition: &ConceptRef) {
        if let Some(mut c) = self.get_concept() {
            c.remove_righthand_of(definition)
        }
    }
}

impl PartialEq for AbstractSyntaxTree {
    fn eq(&self, other: &Self) -> bool {
        self.get_token() == other.get_token()
    }
}

impl Clone for AbstractSyntaxTree {
    fn clone(&self) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            token: self.get_token(),
            concept: self.get_concept(),
            expansion: match self.get_expansion() {
                None => None,
                Some((left, right)) => Some((Box::new(left), Box::new(right))),
            },
        }
    }
}

impl Add<AbstractSyntaxTree> for AbstractSyntaxTree {
    type Output = ZiaResult<AbstractSyntaxTree>;
    fn add(self, other: AbstractSyntaxTree) -> ZiaResult<AbstractSyntaxTree> {
        AbstractSyntaxTree::from_pair(self.get_token() + other.get_token(), &self, &other)
    }
}
