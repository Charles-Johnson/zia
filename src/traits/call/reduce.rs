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
use traits::call::label_getter::{FindDefinition, LabelGetter};
use traits::call::MightExpand;
use traits::SyntaxFactory;

pub trait Reduce<T>
where
    T: SyntaxFromConcept<Self>,
    Self: SyntaxFactory<T> + Combine<T> + MightExpand + Clone,
{
    fn recursively_reduce(&self) -> Self {
        match self.reduce() {
            Some(ref a) => a.recursively_reduce(),
            None => self.clone(),
        }
    }
    fn reduce(&self) -> Option<Self> {
        match self.get_concept() {
            Some(ref c) => c.reduce(),
            None => match self.get_expansion() {
                None => None,
                Some((ref left, ref right)) => {
                    match_left_right::<T, Self>(left.reduce(), right.reduce(), left, right)
                }
            },
        }
    }
}

impl<S, T> Reduce<T> for S
where
    T: SyntaxFromConcept<S>,
    S: SyntaxFactory<T> + Combine<T> + MightExpand + Clone,
{
}

pub trait SyntaxFromConcept<T>
where
    Self: LabelGetter + FindDefinition<Self> + PartialEq,
    T: SyntaxFactory<Self> + Combine<Self> + Clone,
{
    fn reduce(&self) -> Option<T> {
        match self.get_normal_form() {
            None => match self.get_definition() {
                Some((ref left, ref right)) => {
                    let left_result = left.reduce();
                    let right_result = right.reduce();
                    match_left_right::<Self, T>(
                        left_result,
                        right_result,
                        &left.to_ast(),
                        &right.to_ast(),
                    )
                }
                None => None,
            },
            Some(ref n) => Some(n.to_ast()),
        }
    }
    fn to_ast(&self) -> T {
        match self.get_label() {
            Some(ref s) => T::new(s, Some(self.clone())),
            None => match self.get_definition() {
                Some((ref left, ref right)) => left.to_ast().combine_with(&right.to_ast()),
                None => panic!("Unlabelled concept with no definition"),
            },
        }
    }
}

impl<S, T> SyntaxFromConcept<T> for S
where
    S: LabelGetter + FindDefinition<S> + PartialEq,
    T: SyntaxFactory<S> + Combine<S> + Clone,
{
}

fn match_left_right<T: LabelGetter + FindDefinition<T> + PartialEq, U: Combine<T>>(
    left: Option<U>,
    right: Option<U>,
    original_left: &U,
    original_right: &U,
) -> Option<U> {
    match (left, right) {
        (None, None) => None,
        (Some(new_left), None) => Some(contract_pair::<T, U>(&new_left, original_right)),
        (None, Some(new_right)) => Some(contract_pair::<T, U>(original_left, &new_right)),
        (Some(new_left), Some(new_right)) => Some(contract_pair::<T, U>(&new_left, &new_right)),
    }
}

fn contract_pair<T: LabelGetter + FindDefinition<T> + PartialEq, U: Combine<T>>(
    lefthand: &U,
    righthand: &U,
) -> U {
    if let (Some(lc), Some(rc)) = (lefthand.get_concept(), righthand.get_concept()) {
        if let Some(def) = lc.find_definition(&rc) {
            if let Some(ref a) = def.get_label() {
                return U::from_pair(a, Some(def), &lefthand, &righthand);
            }
        }
    }
    lefthand.combine_with(righthand)
}
