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

use reading::{FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction};
use writing::{RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};

pub struct AbstractConcept {
    definition: Option<(usize, usize)>,
    lefthand_of: Vec<usize>,
    righthand_of: Vec<usize>,
    reduces_to: Option<usize>,
    reduces_from: Vec<usize>,
}

impl Default for AbstractConcept {
    fn default() -> AbstractConcept {
        AbstractConcept {
            definition: None,
            lefthand_of: Vec::new(),
            righthand_of: Vec::new(),
            reduces_to: None,
            reduces_from: Vec::new(),
        }
    }
}

impl GetDefinitionOf for AbstractConcept {
    fn get_lefthand_of(&self) -> Vec<usize> {
        self.lefthand_of.clone()
    }
    fn get_righthand_of(&self) -> Vec<usize> {
        self.righthand_of.clone()
    }
}

impl GetDefinition for AbstractConcept {
    fn get_definition(&self) -> Option<(usize, usize)> {
        self.definition
    }
}

impl SetDefinition for AbstractConcept {
    fn set_definition(&mut self, lefthand: usize, righthand: usize) {
        self.definition = Some((lefthand, righthand));
    }
    fn add_as_lefthand_of(&mut self, lefthand: usize) {
        self.lefthand_of.push(lefthand);
    }
    fn add_as_righthand_of(&mut self, righthand: usize) {
        self.righthand_of.push(righthand);
    }
}

impl RemoveDefinition for AbstractConcept {
    fn remove_definition(&mut self) {
        self.definition = None
    }
    fn remove_as_lefthand_of(&mut self, definition: usize) {
        if let Some(pos) = self.lefthand_of.iter().position(|x| *x == definition) {
            self.lefthand_of.remove(pos);
        } else {
            panic!(
                "Concept does not exist in lefthand_of concept number {}",
                definition
            );
        }
    }
    fn remove_as_righthand_of(&mut self, definition: usize) {
        if let Some(pos) = self.righthand_of.iter().position(|x| *x == definition) {
            self.righthand_of.remove(pos);
        } else {
            panic!(
                "Concept number does not exist in righthand_of concept number {}",
                definition
            );
        }
    }
}

impl GetReduction for AbstractConcept {
    fn get_reduction(&self) -> Option<usize> {
        self.reduces_to
    }
}

impl FindWhatReducesToIt for AbstractConcept {
    fn find_what_reduces_to_it(&self) -> Vec<usize> {
        self.reduces_from.clone()
    }
}

impl SetReduction for AbstractConcept {
    fn make_reduce_to(&mut self, concept: usize) {
        self.reduces_to = Some(concept);
    }
    fn make_reduce_from(&mut self, concept: usize) {
        self.reduces_from.push(concept);
    }
}

impl RemoveReduction for AbstractConcept {
    fn make_reduce_to_none(&mut self) {
        self.reduces_to = None;
    }
    fn no_longer_reduces_from(&mut self, concept: usize) {
        if let Some(pos) = self.reduces_from.iter().position(|x| *x == concept) {
            self.reduces_from.remove(pos);
        } else {
            panic!(
                "Concept number does not think it reduces from concept number {}",
                concept
            );
        }
    }
}
