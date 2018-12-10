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

pub trait GetId {
    fn get_id(&self) -> usize;
}

pub trait SetId {
    fn set_id(&mut self, id: usize);
}

pub trait GetDefinition<T> {
    fn get_definition(&self) -> Option<(T, T)>;
}

pub trait GetReduction<T> {
    fn get_reduction(&self) -> Option<T>;
}

pub trait FindWhatReducesToIt<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T>;
}

pub trait RemoveReduction<T> {
    fn make_reduce_to_none(&mut self);
    fn no_longer_reduces_from(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_as_lefthand_of(&mut self, &T);
    fn add_as_righthand_of(&mut self, &T);
}

pub trait SetReduction<T> {
    fn make_reduce_to(&mut self, &T);
    fn make_reduce_from(&mut self, &T);
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_as_lefthand_of(&mut self, &T);
    fn remove_as_righthand_of(&mut self, &T);
}

pub trait MaybeString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}

pub trait ConvertTo<T> {
    fn convert(&self) -> Option<T>;
}
