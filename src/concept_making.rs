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

pub trait StringMaker<T>
where
    T: From<String>,
    Self: ConceptAdder<T> + StringAdder,
{
    fn new_string(&mut self, string: &str) -> usize {
        let string_concept = string.to_string().into();
        let index = self.add_concept(string_concept);
        self.add_string(index, string);
        index
    }
}

impl<S, T> StringMaker<T> for S
where
    T: From<String>,
    S: ConceptAdder<T> + StringAdder,
{
}

pub trait AbstractMaker<T>
where
    T: Default,
    Self: ConceptAdder<T>,
{
    fn new_abstract(&mut self) -> usize {
        let concept = T::default();
        self.add_concept(concept)
    }
}

impl<S, T> AbstractMaker<T> for S
where
    T: Default,
    S: ConceptAdder<T>,
{
}

pub trait StringAdder {
    fn add_string(&mut self, usize, &str);
}

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, T) -> usize;
}
