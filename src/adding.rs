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

use ast::traits::Container as SyntaxContainer;
use constants::LABEL;
use reading::{FindDefinition, MaybeString, MightExpand};
use std::fmt;
use writing::{
    DeleteReduction, GetDefinition, GetDefinitionOf, GetNormalForm, GetReduction, InsertDefinition,
    MaybeConcept, RemoveReduction, SetDefinition, SetReduction, UpdateNormalForm, ZiaError,
    ZiaResult,
};

pub trait ExecuteReduction<T>
where
    Self: ConceptMaker<T> + DeleteReduction<T>,
    T: SetReduction
        + GetDefinitionOf
        + Default
        + From<String>
        + RemoveReduction
        + GetReduction
        + SetDefinition
        + GetDefinition
        + MaybeString,
{
    fn execute_reduction<U: SyntaxContainer + MaybeConcept + fmt::Display>(
        &mut self,
        syntax: &U,
        normal_form: &U,
    ) -> ZiaResult<String> {
        if normal_form.contains(syntax) {
            Err(ZiaError::ExpandingReduction)
        } else if syntax == normal_form {
            try!(self.try_removing_reduction::<U>(syntax));
            Ok("".to_string())
        } else {
            let syntax_concept = try!(self.concept_from_ast::<U>(syntax));
            let normal_form_concept = try!(self.concept_from_ast::<U>(normal_form));
            try!(self.update_normal_form(syntax_concept, normal_form_concept));
            Ok("".to_string())
        }
    }
}

impl<S, T> ExecuteReduction<T> for S
where
    S: ConceptMaker<T> + DeleteReduction<T>,
    T: SetReduction
        + GetDefinitionOf
        + Default
        + From<String>
        + RemoveReduction
        + GetReduction
        + SetDefinition
        + GetDefinition
        + MaybeString,
{
}

pub trait ConceptMaker<T>
where
    T: From<String>
        + Default
        + SetReduction
        + GetDefinitionOf
        + GetDefinition
        + SetDefinition
        + MaybeString
        + GetReduction,
    Self: Labeller<T> + GetNormalForm<T>,
{
    fn concept_from_ast<U: MaybeConcept + MightExpand<U> + fmt::Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<usize> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let string = &ast.to_string();
            match ast.get_expansion() {
                None => self.new_labelled_abstract(string),
                Some((ref left, ref right)) => {
                    let mut leftc = try!(self.concept_from_ast(left));
                    let mut rightc = try!(self.concept_from_ast(right));
                    let concept = try!(self.find_or_insert_definition(leftc, rightc));
                    if !string.contains(' ') {
                        try!(self.label(concept, string));
                    }
                    Ok(concept)
                }
            }
        }
    }
}

impl<S, T> ConceptMaker<T> for S
where
    T: From<String>
        + Default
        + GetDefinitionOf
        + SetReduction
        + GetDefinition
        + SetDefinition
        + MaybeString
        + GetReduction,
    S: Labeller<T> + GetNormalForm<T>,
{
}

pub trait ContextMaker<T>
where
    Self: Labeller<T> + Default,
    T: GetDefinitionOf
        + From<String>
        + Default
        + SetReduction
        + GetDefinition
        + GetReduction
        + SetDefinition
        + MaybeString,
{
    fn new() -> Self {
        let mut cont = Self::default();
        cont.setup().unwrap();
        cont
    }
}

impl<S, T> ContextMaker<T> for S
where
    S: Labeller<T> + Default,
    T: GetDefinitionOf
        + From<String>
        + Default
        + SetReduction
        + GetDefinition
        + GetReduction
        + SetDefinition
        + MaybeString,
{
}

pub trait Labeller<T>
where
    T: SetReduction
        + From<String>
        + Default
        + GetDefinitionOf
        + SetDefinition
        + GetReduction
        + GetDefinition
        + GetReduction
        + MaybeString,
    Self: StringMaker<T> + FindOrInsertDefinition<T> + UpdateNormalForm<T>,
{
    fn label(&mut self, concept: usize, string: &str) -> ZiaResult<()> {
        let definition = try!(self.find_or_insert_definition(LABEL, concept));
        let string_id = self.new_string(string);
        self.update_normal_form(definition, string_id)
    }
    fn new_labelled_abstract(&mut self, string: &str) -> ZiaResult<usize> {
        let new_abstract = self.new_abstract();
        try!(self.label(new_abstract, string));
        Ok(new_abstract)
    }
    fn setup(&mut self) -> ZiaResult<()> {
        self.new_abstract(); // for LABEL
        let define_concept = self.new_abstract(); // for DEFINE;
        let reduction_concept = self.new_abstract(); // for REDUCTION
        try!(self.label(define_concept, ":=")); //two more ids occupied
        self.label(reduction_concept, "->") //two more ids occupied
    }
}

impl<S, T> Labeller<T> for S
where
    T: SetReduction
        + From<String>
        + Default
        + GetDefinitionOf
        + SetDefinition
        + GetReduction
        + GetDefinition
        + GetReduction
        + MaybeString,
    S: StringMaker<T> + FindOrInsertDefinition<T> + UpdateNormalForm<T>,
{
}

pub trait FindOrInsertDefinition<T>
where
    T: Default + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
    Self: AbstractMaker<T> + InsertDefinition<T> + FindDefinition<T>,
{
    fn find_or_insert_definition(&mut self, lefthand: usize, righthand: usize) -> ZiaResult<usize> {
        let pair = self.find_definition(lefthand, righthand);
        match pair {
            None => {
                let definition = self.new_abstract();
                try!(self.insert_definition(definition, lefthand, righthand));
                Ok(definition)
            }
            Some(def) => Ok(def),
        }
    }
}

impl<S, T> FindOrInsertDefinition<T> for S
where
    T: Default + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
    S: AbstractMaker<T> + InsertDefinition<T> + FindDefinition<T>,
{
}

pub trait StringMaker<T>
where
    T: From<String>,
    Self: ConceptAdder<T> + StringAdder,
{
    fn new_string(&mut self, string: &str) -> usize {
        let string_concept = string.to_string().into();
        let index = self.add_concept(string_concept);
        self.add_string(index, string);
        index
    }
}

impl<S, T> StringMaker<T> for S
where
    T: From<String>,
    S: ConceptAdder<T> + StringAdder,
{
}

pub trait AbstractMaker<T>
where
    T: Default,
    Self: ConceptAdder<T>,
{
    fn new_abstract(&mut self) -> usize {
        let concept = T::default();
        self.add_concept(concept)
    }
}

impl<S, T> AbstractMaker<T> for S
where
    T: Default,
    S: ConceptAdder<T>,
{
}

pub trait StringAdder {
    fn add_string(&mut self, usize, &str);
}

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, T) -> usize;
}
