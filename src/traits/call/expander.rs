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
use ast::Combine;
use concepts::traits::Display;
use traits::call::reduce::SyntaxFromConcept;
use traits::call::right_hand_call::definer::Pair;
use traits::call::{MaybeConcept, MightExpand};
use traits::SyntaxFactory;

pub trait Expander<T>
where
    T: Display + SyntaxFromConcept,
    Self: MaybeConcept<T>
        + MightExpand
        + Display
        + Clone
        + Pair<T, Self>
        + Combine<T>
        + SyntaxFactory<T>,
{
    fn expand(&self) -> Self {
        if let Some(ref con) = self.get_concept() {
            if let Some((ref left, ref right)) = con.get_definition() {
                left.to_ast::<Self>()
                    .expand()
                    .combine_with(&right.to_ast::<Self>().expand())
            } else {
                con.to_ast::<Self>()
            }
        } else if let Some((ref left, ref right)) = self.get_expansion() {
            left.expand().combine_with(&right.expand())
        } else {
            self.clone()
        }
    }
}

impl<S, T> Expander<T> for S
where
    T: Display + SyntaxFromConcept,
    S: MaybeConcept<T> + MightExpand + Display + Clone + Pair<T, S> + Combine<T> + SyntaxFactory<T>,
{
}
