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
use std::fmt;
use std::ops::Add;
use traits::call::label_getter::FindDefinition;
use traits::call::left_hand_call::definer::Pair;
use traits::call::{MaybeConcept, MightExpand};
use traits::SyntaxFactory;
use utils::ZiaResult;

pub enum AbstractSyntaxTree {
    Atom(SyntaxToMaybeConcept),
    Expression(Expression),
}

pub struct SyntaxToMaybeConcept {
    syntax: String,
    concept: Option<ConceptRef>,
}

pub struct Expression {
    syntax_to_maybe_concept: SyntaxToMaybeConcept,
    lefthand: Box<AbstractSyntaxTree>,
    righthand: Box<AbstractSyntaxTree>,
}

impl Expression {
    fn get_lefthand(&self) -> AbstractSyntaxTree {
        let borrowed_left: &AbstractSyntaxTree = self.lefthand.borrow();
        borrowed_left.clone()
    }
    fn get_righthand(&self) -> AbstractSyntaxTree {
        let borrowed_right: &AbstractSyntaxTree = self.righthand.borrow();
        borrowed_right.clone()
    }
}

impl SyntaxFactory<ConceptRef> for AbstractSyntaxTree {
    fn new(s: &str, concept: Option<ConceptRef>) -> AbstractSyntaxTree {
        AbstractSyntaxTree::Atom(SyntaxToMaybeConcept::new(s, concept))
    }
}

impl SyntaxFactory<ConceptRef> for SyntaxToMaybeConcept {
    fn new(s: &str, concept: Option<ConceptRef>) -> SyntaxToMaybeConcept {
        SyntaxToMaybeConcept {
            syntax: s.to_string(),
            concept,
        }
    }
}

impl Pair<AbstractSyntaxTree> for AbstractSyntaxTree {
    fn from_pair(
        syntax: &str,
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> ZiaResult<AbstractSyntaxTree> {
        Ok(AbstractSyntaxTree::Expression(try!(Expression::from_pair(
            syntax, lefthand, righthand
        ))))
    }
}

impl Pair<AbstractSyntaxTree> for Expression {
    fn from_pair(
        syntax: &str,
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> ZiaResult<Expression> {
        let mut concept: Option<ConceptRef> = None;
        if let Some(rc) = righthand.get_concept() {
            if let Some(def) = try!(lefthand.find_definition(&rc)) {
                concept = Some(def.clone());
            }
        }
        Ok(Expression {
            syntax_to_maybe_concept: SyntaxToMaybeConcept::new(syntax, concept),
            lefthand: Box::new(lefthand.clone()),
            righthand: Box::new(righthand.clone()),
        })
    }
}

impl fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                AbstractSyntaxTree::Atom(ref a) => a.to_string(),
                AbstractSyntaxTree::Expression(ref e) => e.to_string(),
            }
        )
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.syntax_to_maybe_concept.to_string(),)
    }
}

impl fmt::Display for SyntaxToMaybeConcept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.syntax.clone(),)
    }
}

impl MaybeConcept<ConceptRef> for AbstractSyntaxTree {
    fn get_concept(&self) -> Option<ConceptRef> {
        match *self {
            AbstractSyntaxTree::Atom(ref a) => a.get_concept(),
            AbstractSyntaxTree::Expression(ref e) => e.get_concept(),
        }
    }
}

impl MaybeConcept<ConceptRef> for Expression {
    fn get_concept(&self) -> Option<ConceptRef> {
        self.syntax_to_maybe_concept.get_concept()
    }
}

impl MaybeConcept<ConceptRef> for SyntaxToMaybeConcept {
    fn get_concept(&self) -> Option<ConceptRef> {
        self.concept.clone()
    }
}

impl MightExpand for AbstractSyntaxTree {
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree, AbstractSyntaxTree)> {
        match *self {
            AbstractSyntaxTree::Atom(_) => None,
            AbstractSyntaxTree::Expression(ref e) => Some((e.get_lefthand(), e.get_righthand())),
        }
    }
}

impl PartialEq for AbstractSyntaxTree {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Clone for AbstractSyntaxTree {
    fn clone(&self) -> AbstractSyntaxTree {
        match *self {
            AbstractSyntaxTree::Atom(ref a) => AbstractSyntaxTree::Atom(a.clone()),
            AbstractSyntaxTree::Expression(ref e) => AbstractSyntaxTree::Expression(e.clone()),
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        Expression {
            syntax_to_maybe_concept: self.syntax_to_maybe_concept.clone(),
            lefthand: Box::new(self.get_lefthand()),
            righthand: Box::new(self.get_righthand()),
        }
    }
}

impl Clone for SyntaxToMaybeConcept {
    fn clone(&self) -> SyntaxToMaybeConcept {
        SyntaxToMaybeConcept {
            syntax: self.syntax.clone(),
            concept: self.concept.clone(),
        }
    }
}

impl Add<AbstractSyntaxTree> for AbstractSyntaxTree {
    type Output = ZiaResult<AbstractSyntaxTree>;
    fn add(self, other: AbstractSyntaxTree) -> ZiaResult<AbstractSyntaxTree> {
        let left_string: String;
        match self {
            AbstractSyntaxTree::Expression(ref e) => {
                left_string = "(".to_string() + &e.to_string() + ")"
            }
            AbstractSyntaxTree::Atom(ref a) => left_string = a.to_string(),
        };
        let right_string: String;
        match other {
            AbstractSyntaxTree::Expression(ref e) => {
                right_string = "(".to_string() + &e.to_string() + ")"
            }
            AbstractSyntaxTree::Atom(ref a) => right_string = a.to_string(),
        };
        AbstractSyntaxTree::from_pair(&(left_string + " " + &right_string), &self, &other)
    }
}
