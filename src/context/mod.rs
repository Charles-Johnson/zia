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

pub mod traits;

use self::traits::{
    ConceptAdder, ConceptReader, ConceptRemover, ConceptWriter, StringAdder,
	StringConcept,
};
use std::collections::HashMap;

pub struct Context<T> {
    string_map: HashMap<String, usize>,
    concepts: Vec<Option<T>>,
	gaps: Vec<usize>,
}

impl<T> Default for Context<T> {
    fn default() -> Context<T> {
        Context::<T> {
            string_map: HashMap::new(),
            concepts: Vec::new(),
			gaps: Vec::new(),
        }
    }
}

impl<T> StringAdder for Context<T> {
    fn add_string(&mut self, string_id: usize, string: &str) {
        self.string_map.insert(string.to_string(), string_id);
    }
}

impl<T> ConceptWriter<T> for Context<T> {
    fn write_concept(&mut self, id: usize) -> &mut T {
        match self.concepts[id] {
			Some(ref mut c) => c,
			None => panic!("No concept with id = {}", id),
		}
    }
}

impl<T> ConceptReader<T> for Context<T> {
    fn read_concept(&self, id: usize) -> &T {
         match self.concepts[id] {
			Some(ref c) => c,
			None => panic!("No concept with id = {}", id),
		}
    }
}

impl<T> ConceptRemover for Context<T> {
    fn remove_concept(&mut self, id: usize) {
		self.concepts[id] = None;
        self.gaps.push(id);
    }
}

impl<T> ConceptAdder<T> for Context<T> {
    fn add_concept(&mut self, concept: T) -> usize {
		match self.gaps.pop() {
        	None => {
				let index = self.concepts.len();
				self.concepts.push(Some(concept));
				index
			},
			Some(index) => {
				self.concepts[index] = Some(concept);
				index
			},
		}
    }
}

impl<T> StringConcept for Context<T> {
    fn get_string_concept(&self, s: &str) -> Option<usize> {
        match self.string_map.get(s) {
            None => None,
            Some(sr) => Some(*sr),
        }
    }
}
