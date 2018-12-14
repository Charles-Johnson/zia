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
use super::traits::{MaybeConcept, SyntaxFactory};
use std::fmt;

pub struct Symbol {
    syntax: String,
    concept: Option<usize>,
}

impl SyntaxFactory for Symbol {
    fn new(s: &str, concept: Option<usize>) -> Symbol {
        Symbol {
            syntax: s.to_string(),
            concept,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.syntax)
    }
}

impl MaybeConcept for Symbol {
    fn get_concept(&self) -> Option<usize> {
        self.concept
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Symbol {
        Symbol {
            syntax: self.syntax.clone(),
            concept: self.concept,
        }
    }
}
