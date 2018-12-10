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
use constants::LABEL;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
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
                    AbstractFactory, InsertDefinition, LabelConcept, Labeller,
                    StringFactory, UpdateNormalForm,
                },
                refactor::{delete_normal_form::DeleteReduction},
                ConceptNumber, MaybeDisconnected,
            },
            Container,
        },
        Call, GetNormalForm,
    },
    syntax_converter::{StringConcept, SyntaxConverter},
    GetId, SetId,
};

pub struct Context<T, V> {
    string_map: HashMap<String, Rc<RefCell<V>>>,
    concepts: Vec<T>,
}

impl<T, V> Context<T, V>
where
    T: InsertDefinition
        + UpdateNormalForm
        + GetDefinitionOf<T>
        + StringFactory
        + AbstractFactory
        + ConvertTo<Rc<RefCell<V>>>,
    V: Display,
{
    pub fn new() -> Context<T, V> {
        let mut cont = Context::<T, V> {
            string_map: HashMap::new(),
            concepts: Vec::new(),
        };
        cont.setup().unwrap();
        cont
    }
}

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

impl<T, V> Execute<T, V> for Context<T, V>
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
    V: Display,
{
}

pub trait StringAdder<V> {
    fn add_string(&mut self, string_ref: &Rc<RefCell<V>>);
}

impl<T, V> StringAdder<V> for Context<T, V>
where
    V: Display,
{
    fn add_string(&mut self, string_ref: &Rc<RefCell<V>>) {
        self.string_map
            .insert(string_ref.borrow().to_string(), string_ref.clone());
    }
}

pub trait ConceptHandler<T> {
	fn get_concept(&self, usize) -> T;
	fn remove_concept_by_id(&mut self, usize);
}

impl<T, V> ConceptHandler<T> for Context<T, V> 
where
	T: Clone,
{
	fn get_concept(&self, id: usize) -> T {
		self.concepts[id].clone()
	}
	fn remove_concept_by_id(&mut self, id: usize) {
		self.concepts.remove(id);		
	}
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

impl<T, V> ConceptNumber for Context<T, V> {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

pub trait BlindConceptAdder<T> {
	fn blindly_add_concept(&mut self, &T);
}

impl<T, V> BlindConceptAdder<T> for Context<T, V> 
where
	T: Clone,
{
	fn blindly_add_concept(&mut self, concept: &T) {
		self.concepts.push(concept.clone())
	}
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

impl<T, V> StringConcept<T> for Context<T, V>
where
    T: From<Rc<RefCell<V>>> + Clone,
{
    fn get_string_concept(&self, s: &str) -> Option<T> {
        match self.string_map.get(s) {
            None => None,
            Some(sr) => Some(sr.clone().into()),
        }
    }
}

impl<T, V> LabelConcept<T> for Context<T, V>
where
    T: Clone,
{
    fn get_label_concept(&self) -> T {
        self.concepts[LABEL].clone()
    }
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
