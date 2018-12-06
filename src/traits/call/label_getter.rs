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
use std::fmt::Display;
use traits::call::{GetNormalForm, MaybeConcept};
use traits::{GetDefinition, Id};
use utils::ZiaResult;

pub trait LabelGetter
where
    Self: Id
        + GetNormalForm<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + Clone
        + PartialEq
        + MaybeString,
{
    fn get_concept_of_label(&self) -> Option<Self> {
        for candidate in self.get_righthand_of() {
            match candidate.get_definition() {
                None => panic!("Candidate should have a definition!"),
                Some((ref left, _)) => if left.get_id() == LABEL {
                    return Some(candidate.clone());
                },
            };
        }
        None
    }
    fn get_label(&self) -> ZiaResult<Option<String>> {
        Ok(match self.get_concept_of_label() {
            None => None,
            Some(d) => match try!(d.get_normal_form()) {
                None => None,
                Some(n) => Some(n.get_string()),
            },
        })
    }
}

impl<T> LabelGetter for T where
    T: Id
        + GetNormalForm<T>
        + GetDefinition<T>
        + GetDefinitionOf<T>
        + Clone
        + PartialEq
        + Display
        + MaybeString
{}

pub trait MaybeString {
    fn get_string(&self) -> String;
}

pub trait FindDefinition<T>
where
    T: GetDefinitionOf<T> + Clone + PartialEq,
    Self: GetDefinitionOf<T>,
{
    fn find_definition(&self, righthand: &T) -> Option<T> {
        let mut candidates: Vec<T> = Vec::new();
        for candidate in self.get_lefthand_of() {
            let has_righthand = righthand.get_righthand_of().contains(&candidate);
            let new_candidate = !candidates.contains(&candidate);
            if has_righthand && new_candidate {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => None,
            1 => Some(candidates[0].clone()),
            _ => panic!("Multiple definitions with the same lefthand and righthand pair exist."),
        }
    }
}

impl<S, T> FindDefinition<T> for S
where
    T: GetDefinitionOf<T> + Clone + PartialEq,
    S: GetDefinitionOf<T>,
{}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}

impl<T, U> GetDefinitionOf<T> for U
where
    T: GetDefinitionOf<T>,
    U: MaybeConcept<T>,
{
    fn get_lefthand_of(&self) -> Vec<T> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_lefthand_of(),
        }
    }
    fn get_righthand_of(&self) -> Vec<T> {
        match self.get_concept() {
            None => Vec::new(),
            Some(c) => c.get_righthand_of(),
        }
    }
}
