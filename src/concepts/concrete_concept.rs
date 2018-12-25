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
use reading::{FindWhatReducesToIt, GetDefinitionOf};
use std::collections::HashSet;
use writing::{MakeReduceFrom, NoLongerReducesFrom, RemoveAsDefinitionOf, SetAsDefinitionOf};

#[derive(Default)]
pub struct ConcreteConcept {
    lefthand_of: HashSet<usize>,
    righthand_of: HashSet<usize>,
    reduces_from: HashSet<usize>,
}

impl GetDefinitionOf for ConcreteConcept {
    fn get_lefthand_of(&self) -> HashSet<usize> {
        self.lefthand_of.clone()
    }
    fn get_righthand_of(&self) -> HashSet<usize> {
        self.righthand_of.clone()
    }
}

impl FindWhatReducesToIt for ConcreteConcept {
    fn find_what_reduces_to_it(&self) -> HashSet<usize> {
        self.reduces_from.clone()
    }
}

impl SetAsDefinitionOf for ConcreteConcept {
    fn add_as_lefthand_of(&mut self, index: usize) {
        self.lefthand_of.insert(index);
    }
    fn add_as_righthand_of(&mut self, index: usize) {
        self.righthand_of.insert(index);
    }
}

impl MakeReduceFrom for ConcreteConcept {
    fn make_reduce_from(&mut self, index: usize) {
        self.reduces_from.insert(index);
    }
}

impl RemoveAsDefinitionOf for ConcreteConcept {
    fn remove_as_lefthand_of(&mut self, index: usize) {
        self.lefthand_of.remove(&index);
    }
    fn remove_as_righthand_of(&mut self, index: usize) {
        self.righthand_of.remove(&index);
    }
}

impl NoLongerReducesFrom for ConcreteConcept {
    fn no_longer_reduces_from(&mut self, index: usize) {
        self.reduces_from.remove(&index);
    }
}
