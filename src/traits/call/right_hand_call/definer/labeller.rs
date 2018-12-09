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
use std::marker;
use traits::call::label_getter::FindDefinition;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::call::right_hand_call::Container;
use traits::call::{GetNormalForm, GetReduction};
use utils::{ZiaError, ZiaResult};

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, &T);
}

pub trait UpdateNormalForm
where
    Self: GetNormalForm + SetReduction<Self> + PartialEq,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        if let Some(n) = normal_form.get_normal_form() {
            if *self == n {
                return Err(ZiaError::CyclicReduction);
            }
        }
        if let Some(ref n) = self.get_reduction() {
            if n == normal_form {
                return Err(ZiaError::RedundantReduction);
            }
        }
        self.make_reduce_to(normal_form);
        normal_form.make_reduce_from(self);
        Ok(())
    }
}

impl<T> UpdateNormalForm for T where T: GetNormalForm + SetReduction<Self> + PartialEq {}

pub trait SetReduction<T> {
    fn make_reduce_to(&mut self, &T);
    fn make_reduce_from(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_as_lefthand_of(&mut self, &T);
    fn add_as_righthand_of(&mut self, &T);
}

pub trait InsertDefinition
where
    Self: SetDefinition<Self> + marker::Sized + Container + GetReduction<Self>,
{
    fn insert_definition(&mut self, lefthand: &mut Self, righthand: &mut Self) -> ZiaResult<()> {
        if lefthand.contains(self) || righthand.contains(self) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.check_reductions(lefthand));
            try!(self.check_reductions(righthand));
            self.set_definition(lefthand, righthand);
            lefthand.add_as_lefthand_of(self);
            righthand.add_as_righthand_of(self);
            Ok(())
        }
    }
    fn check_reductions(&self, concept: &Self) -> ZiaResult<()> {
        if let Some(ref r) = concept.get_reduction() {
            if r == self || r.contains(self) {
                Err(ZiaError::ExpandingReduction)
            } else {
                self.check_reductions(r)
            }
        } else {
            Ok(())
        }
    }
}

impl<T> InsertDefinition for T where
    T: SetDefinition<T> + marker::Sized + Container + GetReduction<Self>
{
}

pub trait Labeller<T>
where
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + GetDefinitionOf<T>,
    Self: StringMaker<T> + FindOrInsertDefinition<T> + LabelConcept<T>,
{
    fn label(&mut self, concept: &mut T, string: &str) -> ZiaResult<()> {
        let mut label_concept = self.get_label_concept();
        let mut definition = try!(self.find_or_insert_definition(&mut label_concept, concept));
        let mut string_ref = self.new_string(string);
        definition.update_normal_form(&mut string_ref)
    }
    fn new_labelled_abstract(&mut self, string: &str) -> ZiaResult<T> {
        let mut new_abstract = self.new_abstract();
        try!(self.label(&mut new_abstract, string));
        Ok(new_abstract)
    }
    fn setup(&mut self) -> ZiaResult<()> {
        self.new_abstract(); // for LABEL
        let mut define_concept = self.new_abstract(); // for DEFINE;
        let mut reduction_concept = self.new_abstract(); // for REDUCTION
        try!(self.label(&mut define_concept, ":=")); //two more ids occupied
        self.label(&mut reduction_concept, "->") //two more ids occupied
    }
}

pub trait LabelConcept<T> {
    fn get_label_concept(&self) -> T;
}

impl<S, T> Labeller<T> for S
where
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + GetDefinitionOf<T>,
    S: StringMaker<T> + FindOrInsertDefinition<T> + LabelConcept<T>,
{
}

pub trait StringMaker<T>
where
    T: StringFactory,
    Self: ConceptAdder<T> + ConceptNumber,
{
    fn new_string(&mut self, string: &str) -> T {
        let new_id = self.number_of_concepts();
        let string_ref = T::new_string(new_id, string);
        self.add_concept(&string_ref);
        string_ref
    }
}

impl<S, T> StringMaker<T> for S
where
    T: StringFactory,
    S: ConceptAdder<T> + ConceptNumber,
{
}

pub trait FindOrInsertDefinition<T>
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    Self: AbstractMaker<T>,
{
    fn find_or_insert_definition(&mut self, lefthand: &mut T, righthand: &mut T) -> ZiaResult<T> {
        let application = lefthand.find_definition(righthand);
        match application {
            None => {
                let mut definition = self.new_abstract();
                try!(definition.insert_definition(lefthand, righthand));
                Ok(definition.clone())
            }
            Some(def) => Ok(def),
        }
    }
}

impl<S, T> FindOrInsertDefinition<T> for S
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    S: AbstractMaker<T>,
{
}

pub trait AbstractMaker<T>
where
    T: AbstractFactory,
    Self: ConceptAdder<T> + ConceptNumber,
{
    fn new_abstract(&mut self) -> T {
        let new_id = self.number_of_concepts();
        let concept_ref = T::new_abstract(new_id);
        self.add_concept(&concept_ref);
        concept_ref
    }
}

impl<S, T> AbstractMaker<T> for S
where
    T: AbstractFactory,
    S: ConceptAdder<T> + ConceptNumber,
{
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}
