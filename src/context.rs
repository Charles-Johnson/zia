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
use concepts::string_concept::StringRef;
use concepts::{Display, ConvertTo};
use constants::LABEL;
use std::collections::HashMap;
use traits::call::right_hand_call::definer::{MaybeDisconnected, concept_maker::ConceptMaker, delete_definition::DeleteDefinition};
use traits::call::right_hand_call::definer::labeller::{ConceptAdder, LabelConcept, Labeller, StringFactory, AbstractFactory, SetDefinition, SetReduction, UpdateNormalForm, InsertDefinition};
use traits::call::right_hand_call::definer::refactor::{delete_normal_form::DeleteReduction, refactor_id::ConceptTidyer};
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::call::{Call, GetReduction, label_getter::{GetDefinitionOf, MaybeString}, reduce::SyntaxFromConcept};
use traits::syntax_converter::{SyntaxConverter, StringConcept, label::Label};
use traits::{GetId, SetId, GetDefinition};

pub struct Context<T> {
    string_map: HashMap<String, StringRef<T>>,
    concepts: Vec<T>,
}

impl<T> Context<T> 
where
	T: InsertDefinition
		+ UpdateNormalForm
		+ GetDefinitionOf<T>
		+ StringFactory
		+ AbstractFactory
		+ ConvertTo<StringRef<T>>,
{
    pub fn new() -> Context<T> {
        let mut cont = Context::<T> {
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

impl<T> Execute for Context<T> 
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
		+ From<StringRef<T>>
		+ ConvertTo<StringRef<T>>
		+ SetId,
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

pub trait StringAdder<T> {
	fn add_string(&mut self, string_ref: &StringRef<T>);
}

impl<T> StringAdder<T> for Context<T> {
    fn add_string(&mut self, string_ref: &StringRef<T>) {
        self.string_map
            .insert(string_ref.borrow().to_string(), string_ref.clone());
    }
}

impl<T: SetId + GetId> ConceptTidyer<T> for Context<T> {
    fn remove_concept(&mut self, concept: &T) {
        self.concepts.remove(concept.get_id());
    }
    fn correct_id(&mut self, id: usize) {
        self.concepts[id].set_id(id);
    }
}

impl<T> ConceptNumber for Context<T> {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

impl<T> ConceptAdder<T> for Context<T> 
where
	T: ConvertTo<StringRef<T>> + Clone,
{
    fn add_concept(&mut self, concept: &T) {
        self.concepts.push(concept.clone());
        if let Some(ref sr) = concept.convert() {
            self.add_string(sr);
        }
    }
}

impl<T> StringConcept<T> for Context<T> 
where
	T: From<StringRef<T>>
		+ Clone,
{
    fn get_string_concept(&self, s: &str) -> Option<T> {
        match self.string_map.get(s) {
			None => None,
			Some(sr) => Some(T::from(sr.clone()))
		}
    }
}

impl<T: Clone> LabelConcept<T> for Context<T> {
    fn get_label_concept(&self) -> T {
        self.concepts[LABEL].clone()
    }
}

impl<T> ConceptMaker<T, AbstractSyntaxTree<T>> for Context<T> 
where
	T: GetDefinition<T>
		+ ConvertTo<StringRef<T>>
		+ From<StringRef<T>> 
		+ StringFactory 
		+ AbstractFactory 
		+ SetDefinition<T> 
		+ GetReduction<T> 
		+ Clone 
		+ SetReduction<T>
		+ PartialEq
		+ GetDefinitionOf<T>
		+ MaybeString,
{}
