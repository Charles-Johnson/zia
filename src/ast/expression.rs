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
use reading::{MaybeConcept, Pair};
use std::{borrow::Borrow, fmt};

/// Represents syntax separated by a space outside of any parentheses.
pub struct Expression<S, T> {
    /// Overall syntax.
    syntax: S,
    /// Syntax on the lefthand side of the space.
    lefthand: Box<T>,
	/// Syntax on the righthand side of the space.
    righthand: Box<T>,
}

impl<S, T: Clone> Expression<S, T> {
    /// Returns a clone from inside of the `Box` of the `lefthand` field. 
    pub fn get_lefthand(&self) -> T {
        let borrowed_left: &T = self.lefthand.borrow();
        borrowed_left.clone()
    }
    /// Returns a clone from inside of the `Box` of the `righthand` field. 
    pub fn get_righthand(&self) -> T {
        let borrowed_right: &T = self.righthand.borrow();
        borrowed_right.clone()
    }
}

impl<S: From<(String, Option<usize>)>, T: Clone> Pair<T> for Expression<S, T> {
	/// Combines two syntax trees and details of an overall syntax into an expression.
    fn from_pair(
        syntax: (String, Option<usize>),
        lefthand: &T,
        righthand: &T,
    ) -> Expression<S, T> {
        Expression::<S, T> {
            syntax: S::from(syntax),
            lefthand: Box::new(lefthand.clone()),
            righthand: Box::new(righthand.clone()),
        }
    }
}

impl<S: fmt::Display, T> fmt::Display for Expression<S, T> {
    /// Displays the same as the overall syntax.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.syntax.to_string())
    }
}

impl<S: MaybeConcept, T> MaybeConcept for Expression<S, T> {
	/// Get the possible concept corresponding to the overall syntax.
    fn get_concept(&self) -> Option<usize> {
        self.syntax.get_concept()
    }
}

impl<S: Clone, T: Clone> Clone for Expression<S, T> {
	/// Clones the syntax field and Boxes a clone of the inside of the lefthand and righthand fields to initialise a new Expression.
    fn clone(&self) -> Expression<S, T> {
        Expression::<S, T> {
            syntax: self.syntax.clone(),
            lefthand: Box::new(self.get_lefthand()),
            righthand: Box::new(self.get_righthand()),
        }
    }
}
