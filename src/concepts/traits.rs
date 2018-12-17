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

pub trait GetDefinition {
    fn get_definition(&self) -> Option<(usize, usize)>;
}

pub trait GetReduction {
    fn get_reduction(&self) -> Option<usize>;
}

pub trait FindWhatReducesToIt {
    fn find_what_reduces_to_it(&self) -> Vec<usize>;
}

pub trait RemoveReduction {
    fn make_reduce_to_none(&mut self);
    fn no_longer_reduces_from(&mut self, usize);
}

pub trait SetDefinition {
    fn set_definition(&mut self, usize, usize);
    fn add_as_lefthand_of(&mut self, usize);
    fn add_as_righthand_of(&mut self, usize);
}

pub trait SetReduction {
    fn make_reduce_to(&mut self, usize);
    fn make_reduce_from(&mut self, usize);
}

pub trait RemoveDefinition {
    fn remove_definition(&mut self);
    fn remove_as_lefthand_of(&mut self, usize);
    fn remove_as_righthand_of(&mut self, usize);
}

pub trait MaybeString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetDefinitionOf {
    fn get_lefthand_of(&self) -> Vec<usize>;
    fn get_righthand_of(&self) -> Vec<usize>;
}
