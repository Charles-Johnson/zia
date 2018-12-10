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

pub mod traits;

use concepts::{ConvertTo, Display};
use constants::LABEL;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use traits::{
    call::{
        label_getter::GetDefinitionOf,
        right_hand_call::{
            definer::{
                labeller::{
                    AbstractFactory, InsertDefinition, LabelConcept, Labeller,
                    StringFactory, UpdateNormalForm,
                },
                ConceptNumber,
            },
        },
    },
    syntax_converter::StringConcept,
};

use self::traits::{StringAdder, ConceptHandler, BlindConceptAdder};

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

impl<T, V> StringAdder<V> for Context<T, V>
where
    V: Display,
{
    fn add_string(&mut self, string_ref: &Rc<RefCell<V>>) {
        self.string_map
            .insert(string_ref.borrow().to_string(), string_ref.clone());
    }
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

impl<T, V> ConceptNumber for Context<T, V> {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

impl<T, V> BlindConceptAdder<T> for Context<T, V> 
where
	T: Clone,
{
	fn blindly_add_concept(&mut self, concept: &T) {
		self.concepts.push(concept.clone())
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
