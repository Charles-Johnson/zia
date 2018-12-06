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
use std::marker::Sized;
use traits::call::GetReduction;

pub trait DeleteReduction
where
    Self: GetReduction<Self> + RemoveReduction<Self> + Sized,
{
    fn delete_reduction(&mut self) {
        match self.get_reduction() {
            None => panic!("No normal form to delete"),
            Some(mut n) => {
                n.no_longer_reduces_from(self);
                self.make_reduce_to_none();
            }
        };
    }
}

impl<T> DeleteReduction for T where T: GetReduction<T> + RemoveReduction<T> {}

pub trait RemoveReduction<T> {
    fn make_reduce_to_none(&mut self);
    fn no_longer_reduces_from(&mut self, &T);
}
