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

use std::fmt;
use traits::{SyntaxFactory, call::MaybeConcept};

pub struct Symbol<T> {
    syntax: String,
    concept: Option<T>,
}

impl<T> SyntaxFactory<T> for Symbol<T> {
    fn new(s: &str, concept: Option<T>) -> Symbol<T> {
        Symbol {
            syntax: s.to_string(),
            concept,
        }
    }
}

impl<T> fmt::Display for Symbol<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.syntax.clone(),)
    }
}

impl<T: Clone> MaybeConcept<T> for Symbol<T> {
    fn get_concept(&self) -> Option<T> {
        self.concept.clone()
    }
}

impl<T: Clone> Clone for Symbol<T> {
    fn clone(&self) -> Symbol<T> {
        Symbol::<T> {
            syntax: self.syntax.clone(),
            concept: self.concept.clone(),
        }
    }
}