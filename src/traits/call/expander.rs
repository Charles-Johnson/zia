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
use std::fmt::Display;
use std::ops::Add;
use traits::call::reduce::SyntaxFromConcept;
use traits::call::right_hand_call::definer::Pair;
use traits::call::{MaybeConcept, MightExpand};
use traits::SyntaxFactory;

pub trait Expander<T>
where
    T: Display + SyntaxFromConcept<Self>,
    Self: MaybeConcept<T>
        + MightExpand
        + Display
        + Clone
        + Pair<T, Self>
        + Add<Self, Output = Self>
        + SyntaxFactory<T>,
{
    fn expand(&self) -> Self {
        if let Some(ref con) = self.get_concept() {
            if let Some((ref left, ref right)) = con.get_definition() {
                left.to_ast().expand() + right.to_ast().expand()
            } else {
                con.to_ast()
            }
        } else if let Some((ref left, ref right)) = self.get_expansion() {
            left.expand() + right.expand()
        } else {
            self.clone()
        }
    }
}

impl<S, T> Expander<T> for S
where
    T: Display + SyntaxFromConcept<S>,
    S: MaybeConcept<T>
        + MightExpand
        + Display
        + Clone
        + Pair<T, S>
        + Add<S, Output = S>
        + SyntaxFactory<T>,
{
}
