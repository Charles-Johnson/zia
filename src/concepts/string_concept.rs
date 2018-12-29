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
use reading::{ConcreteReader, MaybeString};
use writing::ConcreteWriter;

pub struct StringConcept<T> {
	/// The concrete part of the concept. Records which concepts reduces to it and which concepts it composes.
    concrete_concept: T,
	/// The `String` value associated with this concept.
    string: String,
}

impl<T> ConcreteReader for StringConcept<T> {
    type C = T;
    fn read_concrete(&self) -> &T {
        &self.concrete_concept
    }
}

impl<T> ConcreteWriter for StringConcept<T> {
    type C = T;
    fn write_concrete(&mut self) -> &mut T {
        &mut self.concrete_concept
    }
}

impl<T> From<String> for StringConcept<T>
where
    T: Default,
{
    fn from(string: String) -> StringConcept<T> {
        StringConcept::<T> {
            string,
            concrete_concept: T::default(),
        }
    }
}

impl<T> MaybeString for StringConcept<T> {
	/// Because this is definitely a string concept, this always return `Some(string)`
    fn get_string(&self) -> Option<String> {
        Some(self.string.clone())
    }
}
