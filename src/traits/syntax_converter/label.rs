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
use constants::LABEL;
use std::marker::Sized;
use traits::call::FindWhatReducesToIt;
use traits::{GetDefinition, Id};

pub trait Label
where
    Self: FindWhatItsANormalFormOf + GetDefinition<Self> + Clone + Id,
{
    fn get_labellee(&self) -> Option<Self> {
        let mut candidates: Vec<Self> = Vec::new();
        for label in self.find_what_its_a_normal_form_of() {
            match label.get_definition() {
                None => continue,
                Some((r, x)) => {
                    if r.get_id() == LABEL {
                        candidates.push(x)
                    } else {
                        continue;
                    }
                }
            };
        }
        match candidates.len() {
            0 => None,
            1 => Some(candidates[0].clone()),
            _ => panic!("Multiple concepts are labelled with the same string"),
        }
    }
}

impl<S> Label for S
where
    S: GetDefinition<S> + FindWhatItsANormalFormOf + Clone + Id,
{
}

pub trait FindWhatItsANormalFormOf 
where
	Self: FindWhatReducesToIt<Self> + Sized + Clone,
{
    fn find_what_its_a_normal_form_of(&self) -> Vec<Self> {
        let mut normal_form_of: Vec<Self> = Vec::new();
        for concept in self.find_what_reduces_to_it() {
            normal_form_of.push(concept.clone());
            for concept2 in concept.find_what_its_a_normal_form_of() {
                normal_form_of.push(concept2);
            }
        }
        normal_form_of		
	}
}

impl<T> FindWhatItsANormalFormOf for T
where
	T: FindWhatReducesToIt<T> + Sized + Clone,
{}
