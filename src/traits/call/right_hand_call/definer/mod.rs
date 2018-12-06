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
pub mod concept_maker;
pub mod delete_definition;
pub mod labeller;
pub mod refactor;

use self::concept_maker::ConceptMaker;
use self::delete_definition::DeleteDefinition;
use self::labeller::{AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm};
use self::refactor::delete_normal_form::DeleteNormalForm;
use self::refactor::refactor_id::RefactorFrom;
use self::refactor::{Refactor, Unlabeller};
use constants::LABEL;
use std::fmt::Display;
use std::marker::Sized;
use traits::call::label_getter::{GetDefinitionOf, LabelGetter};
use traits::call::{GetReduction, MaybeConcept, MightExpand};
use traits::syntax_converter::label::GetNormalFormOf;
use traits::{GetDefinition, Id};
use utils::{ZiaError, ZiaResult};

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}

pub trait Definer<T, U>
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + Unlabeller
        + MaybeDisconnected,
    U: MightExpand + MaybeConcept<T> + Pair<U> + PartialEq + Display,
    Self: Refactor<T> + ConceptMaker<T, U>,
{
    fn define(&mut self, before: &mut U, after: &U) -> ZiaResult<()> {
        if after.get_expansion().is_some() {
            Err(ZiaError::BadDefinition)
        } else {
            match (
                after.get_concept(),
                before.get_concept(),
                before.get_expansion(),
            ) {
                (_, None, None) => Err(ZiaError::RedundantRefactor),
                (None, Some(ref mut b), None) => self.relabel(b, &after.to_string()),
				(None, Some(ref mut b), Some(_)) => if b.get_label().is_none() {
					self.label(b, &after.to_string())
				} else {
					self.relabel(b, &after.to_string())
				},
                (None, None, Some((ref left, ref right))) => {
                    self.define_new_syntax(&after.to_string(), left, right)
                }
                (Some(ref mut a), Some(ref mut b), None) => self.refactor_atom(b, a),
                (Some(ref mut a), Some(ref mut b), Some(_)) => self.refactor_expression(b, a),
                (Some(ref mut a), None, Some((ref left, ref right))) => {
                    self.redefine(a, left, right)
                }
            }
        }
    }
    fn refactor_atom(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            self.delete_definition(before);
            Ok(())
        } else {
            self.refactor(before, after)
        }
    }
    fn refactor_expression(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            Err(ZiaError::RedundantDefinition)
        } else {
            self.refactor(before, after)
        }
    }
    fn delete_definition(&mut self, concept: &mut T) {
        let mut definition = concept.get_definition();
        concept.delete_definition();
        self.try_delete_concept(concept);
        if let Some((ref mut left, ref mut right)) = definition {
            self.try_delete_concept(left);
            self.try_delete_concept(right);
        }
    }
    fn try_delete_concept(&mut self, concept: &mut T) {
        if concept.is_disconnected() {
            concept.unlabel();
            self.cleanly_remove_concept(concept);
        }
    }
    fn redefine(&mut self, concept: &mut T, left: &U, right: &U) -> ZiaResult<()> {
        if let Some((ref mut left_concept, ref mut right_concept)) = concept.get_definition() {
            try!(self.relabel(left_concept, &left.to_string()));
            self.relabel(right_concept, &right.to_string())
        } else {
            let mut left_concept = try!(self.concept_from_ast(left));
            let mut right_concept = try!(self.concept_from_ast(right));
            concept.insert_definition(&mut left_concept, &mut right_concept);
            Ok(())
        }
    }
    fn relabel(&mut self, concept: &mut T, new_label: &str) -> ZiaResult<()> {
        concept.unlabel();
        self.label(concept, new_label)
    }
    fn define_new_syntax(&mut self, syntax: &str, left: &U, right: &U) -> ZiaResult<()> {
        let new_syntax_tree = U::from_pair(syntax, left, right);
        try!(self.concept_from_ast(&new_syntax_tree));
        Ok(())
    }
}

impl<S, T, U> Definer<T, U> for S
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
    S: Refactor<T> + ConceptMaker<T, U>,
{
}

pub trait Pair<T> {
    fn from_pair(&str, &T, &T) -> Self;
}

pub trait MaybeDisconnected
where
    Self: GetReduction<Self>
        + GetNormalFormOf<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + Id
        + Sized,
{
    fn is_disconnected(&self) -> bool {
        self.get_reduction().is_none()
            && self.get_definition().is_none()
            && self.get_lefthand_of().is_empty()
            && self.righthand_of_without_label_is_empty()
            && self.get_normal_form_of().is_empty()
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
    T: GetReduction<T> + GetNormalFormOf<T> + GetDefinition<T> + GetDefinitionOf<T> + Id
{
}
