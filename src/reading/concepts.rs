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
use std::collections::HashSet;

pub trait GetDefinition {
    fn get_definition(&self) -> Option<(usize, usize)>;
}

pub trait GetReduction {
    fn get_reduction(&self) -> Option<usize>;
}

pub trait FindWhatReducesToIt {
    fn find_what_reduces_to_it(&self) -> HashSet<usize>;
}

impl<T> FindWhatReducesToIt for T
where
    T: ConcreteReader,
    T::C: FindWhatReducesToIt,
{
    fn find_what_reduces_to_it(&self) -> HashSet<usize> {
        self.read_concrete().find_what_reduces_to_it()
    }
}

pub trait MaybeString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetDefinitionOf {
    fn get_lefthand_of(&self) -> HashSet<usize>;
    fn get_righthand_of(&self) -> HashSet<usize>;
}

impl<T> GetDefinitionOf for T
where
    T: ConcreteReader,
    T::C: GetDefinitionOf,
{
    fn get_lefthand_of(&self) -> HashSet<usize> {
        self.read_concrete().get_lefthand_of()
    }
    fn get_righthand_of(&self) -> HashSet<usize> {
        self.read_concrete().get_righthand_of()
    }
}

pub trait ConcreteReader {
    type C;
    fn read_concrete(&self) -> &Self::C;
}
