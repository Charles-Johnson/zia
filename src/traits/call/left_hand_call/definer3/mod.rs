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
mod labeller;

pub use self::labeller::{
    AbstractFactory, AbstractMaker, Definer, InsertDefinition, Labeller, SetDefinition,
    SetNormalForm, StringFactory, StringMaker, UpdateNormalForm,
};
use std::{fmt, marker};
use token::Token;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, HasToken, Id, LabelGetter, MaybeConcept,
    MightExpand,
};
use utils::{ZiaError, ZiaResult};

pub trait DeleteDefinition
where
    Self: GetDefinition<Self> + RemoveDefinition<Self> + marker::Sized,
{
    fn delete_definition(&mut self) {
        match self.get_definition() {
            None => panic!("No definition to remove!"),
            Some((mut app, mut arg)) => {
                app.remove_lefthand_of(self);
                arg.remove_righthand_of(self);
                self.remove_definition();
            }
        };
    }
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_lefthand_of(&mut self, &T);
    fn remove_righthand_of(&mut self, &T);
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

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
}

pub trait RefactorFrom<T> {
    fn refactor_from(&mut self, &T) -> ZiaResult<()>;
}

pub trait ConceptTidyer<T> {
    fn remove_concept(&mut self, &T);
    fn correct_id(&mut self, usize);
}

pub trait Definer3<T, U>
where
    T: fmt::Display
        + Id
        + DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom<T>
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + FindDefinition<T>
        + PartialEq
        + Clone,
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
        } else if let Some((ref left, ref right)) = before.get_expansion() {
            if let Some(mut after_c) = after.get_concept() {
                if let Some((ref mut ap, ref mut ar)) = after_c.get_definition() {
                    try!(self.define2(ap, left));
                    self.define2(ar, right)
                } else {
                    after_c.insert_definition(
                        &mut try!(self.concept_from_ast(left)),
                        &mut try!(self.concept_from_ast(right)),
                    );
                    Ok(())
                }
            } else {
                try!(self.concept_from_ast(&try!(U::from_pair(after.get_token(), left, right,))));
                Ok(())
            }
        } else {
            return Err(ZiaError::Redundancy(
                "Refactoring a symbol that was never previously used is redundant".to_string(),
            ));
        }
    }
}

pub trait ConceptMaker<T, U>
where
    T: StringFactory
        + AbstractFactory
        + fmt::Display
        + InsertDefinition
        + GetNormalForm<T>
        + UpdateNormalForm
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: MaybeConcept<T> + HasToken + MightExpand,
    Self: Labeller<T>,
{
    fn concept_from_ast(&mut self, ast: &U) -> ZiaResult<T> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let mut c = match ast.get_token() {
                Token::Atom(s) => try!(self.new_labelled_abstract(&s)),
                Token::Expression(_) => self.new_abstract(),
            };
            if let Some((mut app, mut arg)) = ast.get_expansion() {
                let mut appc = try!(self.concept_from_ast(&app));
                let mut argc = try!(self.concept_from_ast(&arg));
                c.insert_definition(&mut appc, &mut argc);
            }
            Ok(c)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

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

pub trait Pair
where
    Self: marker::Sized + Clone,
{
    fn from_pair(Token, &Self, &Self) -> ZiaResult<Self>;
}
