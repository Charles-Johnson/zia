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
mod concept_maker;
pub mod definer2;
pub mod delete_definition;
pub mod labeller;

use self::concept_maker::ConceptMaker;
use self::definer2::delete_normal_form::DeleteNormalForm;
use self::definer2::refactor_id::RefactorFrom;
use self::definer2::Definer2;
use self::delete_definition::DeleteDefinition;
use self::labeller::{AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm};
use std::marker;
use token::Token;
use traits::call::label_getter::LabelGetter;
use traits::call::{HasToken, MaybeConcept, MightExpand};
use utils::{ZiaError, ZiaResult};

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}

pub trait Definer3<T, U>
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom<T>
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + LabelGetter,
    U: MightExpand + MaybeConcept<T> + HasToken + Pair + PartialEq,
    Self: Definer2<T, U> + ConceptMaker<T, U>,
{
    fn define(&mut self, before: &U, after: &U) -> ZiaResult<()> {
        if let Some(mut before_c) = before.get_concept() {
            if before == after {
                before_c.delete_definition();
                Ok(())
            } else {
                self.define2(&mut before_c, after)
            }
        } else if let Some((ref before_left, ref before_right)) = before.get_expansion() {
            if let Some(mut after_c) = after.get_concept() {
                if let Some((ref mut after_left, ref mut after_right)) = after_c.get_definition() {
                    try!(self.define2(after_left, before_left));
                    self.define2(after_right, before_right)
                } else {
                    after_c.insert_definition(
                        &mut try!(self.concept_from_ast(before_left)),
                        &mut try!(self.concept_from_ast(before_right)),
                    );
                    Ok(())
                }
            } else {
                try!(self.concept_from_ast(&try!(U::from_pair(
                    after.get_token(),
                    before_left,
                    before_right,
                ))));
                Ok(())
            }
        } else {
            return Err(ZiaError::Redundancy(
                "Refactoring a symbol that was never previously used is redundant".to_string(),
            ));
        }
    }
}

impl<S, T, U> Definer3<T, U> for S
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom<T>
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + LabelGetter,
    U: MightExpand + MaybeConcept<T> + HasToken + Pair + PartialEq,
    S: Definer2<T, U> + ConceptMaker<T, U>,
{}

pub trait Pair
where
    Self: marker::Sized + Clone,
{
    fn from_pair(Token, &Self, &Self) -> ZiaResult<Self>;
}
