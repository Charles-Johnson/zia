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

pub trait StringAdder {
    fn add_string(&mut self, usize, &str);
}

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, T) -> usize;
}

pub trait ConceptReader<T> {
    fn read_concept(&self, usize) -> &T;
}

pub trait ConceptWriter<T> {
    fn write_concept(&mut self, usize) -> &mut T;
}
pub trait ConceptRemover {
    fn remove_concept(&mut self, usize);
}

pub trait StringConcept {
    fn get_string_concept(&self, &str) -> Option<usize>;
}
