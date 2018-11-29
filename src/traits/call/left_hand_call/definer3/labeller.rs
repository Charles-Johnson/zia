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
use traits::call::label_getter::{FindDefinition, LabelGetter};
use traits::call::left_hand_call::definer3::ConceptNumber;
use traits::call::MaybeConcept;
use utils::ZiaResult;

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, &T);
}

pub trait UpdateNormalForm
where
    Self: SetNormalForm<Self>,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        try!(self.set_normal_form(normal_form));
        normal_form.add_normal_form_of(self);
        Ok(())
    }
}

impl<T> UpdateNormalForm for T where T: SetNormalForm<T> {}

pub trait SetNormalForm<T>
where
    Self: marker::Sized,
{
    fn set_normal_form(&mut self, &T) -> ZiaResult<()>;
    fn add_normal_form_of(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_lefthand_of(&mut self, &T);
    fn add_righthand_of(&mut self, &T);
}

impl<T,U> SetDefinition<T> for U 
where
	T: SetDefinition<T>,
	U: MaybeConcept<T>,
{
    fn set_definition(&mut self, lefthand: &T, righthand: &T) {
        if let Some(mut c) = self.get_concept() {
            c.set_definition(lefthand, righthand)
        }
    }
    fn add_lefthand_of(&mut self, concept: &T) {
        if let Some(mut c) = self.get_concept() {
            c.add_lefthand_of(concept)
        }
    }
    fn add_righthand_of(&mut self, concept: &T) {
        if let Some(mut c) = self.get_concept() {
            c.add_righthand_of(concept)
        }
    }
}

pub trait InsertDefinition
where
    Self: SetDefinition<Self> + marker::Sized,
{
    fn insert_definition(&mut self, lefthand: &mut Self, righthand: &mut Self) {
        self.set_definition(lefthand, righthand);
        lefthand.add_lefthand_of(self);
        righthand.add_righthand_of(self);
    }
}

impl<T> InsertDefinition for T where T: SetDefinition<T> + marker::Sized {}

pub trait Labeller<T>
where
    T: StringFactory + AbstractFactory + LabelGetter + InsertDefinition + UpdateNormalForm,
    Self: StringMaker<T> + Definer<T> + LabelConcept<T>,
{
    fn label(&mut self, concept: &mut T, string: &str) -> ZiaResult<()> {
        let mut label_concept = self.get_label_concept();
        let mut definition = try!(self.insert_definition(&mut label_concept, concept));
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
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + LabelGetter,
    S: StringMaker<T> + Definer<T> + LabelConcept<T>,
{}

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

pub trait Definer<T>
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    Self: AbstractMaker<T>,
{
    fn insert_definition(&mut self, lefthand: &mut T, righthand: &mut T) -> ZiaResult<T> {
        let application = try!(lefthand.find_definition(righthand));
        match application {
            None => {
                let mut definition = self.new_abstract();
                definition.insert_definition(lefthand, righthand);
                Ok(definition.clone())
            }
            Some(def) => Ok(def),
        }
    }
}

impl<S, T> Definer<T> for S
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    S: AbstractMaker<T>,
{}

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
{}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}
