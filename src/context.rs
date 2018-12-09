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
use ast::AbstractSyntaxTree;
use concepts::{ConvertTo, Display};
use constants::LABEL;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use traits::call::right_hand_call::definer::labeller::{
    AbstractFactory, ConceptAdder, InsertDefinition, LabelConcept, Labeller, SetDefinition,
    SetReduction, StringFactory, UpdateNormalForm,
};
use traits::call::right_hand_call::definer::refactor::{
    delete_normal_form::DeleteReduction, refactor_id::ConceptTidyer,
};
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::call::right_hand_call::definer::{
    concept_maker::ConceptMaker, delete_definition::DeleteDefinition, MaybeDisconnected,
};
use traits::call::{
    label_getter::{GetDefinitionOf, MaybeString},
    reduce::SyntaxFromConcept,
    Call, GetReduction,
};
use traits::syntax_converter::{label::Label, StringConcept, SyntaxConverter};
use traits::{GetDefinition, GetId, SetId};

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

pub trait Execute {
    fn execute(&mut self, command: &str) -> String;
}

impl<T, V> Execute for Context<T, V>
where
    T: Label
        + GetDefinitionOf<T>
        + PartialEq
        + AbstractFactory
        + StringFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept<AbstractSyntaxTree<T>>
        + MaybeDisconnected
        + Display
        + From<Rc<RefCell<V>>>
        + ConvertTo<Rc<RefCell<V>>>
        + SetId,
    V: Display,
{
    fn execute(&mut self, command: &str) -> String {
        let ast = match self.ast_from_expression(command) {
            Ok(a) => a,
            Err(e) => return e.to_string(),
        };
        match self.call(&ast) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
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

impl<T, V> ConceptTidyer<T> for Context<T, V>
where
    T: SetId + GetId,
{
    fn remove_concept(&mut self, concept: &T) {
        self.concepts.remove(concept.get_id());
    }
    fn correct_id(&mut self, id: usize) {
        self.concepts[id].set_id(id);
    }
}

impl<T, V> ConceptNumber for Context<T, V> {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

impl<T, V> ConceptAdder<T> for Context<T, V>
where
    T: ConvertTo<Rc<RefCell<V>>> + Clone,
    V: Display,
{
    fn add_concept(&mut self, concept: &T) {
        self.concepts.push(concept.clone());
        if let Some(ref sr) = concept.convert() {
            self.add_string(sr);
        }
    }
}

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

impl<T, V> ConceptMaker<T, AbstractSyntaxTree<T>> for Context<T, V>
where
    T: GetDefinition<T>
        + ConvertTo<Rc<RefCell<V>>>
        + From<Rc<RefCell<V>>>
        + StringFactory
        + AbstractFactory
        + SetDefinition<T>
        + GetReduction<T>
        + Clone
        + SetReduction<T>
        + PartialEq
        + GetDefinitionOf<T>
        + MaybeString,
    V: Display,
{
}
