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
pub use self::container::{Container, MightExpand};

mod container {
	pub trait Container
	where
		Self: PartialEq + MightExpand,
	{
		fn contains(&self, other: &Self) -> bool {
		    if let Some((ref left, ref right)) = self.get_expansion() {
		        left == other || right == other || left.contains(other) || right.contains(other)
		    } else {
		        false
		    }
		}
	}

	impl<T> Container for T where T: PartialEq + MightExpand {}

	pub trait MightExpand
	where
		Self: Sized,
	{
		fn get_expansion(&self) -> Option<(Self, Self)>;
	}
}

pub trait Display {
    fn to_string(&self) -> String;
}

pub trait DisplayJoint {
    fn display_joint(&self) -> String;
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait Pair<T, U> {
    fn from_pair(&str, Option<T>, &U, &U) -> Self;
}

pub trait SyntaxFactory<T> {
    fn new(&str, Option<T>) -> Self;
}
