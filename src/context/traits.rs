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
use std::{cell::RefCell, rc::Rc};

pub trait LabelConcept<T> {
    fn get_label_concept(&self) -> T;
}

pub trait StringAdder<V> {
    fn add_string(&mut self, &Rc<RefCell<V>>, &str);
}

pub trait BlindConceptAdder<T> {
	fn blindly_add_concept(&mut self, &T);
}

pub trait ConceptHandler<T> {
	fn get_concept(&self, usize) -> T;
	fn remove_concept_by_id(&mut self, usize);
}

pub trait StringConcept<T> {
    fn get_string_concept(&self, &str) -> Option<T>;
}

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}
