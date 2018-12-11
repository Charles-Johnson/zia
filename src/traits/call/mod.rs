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
pub mod expander;
pub mod reduce;
pub mod right_hand_call;

pub use self::reduce::{Reduce, SyntaxFromConcept};
pub use concepts::traits::{GetReduction, FindWhatReducesToIt, GetNormalForm};
use traits::GetDefinition;
pub use ast::traits::{MightExpand, MaybeConcept};
pub use concepts::traits::{GetDefinitionOf, MaybeString, LabelGetter, FindDefinition};

impl<T> MightExpand for T
where
    T: GetDefinition<T>,
{
    fn get_expansion(&self) -> Option<(T, T)> {
        self.get_definition()
    }
}
