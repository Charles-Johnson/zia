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
use std::ops::Add;
use token::Token;
use traits::call::label_getter::{FindDefinition, LabelGetter};
use traits::call::left_hand_call::definer3::Pair;
use traits::call::{HasToken, MaybeConcept, MightExpand};
use traits::SyntaxFactory;
use utils::ZiaResult;

pub trait Reduce<T>
where
    T: SyntaxFromConcept<Self>,
    Self: SyntaxFactory<T>
        + Add<Self, Output = ZiaResult<Self>>
        + MaybeConcept<T>
        + MightExpand
        + HasToken
        + Pair,
{
    fn recursively_reduce(&self) -> ZiaResult<Self> {
        match try!(self.reduce()) {
            Some(ref a) => a.recursively_reduce(),
            None => Ok(self.clone()),
        }
    }
    fn reduce(&self) -> ZiaResult<Option<Self>> {
        match self.get_concept() {
            Some(ref c) => c.reduce_concept(),
            None => match self.get_expansion() {
                None => Ok(None),
                Some((ref left, ref right)) => match_left_right::<T, Self>(
                    try!(left.reduce()),
                    try!(right.reduce()),
                    left,
                    right,
                ),
            },
        }
    }
}

impl<S, T> Reduce<T> for S
where
    T: SyntaxFromConcept<S> + LabelGetter,
    S: SyntaxFactory<T>
        + Add<S, Output = ZiaResult<S>>
        + MaybeConcept<T>
        + MightExpand
        + HasToken
        + Pair
        + Clone,
{}

pub trait SyntaxFromConcept<T>
where
    Self: LabelGetter,
    T: SyntaxFactory<Self>
        + Add<T, Output = ZiaResult<T>>
        + MaybeConcept<Self>
        + HasToken
        + Pair
        + Clone,
{
    fn reduce_concept(&self) -> ZiaResult<Option<T>> {
        match try!(self.get_normal_form()) {
            None => match self.get_definition() {
                Some((ref left, ref right)) => {
                    let left_result = try!(left.reduce_concept());
                    let right_result = try!(right.reduce_concept());
                    match_left_right::<Self, T>(
                        left_result,
                        right_result,
                        &try!(left.ast_from_concept()),
                        &try!(right.ast_from_concept()),
                    )
                }
                None => Ok(None),
            },
            Some(ref n) => Ok(Some(try!(n.ast_from_concept()))),
        }
    }
    fn ast_from_concept(&self) -> ZiaResult<T> {
        match try!(self.get_label()) {
            Some(ref s) => Ok(T::new(s, Some(self.clone()))),
            None => match self.get_definition() {
                Some((ref left, ref right)) => {
                    try!(left.ast_from_concept()) + try!(right.ast_from_concept())
                }
                None => panic!("Unlabelled concept with no definition"),
            },
        }
    }
}

impl<S, T> SyntaxFromConcept<T> for S
where
    S: LabelGetter,
    T: SyntaxFactory<S>
        + Add<T, Output = ZiaResult<T>>
        + MaybeConcept<Self>
        + HasToken
        + Pair
        + Clone,
{}

fn match_left_right<T: LabelGetter, U: HasToken + Pair + MaybeConcept<T>>(
    left: Option<U>,
    right: Option<U>,
    original_left: &U,
    original_right: &U,
) -> ZiaResult<Option<U>> {
    match (left, right) {
        (None, None) => Ok(None),
        (Some(new_left), None) => Ok(Some(try!(contract_pair::<T, U>(&new_left, original_right)))),
        (None, Some(new_right)) => Ok(Some(try!(contract_pair::<T, U>(original_left, &new_right)))),
        (Some(new_left), Some(new_right)) => {
            Ok(Some(try!(contract_pair::<T, U>(&new_left, &new_right))))
        }
    }
}

fn contract_pair<T: LabelGetter, U: HasToken + Pair + MaybeConcept<T>>(
    lefthand: &U,
    righthand: &U,
) -> ZiaResult<U> {
    if let (Some(lc), Some(rc)) = (lefthand.get_concept(), righthand.get_concept()) {
        if let Some(def) = try!(lc.find_definition(&rc)) {
            if let Some(ref a) = try!(def.get_label()) {
                return U::from_pair(Token::Atom(a.clone()), lefthand, righthand);
            }
        }
    }
    U::from_pair(
        lefthand.get_token() + righthand.get_token(),
        lefthand,
        righthand,
    )
}
