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
use token::Token;
use traits::call::{HasToken, MaybeConcept};
use traits::call::left_hand_call::definer3::labeller::{AbstractFactory, StringFactory, Labeller, UpdateNormalForm, InsertDefinition};
use traits::call::left_hand_call::definer3::ConceptNumber;
use traits::{FindDefinition, Id};
use traits::call::GetNormalForm;
use traits::call::label_getter::LabelGetter;
use utils::{ZiaError, ZiaResult};

pub trait Definer2<T, U>
where
    T: InsertDefinition
        + StringFactory
        + AbstractFactory
        + fmt::Display
        + Id
        + RefactorFrom<T>
        + DeleteNormalForm
        + UpdateNormalForm
        + Clone
        + PartialEq
        + FindDefinition<T>,
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
    T: RefactorFrom<T>
        + Id
        + DeleteNormalForm
        + fmt::Display
        + PartialEq
        + FindDefinition<T>
        + Clone,
    Self: RefactorId<T> + Unlabeller<T>,
{
    fn refactor(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        try!(self.unlabel(before));
        self.refactor_id(before, after)
    }
}

pub trait RefactorId<T>
where
    T: Id + RefactorFrom<T>,
    Self: ConceptTidyer<T> + ConceptNumber,
{
    fn refactor_id(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if self.number_of_concepts() > before.get_id() {
            try!(after.refactor_from(before));
            self.remove_concept(before);
            for id in before.get_id()..self.number_of_concepts() {
                self.correct_id(id);
            }
            Ok(())
        } else {
            panic!("refactoring id has gone wrong!")
        }
    }
}

pub trait DeleteNormalForm
where
    Self: GetNormalForm<Self> + RemoveNormalForm<Self>,
{
    fn delete_normal_form(&mut self) -> ZiaResult<()> {
        match try!(self.get_normal_form()) {
            None => (),
            Some(mut n) => {
                n.remove_normal_form_of(self);
                self.remove_normal_form();
            }
        };
        Ok(())
    }
}

pub trait RemoveNormalForm<T> {
    fn remove_normal_form(&mut self);
    fn remove_normal_form_of(&mut self, &T);
}

/////////////////////////////////////////////////////////////////////////////////

pub trait Unlabeller<T>
where
    T: FindDefinition<T> + PartialEq + DeleteNormalForm + fmt::Display + Clone,
    Self: LabelGetter<T>,
{
    fn unlabel(&mut self, concept: &T) -> ZiaResult<()> {
        match try!(self.get_concept_of_label(concept)) {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
}

pub trait RefactorFrom<T> {
    fn refactor_from(&mut self, &T) -> ZiaResult<()>;
}

pub trait ConceptTidyer<T> {
    fn remove_concept(&mut self, &T);
    fn correct_id(&mut self, usize);
}
