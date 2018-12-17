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
mod abstract_concept;
pub mod string_concept;
pub mod traits;

use self::abstract_concept::AbstractConcept;
pub use self::string_concept::StringConcept;
use reading::{
    FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction,
    MaybeString,
};
use {RemoveDefinition, RemoveReduction, SetDefinition, SetReduction,
};

pub enum Concept {
    Abstract(AbstractConcept),
    String(StringConcept),
}

impl GetDefinition for Concept {
    fn get_definition(&self) -> Option<(usize, usize)> {
        match *self {
            Concept::Abstract(ref c) => c.get_definition(),
            Concept::String(ref c) => c.get_definition(),
        }
    }
}

impl GetDefinitionOf for Concept {
    fn get_righthand_of(&self) -> Vec<usize> {
        match *self {
            Concept::Abstract(ref c) => c.get_righthand_of(),
            Concept::String(ref c) => c.get_righthand_of(),
        }
    }
    fn get_lefthand_of(&self) -> Vec<usize> {
        match *self {
            Concept::Abstract(ref c) => c.get_lefthand_of(),
            Concept::String(ref c) => c.get_lefthand_of(),
        }
    }
}

impl SetDefinition for Concept {
    fn set_definition(&mut self, lefthand: usize, righthand: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.set_definition(lefthand, righthand),
            Concept::String(ref mut c) => c.set_definition(lefthand, righthand),
        }
    }
    fn add_as_lefthand_of(&mut self, lefthand: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.add_as_lefthand_of(lefthand),
            Concept::String(ref mut c) => c.add_as_lefthand_of(lefthand),
        }
    }
    fn add_as_righthand_of(&mut self, righthand: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.add_as_righthand_of(righthand),
            Concept::String(ref mut c) => c.add_as_righthand_of(righthand),
        }
    }
}

impl RemoveDefinition for Concept {
    fn remove_definition(&mut self) {
        match *self {
            Concept::Abstract(ref mut c) => c.remove_definition(),
            Concept::String(ref mut c) => c.remove_definition(),
        }
    }
    fn remove_as_lefthand_of(&mut self, definition: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.remove_as_lefthand_of(definition),
            Concept::String(ref mut c) => c.remove_as_lefthand_of(definition),
        }
    }
    fn remove_as_righthand_of(&mut self, definition: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.remove_as_righthand_of(definition),
            Concept::String(ref mut c) => c.remove_as_righthand_of(definition),
        }
    }
}

impl GetReduction for Concept {
    fn get_reduction(&self) -> Option<usize> {
        match *self {
            Concept::Abstract(ref c) => c.get_reduction(),
            Concept::String(ref c) => c.get_reduction(),
        }
    }
}

impl FindWhatReducesToIt for Concept {
    fn find_what_reduces_to_it(&self) -> Vec<usize> {
        match *self {
            Concept::Abstract(ref c) => c.find_what_reduces_to_it(),
            Concept::String(ref c) => c.find_what_reduces_to_it(),
        }
    }
}

impl SetReduction for Concept {
    fn make_reduce_to(&mut self, concept: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.make_reduce_to(concept),
            Concept::String(ref mut c) => c.make_reduce_to(concept),
        }
    }
    fn make_reduce_from(&mut self, concept: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.make_reduce_from(concept),
            Concept::String(ref mut c) => c.make_reduce_from(concept),
        }
    }
}

impl RemoveReduction for Concept {
    fn make_reduce_to_none(&mut self) {
        match *self {
            Concept::Abstract(ref mut c) => c.make_reduce_to_none(),
            Concept::String(ref mut c) => c.make_reduce_to_none(),
        };
    }
    fn no_longer_reduces_from(&mut self, concept: usize) {
        match *self {
            Concept::Abstract(ref mut c) => c.no_longer_reduces_from(concept),
            Concept::String(ref mut c) => c.no_longer_reduces_from(concept),
        };
    }
}

impl From<String> for Concept {
    fn from(string: String) -> Concept {
        Concept::String(string.into())
    }
}

impl Default for Concept {
    fn default() -> Concept {
        Concept::Abstract(AbstractConcept::default())
    }
}

impl MaybeString for Concept {
    fn get_string(&self) -> Option<String> {
        match *self {
            Concept::String(ref s) => s.get_string(),
            _ => None,
        }
    }
}
