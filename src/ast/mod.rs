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
mod expression;
mod symbol;

use self::expression::Expression;
use self::symbol::Symbol;
use reading::{DisplayJoint, MaybeConcept, MightExpand, Pair, SyntaxFactory};
use std::fmt;

pub enum AbstractSyntaxTree {
    Symbol(Symbol),
    Expression(Expression<Symbol, AbstractSyntaxTree>),
}

impl MaybeConcept for AbstractSyntaxTree {
    fn get_concept(&self) -> Option<usize> {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => a.get_concept(),
            AbstractSyntaxTree::Expression(ref e) => e.get_concept(),
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
            AbstractSyntaxTree::Symbol(ref a) => AbstractSyntaxTree::Symbol(a.clone()),
            AbstractSyntaxTree::Expression(ref e) => AbstractSyntaxTree::Expression(e.clone()),
        }
    }
}

impl MightExpand<AbstractSyntaxTree> for AbstractSyntaxTree {
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree, AbstractSyntaxTree)> {
        match *self {
            AbstractSyntaxTree::Symbol(_) => None,
            AbstractSyntaxTree::Expression(ref e) => Some((e.get_lefthand(), e.get_righthand())),
        }
    }
}

impl DisplayJoint for AbstractSyntaxTree {
    fn display_joint(&self) -> String {
        match *self {
            AbstractSyntaxTree::Expression(ref e) => "(".to_string() + &e.to_string() + ")",
            AbstractSyntaxTree::Symbol(ref a) => a.to_string(),
        }
    }
}

impl fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => write!(f, "{}", a.to_string()),
            AbstractSyntaxTree::Expression(ref e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl Pair<AbstractSyntaxTree> for AbstractSyntaxTree {
    fn from_pair(
        syntax: &str,
        concept: Option<usize>,
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> AbstractSyntaxTree {
        AbstractSyntaxTree::Expression(Expression::<Symbol, AbstractSyntaxTree>::from_pair(
            syntax, concept, lefthand, righthand,
        ))
    }
}

impl SyntaxFactory for AbstractSyntaxTree {
    fn new(s: &str, concept: Option<usize>) -> AbstractSyntaxTree {
        AbstractSyntaxTree::Symbol(Symbol::new(s, concept))
    }
}
