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
use constants::LABEL;
use std::fmt::Display;
use std::marker::Sized;
use traits::call::label_getter::{GetDefinitionOf, LabelGetter};
use traits::call::{GetNormalForm, MaybeConcept, MightExpand};
use traits::syntax_converter::label::GetNormalFormOf;
use traits::{GetDefinition, Id};
use utils::{ZiaError, ZiaResult};

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}

pub trait Definer3<T, U>
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + LabelGetter
        + MaybeDisconnected,
    U: MightExpand + MaybeConcept<T> + Pair<U> + PartialEq + Display,
    Self: Definer2<T, U> + ConceptMaker<T, U>,
{
    fn define(&mut self, before: &mut U, after: &U) -> ZiaResult<()> {
        if after.get_expansion().is_some() {
            Err(ZiaError::Syntax(
                "Only symbols can have definitions".to_string(),
            ))
        } else if before == after {
            if let Some(ref mut before_c) = before.get_concept() {
                self.delete_definition(before_c)
            } else {
                Ok(())
            }
        } else if let Some(mut before_c) = before.get_concept() {
            self.define2(&mut before_c, after)
        } else if let Some((ref before_left, ref before_right)) = before.get_expansion() {
            if let Some(mut after_c) = after.get_concept() {
                if let Some((ref mut after_left, ref mut after_right)) = after_c.get_definition() {
                    try!(self.define2(after_left, before_left));
                    self.define2(after_right, before_right)
                } else {
                    let mut left_concept = try!(self.concept_from_ast(before_left));
                    let mut right_concept = try!(self.concept_from_ast(before_right));
                    after_c.insert_definition(&mut left_concept, &mut right_concept);
                    Ok(())
                }
            } else {
                println!("Defining a new concept in terms of two other new concepts");
                let new_syntax = try!(U::from_pair(&after.to_string(), before_left, before_right));
                try!(self.concept_from_ast(&new_syntax));
                Ok(())
            }
        } else {
            return Err(ZiaError::Redundancy(
                "Refactoring a symbol that was never previously used is redundant".to_string(),
            ));
        }
    }
    fn delete_definition(&mut self, concept: &mut T) -> ZiaResult<()> {
        let definition = concept.get_definition();
        concept.delete_definition();
        try!(self.try_delete_concept(concept));
        if let Some((ref left, ref right)) = definition {
            try!(self.try_delete_concept(left));
            try!(self.try_delete_concept(right));
        }
        Ok(())
    }
    fn try_delete_concept(&mut self, concept: &T) -> ZiaResult<()> {
        if try!(concept.is_disconnected()) {
            try!(self.unlabel(concept));
            self.cleanly_remove_concept(concept);
        }
        Ok(())
    }
}

impl<S, T, U> Definer3<T, U> for S
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + LabelGetter
        + MaybeDisconnected,
    U: MightExpand + MaybeConcept<T> + Pair<U> + PartialEq + Display,
    S: Definer2<T, U> + ConceptMaker<T, U>,
{}

pub trait Pair<T>
where
    Self: Sized,
{
    fn from_pair(&str, &T, &T) -> ZiaResult<Self>;
}

pub trait MaybeDisconnected
where
    Self: GetNormalForm<Self>
        + GetNormalFormOf<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + Id,
{
    fn is_disconnected(&self) -> ZiaResult<bool> {
        Ok(try!(self.get_normal_form()).is_none()
            && self.get_definition().is_none()
            && self.get_lefthand_of().is_empty()
            && self.righthand_of_without_label_is_empty()
            && self.get_normal_form_of().is_empty())
    }
    fn righthand_of_without_label_is_empty(&self) -> bool {
        for concept in self.get_righthand_of() {
            if let Some((left, _)) = concept.get_definition() {
                if left.get_id() != LABEL {
                    return false;
                }
            }
        }
        true
    }
}

impl<T> MaybeDisconnected for T where
    T: GetNormalForm<T> + GetNormalFormOf<T> + GetDefinition<T> + GetDefinitionOf<T> + Id
{}
