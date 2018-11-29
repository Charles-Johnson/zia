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
use std::marker;

use traits::GetDefinition;
use traits::call::MaybeConcept;

pub trait DeleteDefinition
where
    Self: GetDefinition<Self> + RemoveDefinition<Self> + marker::Sized,
{
    fn delete_definition(&mut self) {
        match self.get_definition() {
            None => panic!("No definition to remove!"),
            Some((mut app, mut arg)) => {
                app.remove_lefthand_of(self);
                arg.remove_righthand_of(self);
                self.remove_definition();
            }
        };
    }
}

impl<T> DeleteDefinition for T where T: GetDefinition<T> + RemoveDefinition<T> + marker::Sized {}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_lefthand_of(&mut self, &T);
    fn remove_righthand_of(&mut self, &T);
}

impl<T,U> RemoveDefinition<T> for U 
where
	T: RemoveDefinition<T>,
	U: MaybeConcept<T>,
{
    fn remove_definition(&mut self) {
        if let Some(mut c) = self.get_concept() {
            c.remove_definition()
        }
    }
    fn remove_lefthand_of(&mut self, definition: &T) {
        if let Some(mut c) = self.get_concept() {
            c.remove_lefthand_of(definition)
        }
    }
    fn remove_righthand_of(&mut self, definition: &T) {
        if let Some(mut c) = self.get_concept() {
            c.remove_righthand_of(definition)
        }
    }
}
