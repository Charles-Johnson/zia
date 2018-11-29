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
pub mod delete_normal_form;
pub mod refactor_id;

use self::delete_normal_form::DeleteNormalForm;
use self::refactor_id::{RefactorFrom, RefactorId};
use token::Token;
use traits::call::label_getter::LabelGetter;
use traits::call::left_hand_call::definer3::labeller::{
    AbstractFactory, InsertDefinition, Labeller, StringFactory, UpdateNormalForm,
};
use traits::call::{HasToken, MaybeConcept};
use utils::{ZiaError, ZiaResult};

pub trait Definer2<T, U>
where
    T: InsertDefinition
        + StringFactory
        + AbstractFactory
        + RefactorFrom
        + DeleteNormalForm
        + UpdateNormalForm
        + LabelGetter,
    U: HasToken + MaybeConcept<T>,
    Self: Refactor<T> + Labeller<T>,
{
    fn define2(&mut self, before_c: &mut T, after: &U) -> ZiaResult<()> {
        if let Some(mut after_c) = after.get_concept() {
            self.refactor(before_c, &mut after_c)
        } else {
            match after.get_token() {
                Token::Atom(s) => {
                    try!(self.unlabel(before_c));
                    self.label(before_c, &s)
                }
                Token::Expression(_) => Err(ZiaError::Syntax(
                    "Only symbols can have definitions".to_string(),
                )),
            }
        }
    }
}

pub trait Refactor<T>
where
    T: RefactorFrom + DeleteNormalForm + LabelGetter,
    Self: RefactorId<T> + Unlabeller<T>,
{
    fn refactor(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        try!(self.unlabel(before));
        self.refactor_id(before, after)
    }
}

impl<S, T> Refactor<T> for S
where
    T: RefactorFrom + DeleteNormalForm + LabelGetter,
    S: RefactorId<T> + Unlabeller<T>,
{}

pub trait Unlabeller<T>
where
    T: LabelGetter + DeleteNormalForm,
{
    fn unlabel(&mut self, concept: &T) -> ZiaResult<()> {
        match concept.get_concept_of_label() {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
}

impl<S, T> Unlabeller<T> for S where T: LabelGetter + DeleteNormalForm {}
