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
use reading::MaybeConcept;
use std::fmt;

/// Groups a string with a possible concept number.
pub struct Syntax {
    string: String,
    concept: Option<usize>,
}

impl From<(String, Option<usize>)> for Syntax {
    fn from(syntax: (String, Option<usize>)) -> Syntax {
        Syntax {
            string: syntax.0,
            concept: syntax.1,
        }
    }
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.string)
    }
}

impl MaybeConcept for Syntax {
    fn get_concept(&self) -> Option<usize> {
        self.concept
    }
}

impl Clone for Syntax {
    fn clone(&self) -> Syntax {
        Syntax {
            string: self.string.clone(),
            concept: self.concept,
        }
    }
}
