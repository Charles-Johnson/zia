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
pub use self::container::*;

mod container {
    pub use reading::MightExpand;
    pub trait Container
    where
        Self: MightExpand<Self> + PartialEq + Sized,
    {
        fn contains(&self, other: &Self) -> bool {
            if let Some((ref left, ref right)) = self.get_expansion() {
                left == other || right == other || left.contains(other) || right.contains(other)
            } else {
                false
            }
        }
    }

    impl<T> Container for T where T: MightExpand<T> + PartialEq + Sized {}
}
