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
use self::refactor::delete_normal_form::DeleteReduction;
use self::refactor::refactor_id::ConceptCleaner;
use self::refactor::Unlabeller;
use concepts::Display;
use constants::LABEL;
use std::marker::Sized;
use traits::call::label_getter::{FindDefinition, GetDefinitionOf};
use traits::call::{FindWhatReducesToIt, GetReduction, MaybeConcept, MightExpand};
use traits::{GetDefinition, GetId};
use utils::{ZiaError, ZiaResult};

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}

pub trait Definer<T>
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + Unlabeller
        + MaybeDisconnected
        + FindDefinition<T>,
    Self: ConceptMaker<T> + ConceptCleaner<T>,
{
    fn define<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(&mut self, before: &mut U, after: &U) -> ZiaResult<()> {
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
                (None, Some(ref mut b), Some(_)) => {
                    if b.get_label().is_none() {
                        self.label(b, &after.to_string())
                    } else {
                        self.relabel(b, &after.to_string())
                    }
                }
                (None, None, Some((ref left, ref right))) => {
                    self.define_new_syntax(&after.to_string(), left, right)
                }
                (Some(ref mut a), Some(ref mut b), None) => self.check_to_delete_definition(b, a),
                (Some(ref mut a), Some(ref mut b), Some(_)) => {
                    self.check_for_redundant_definition(b, a)
                }
                (Some(ref mut a), None, Some((ref left, ref right))) => {
                    self.redefine(a, left, right)
                }
            }
        }
    }
    fn check_to_delete_definition(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            self.delete_definition(before);
            Ok(())
        } else {
            Err(ZiaError::DefinitionCollision)
        }
    }
    fn check_for_redundant_definition(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            Err(ZiaError::RedundantDefinition)
        } else {
            Err(ZiaError::DefinitionCollision)
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
    fn redefine<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(&mut self, concept: &mut T, left: &U, right: &U) -> ZiaResult<()> {
        if let Some((ref mut left_concept, ref mut right_concept)) = concept.get_definition() {
            try!(self.relabel(left_concept, &left.to_string()));
            self.relabel(right_concept, &right.to_string())
        } else {
            let mut left_concept = try!(self.concept_from_ast(left));
            let mut right_concept = try!(self.concept_from_ast(right));
            try!(concept.insert_definition(&mut left_concept, &mut right_concept));
            Ok(())
        }
    }
    fn relabel(&mut self, concept: &mut T, new_label: &str) -> ZiaResult<()> {
        concept.unlabel();
        self.label(concept, new_label)
    }
    fn define_new_syntax<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(&mut self, syntax: &str, left: &U, right: &U) -> ZiaResult<()> {
        let mut definition_concept: Option<T> = None;
        if let (Some(ref l), Some(ref r)) = (left.get_concept(), right.get_concept()) {
            definition_concept = l.find_definition(r);
        }
        let new_syntax_tree = U::from_pair(syntax, definition_concept, left, right);
        try!(self.concept_from_ast(&new_syntax_tree));
        Ok(())
    }
}

impl<S, T> Definer<T> for S
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + MaybeDisconnected
        + Unlabeller
        + FindDefinition<T>,
    S: ConceptMaker<T> + ConceptCleaner<T>,
{
}

pub trait Pair<T, U> {
    fn from_pair(&str, Option<T>, &U, &U) -> Self;
}

pub trait MaybeDisconnected
where
    Self: GetReduction<Self>
        + FindWhatReducesToIt<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + GetId
        + Sized,
{
    fn is_disconnected(&self) -> bool {
        self.get_reduction().is_none()
            && self.get_definition().is_none()
            && self.get_lefthand_of().is_empty()
            && self.righthand_of_without_label_is_empty()
            && self.find_what_reduces_to_it().is_empty()
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
    T: GetReduction<T> + FindWhatReducesToIt<T> + GetDefinition<T> + GetDefinitionOf<T> + GetId
{
}
