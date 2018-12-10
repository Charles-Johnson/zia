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
use concepts::{ConvertTo, Display};
use std::{cell::RefCell, rc::Rc};
use traits::{
    call::{
        expander::Expander,
        label_getter::GetDefinitionOf,
        reduce::{Reduce, SyntaxFromConcept},
        right_hand_call::{
            definer::{
                concept_maker::ConceptMaker,
                delete_definition::DeleteDefinition,
                labeller::{
                    AbstractFactory, InsertDefinition, Labeller,
                    StringFactory, UpdateNormalForm,
                },
                refactor::{delete_normal_form::DeleteReduction},
                MaybeDisconnected,
            },
            Container,
        },
        Call, GetNormalForm,
    },
    syntax_converter::SyntaxConverter,
    GetId, SetId,
};

pub trait Execute<T, V>
where
    Self: Call<T, V> + SyntaxConverter<T>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>,
{
    fn execute<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        command: &str,
    ) -> String {
        let ast = match self.ast_from_expression::<U>(command) {
            Ok(a) => a,
            Err(e) => return e.to_string(),
        };
        match self.call(&ast) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
}

impl<S, T, V> Execute<T, V> for S
where
    T: AbstractFactory
        + StringFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
        + From<Rc<RefCell<V>>>
        + ConvertTo<Rc<RefCell<V>>>
        + SetId,
    S: Call<T, V> + SyntaxConverter<T>,
{
}


impl<S, T, V> ConceptMaker<T, V> for S
where
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + GetNormalForm
        + UpdateNormalForm
        + GetDefinitionOf<T>
		+ ConvertTo<Rc<RefCell<V>>>,
    S: Labeller<T, V>,
{
}

pub trait StringAdder<V> {
    fn add_string(&mut self, string_ref: &Rc<RefCell<V>>);
}

pub trait ConceptTidyer<T>
where
    T: SetId + GetId,
	Self: ConceptHandler<T>,
{
    fn remove_concept(&mut self, concept: &T) {
        self.remove_concept_by_id(concept.get_id());
    }
    fn correct_id(&mut self, id: usize) {
        self.get_concept(id).set_id(id);
    }
}

impl<S, T> ConceptTidyer<T> for S 
where
	T: SetId + GetId,
	S: ConceptHandler<T>,
{
}

pub trait ConceptAdder<T, V> 
where
	Self: BlindConceptAdder<T> + StringAdder<V>,
	T: ConvertTo<Rc<RefCell<V>>>, 
{
    fn add_concept(&mut self, concept: &T) {
        self.blindly_add_concept(concept);
        if let Some(ref sr) = concept.convert() {
            self.add_string(sr);
        }
	}
}

impl<S, T, V> ConceptAdder<T, V> for S
where
	S: BlindConceptAdder<T> + StringAdder<V>,
	T: ConvertTo<Rc<RefCell<V>>>, 
{}

pub trait BlindConceptAdder<T> {
	fn blindly_add_concept(&mut self, &T);
}

pub trait ConceptHandler<T> {
	fn get_concept(&self, usize) -> T;
	fn remove_concept_by_id(&mut self, usize);
}
