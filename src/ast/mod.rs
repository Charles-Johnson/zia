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
mod syntax;

use self::expression::Expression;
use self::syntax::Syntax;
use reading::{DisplayJoint, MaybeConcept, MightExpand, Pair};
use std::fmt;

/// Syntax is represented as a full binary tree and linked to concepts where possible.
pub enum AbstractSyntaxTree {
	/// Leaf of the tree which represents a string containing no spaces. 
    Symbol(Syntax),
	/// Branch of the tree which represents a string with containing one space outside of any parethenses.
    Expression(Expression<Syntax, AbstractSyntaxTree>),
}

impl MaybeConcept for AbstractSyntaxTree {
	/// Gets the possible concept from the inside type of the variant.
    fn get_concept(&self) -> Option<usize> {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => a.get_concept(),
            AbstractSyntaxTree::Expression(ref e) => e.get_concept(),
        }
    }
}

impl PartialEq for AbstractSyntaxTree {
	/// `AbstractSyntaxTree`s are equal if the syntax they represent is the same.
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
	/// An expression does have an expansion while a symbol does not.
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree, AbstractSyntaxTree)> {
        match *self {
            AbstractSyntaxTree::Symbol(_) => None,
            AbstractSyntaxTree::Expression(ref e) => Some((e.get_lefthand(), e.get_righthand())),
        }
    }
}

impl DisplayJoint for AbstractSyntaxTree {
	/// An expression's syntax is encapsulated in parentheses when joined with other syntax whereas a symbol's syntax is not. 
    fn display_joint(&self) -> String {
        match *self {
            AbstractSyntaxTree::Expression(ref e) => "(".to_string() + &e.to_string() + ")",
            AbstractSyntaxTree::Symbol(ref a) => a.to_string(),
        }
    }
}

impl fmt::Display for AbstractSyntaxTree {
	/// Displays the same as the inside of an `AbstractSyntaxTree` variant.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => write!(f, "{}", a.to_string()),
            AbstractSyntaxTree::Expression(ref e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl Pair<AbstractSyntaxTree> for AbstractSyntaxTree {
    /// Combines a pair of syntax trees and details of an overal syntax into an `AbstractSyntaxTree`
    fn from_pair(
        syntax: (String, Option<usize>),
        lefthand: &AbstractSyntaxTree,
        righthand: &AbstractSyntaxTree,
    ) -> AbstractSyntaxTree {
        AbstractSyntaxTree::Expression(Expression::<Syntax, AbstractSyntaxTree>::from_pair(
            syntax, lefthand, righthand,
        ))
    }
}

impl From<(String, Option<usize>)> for AbstractSyntaxTree {
	/// Constructs a `Symbol` variant from the syntax string and a possible associated concept.  
    fn from(syntax: (String, Option<usize>)) -> AbstractSyntaxTree {
        AbstractSyntaxTree::Symbol(Syntax::from(syntax))
    }
}
