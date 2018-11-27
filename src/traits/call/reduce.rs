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
use std::fmt;
use std::ops::Add;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, LabelGetter, MaybeConcept, MightExpand,
    SyntaxFactory,
};
use utils::{ZiaError, ZiaResult};

pub trait Reduce<T, U>
where
    Self: SyntaxFromConcept<T, U>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + MaybeConcept<T> + MightExpand + Add<U, Output = ZiaResult<U>> + Clone,
{
    fn reduce_concept(&mut self, c: &T) -> ZiaResult<Option<U>> {
        match try!(c.get_normal_form()) {
            None => match c.get_definition() {
                Some((ref mut left, ref mut right)) => {
                    let left_result = try!(self.reduce_concept(left));
                    let right_result = try!(self.reduce_concept(right));
                    match_left_right::<U>(
                        left_result,
                        right_result,
                        &try!(self.ast_from_concept(left)),
                        &try!(self.ast_from_concept(right)),
                    )
                }
                None => Ok(None),
            },
            Some(ref n) => Ok(Some(try!(self.ast_from_concept(n)))),
        }
    }
    fn recursively_reduce(&mut self, ast: &U) -> ZiaResult<U> {
        match try!(self.reduce(ast)) {
            Some(ref a) => self.recursively_reduce(a),
            None => Ok(ast.clone()),
        }
    }
    fn reduce(&mut self, ast: &U) -> ZiaResult<Option<U>> {
        match ast.get_concept() {
            Some(ref c) => self.reduce_concept(c),
            None => match ast.get_expansion() {
                None => Ok(None),
                Some((ref left, ref right)) => match_left_right::<U>(
                    try!(self.reduce(left)),
                    try!(self.reduce(right)),
                    left,
                    right,
                ),
            },
        }
    }
}

impl<S, T, U> Reduce<T, U> for S
where
    S: SyntaxFromConcept<T, U>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + MaybeConcept<T> + MightExpand + Add<U, Output = ZiaResult<U>> + Clone,
{}

pub trait SyntaxFromConcept<T, U>
where
    Self: LabelGetter<T>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + Add<U, Output = ZiaResult<U>>,
{
    fn ast_from_concept(&self, c: &T) -> ZiaResult<U> {
        match try!(self.get_label(c)) {
            Some(ref s) => Ok(U::new(s, Some(c.clone()))),
            None => match c.get_definition() {
                Some((ref left, ref right)) => {
                    try!(self.ast_from_concept(left)) + try!(self.ast_from_concept(right))
                }
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
        }
    }
}

impl<S, T, U> SyntaxFromConcept<T, U> for S
where
    S: LabelGetter<T>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + Add<U, Output = ZiaResult<U>>,
{}

fn match_left_right<T: Clone + Add<T, Output = ZiaResult<T>>>(
    left: Option<T>,
    right: Option<T>,
    original_left: &T,
    original_right: &T,
) -> ZiaResult<Option<T>> {
    match (left, right) {
        (None, None) => Ok(None),
        (Some(new_left), None) => Ok(Some(try!(new_left + original_right.clone()))),
        (None, Some(new_right)) => Ok(Some(try!(original_left.clone() + new_right))),
        (Some(new_left), Some(new_right)) => Ok(Some(try!(new_left + new_right))),
    }
}
