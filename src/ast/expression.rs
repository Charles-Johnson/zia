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
use super::symbol::Symbol;
use concept_reading::{MaybeConcept, Pair, SyntaxFactory};
use std::{borrow::Borrow, fmt};

pub struct Expression<T> {
    symbol: Symbol,
    lefthand: Box<T>,
    righthand: Box<T>,
}

impl<T: Clone> Expression<T> {
    pub fn get_lefthand(&self) -> T {
        let borrowed_left: &T = self.lefthand.borrow();
        borrowed_left.clone()
    }
    pub fn get_righthand(&self) -> T {
        let borrowed_right: &T = self.righthand.borrow();
        borrowed_right.clone()
    }
}

impl<T: Clone> Pair<T> for Expression<T> {
    fn from_pair(
        syntax: &str,
        concept: Option<usize>,
        lefthand: &T,
        righthand: &T,
    ) -> Expression<T> {
        Expression::<T> {
            symbol: Symbol::new(syntax, concept),
            lefthand: Box::new(lefthand.clone()),
            righthand: Box::new(righthand.clone()),
        }
    }
}

impl<T> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol.to_string())
    }
}

impl<T> MaybeConcept for Expression<T> {
    fn get_concept(&self) -> Option<usize> {
        self.symbol.get_concept()
    }
}

impl<T: Clone> Clone for Expression<T> {
    fn clone(&self) -> Expression<T> {
        Expression::<T> {
            symbol: self.symbol.clone(),
            lefthand: Box::new(self.get_lefthand()),
            righthand: Box::new(self.get_righthand()),
        }
    }
}
