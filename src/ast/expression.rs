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
use super::symbol::Symbol;
use super::AbstractSyntaxTree;
use std::borrow::Borrow;
use std::fmt;
use traits::call::right_hand_call::definer::Pair;
use traits::call::MaybeConcept;
use traits::SyntaxFactory;

pub struct Expression {
    symbol: Symbol,
    lefthand: Box<AbstractSyntaxTree>,
    righthand: Box<AbstractSyntaxTree>,
}

impl Expression {
    pub fn get_lefthand(&self) -> AbstractSyntaxTree {
        let borrowed_left: &AbstractSyntaxTree = self.lefthand.borrow();
        borrowed_left.clone()
    }
    pub fn get_righthand(&self) -> AbstractSyntaxTree {
        let borrowed_right: &AbstractSyntaxTree = self.righthand.borrow();
        borrowed_right.clone()
    }
}

impl Pair<ConceptRef, AbstractSyntaxTree> for Expression {
    fn from_pair(
        syntax: &str,
		concept: Option<ConceptRef>,
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> Expression {
        Expression {
            symbol: Symbol::new(syntax, concept),
            lefthand: Box::new(lefthand.clone()),
            righthand: Box::new(righthand.clone()),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol.to_string(),)
    }
}

impl MaybeConcept<ConceptRef> for Expression {
    fn get_concept(&self) -> Option<ConceptRef> {
        self.symbol.get_concept()
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        Expression {
            symbol: self.symbol.clone(),
            lefthand: Box::new(self.get_lefthand()),
            righthand: Box::new(self.get_righthand()),
        }
    }
}
