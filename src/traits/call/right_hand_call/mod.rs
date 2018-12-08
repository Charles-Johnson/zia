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
pub mod definer;

use ast::Combine;
use self::definer::delete_definition::DeleteDefinition;
use self::definer::labeller::{AbstractFactory, InsertDefinition, StringFactory, UpdateNormalForm};
use self::definer::refactor::delete_normal_form::DeleteReduction;
use self::definer::{Definer, MaybeDisconnected, Pair};
use constants::{DEFINE, REDUCTION};
use std::fmt::Display;
use traits::call::reduce::SyntaxFromConcept;
use traits::call::{MaybeConcept, MightExpand};
use traits::SyntaxFactory;
use utils::{ZiaError, ZiaResult};

pub trait RightHandCall<T, U>
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + AbstractFactory
        + StringFactory
        + MaybeDisconnected
		+ SyntaxFromConcept<U>,
    U: MaybeConcept<T> + Container + Pair<T, U> + Display + Clone + Combine<T> + SyntaxFactory<T>,
    Self: Definer<T, U>,
{
    fn call_as_righthand(&mut self, left: &mut U, right: &U) -> ZiaResult<String> {
        match right.get_expansion() {
            Some((ref rightleft, ref mut rightright)) => self.match_righthand_pair(left, rightleft, rightright),
            None => Err(ZiaError::NotAProgram),
        }
    }
	fn match_righthand_pair(&mut self, left: &mut U, rightleft: &U, rightright: &mut U) -> ZiaResult<String> {
		match rightleft.get_concept() {
            Some(c) => match c.get_id() {
                REDUCTION => self.try_reduction(left, rightright),
                DEFINE => self.try_definition(left, rightright),
                _ => { 
					let rightleft_reduction = c.get_reduction();
					if let Some(r) = rightleft_reduction  {
						self.match_righthand_pair(left, &r.to_ast(), rightright)
					} else {
						Err(ZiaError::NotAProgram)
					}
				},
            },
            None => Err(ZiaError::NotAProgram),
		}
	}
    fn try_reduction(&mut self, syntax: &mut U, normal_form: &U) -> ZiaResult<String> {
        if normal_form.contains(syntax) {
            Err(ZiaError::ExpandingReduction)
        } else if syntax == normal_form {
            if let Some(mut c) = syntax.get_concept() {
                c.delete_reduction();
                Ok("".to_string())
            } else {
                Err(ZiaError::RedundantReduction)
            }
        } else {
            let mut syntax_concept = try!(self.concept_from_ast(syntax));
            let mut normal_form_concept = try!(self.concept_from_ast(normal_form));
            try!(syntax_concept.update_normal_form(&mut normal_form_concept));
            Ok("".to_string())
        }
    }
    fn try_definition(&mut self, new: &U, old: &mut U) -> ZiaResult<String> {
        if old.contains(new) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.define(old, new));
            Ok("".to_string())
        }
    }
}

impl<S, T, U> RightHandCall<T, U> for S
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + AbstractFactory
        + StringFactory
        + MaybeDisconnected
		+ SyntaxFromConcept<U>,
    U: MaybeConcept<T> + Container + Pair<T, U> + Display + Clone + Combine<T> + SyntaxFactory<T>,
    Self: Definer<T, U>,
{
}

pub trait Container
where
    Self: PartialEq + MightExpand,
{
    fn contains(&self, other: &Self) -> bool {
        if let Some((ref left, ref right)) = self.get_expansion() {
            left == other || right == other || left.contains(other) || right.contains(other)
        } else {
            false
        }
    }
}

impl<T> Container for T where T: PartialEq + MightExpand {}
