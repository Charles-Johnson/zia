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
use std::fmt;
use utils::{ZiaError, ZiaResult};

pub trait Application<T> {
    fn get_applicand_of(&self) -> Vec<T>;
    fn get_argument_of(&self) -> Vec<T>;
    fn get_definition(&self) -> Option<(T, T)>;
    fn set_definition(&mut self, &T, &T);
    fn add_applicand_of(&mut self, &T);
    fn add_argument_of(&mut self, &T);
    fn delete_definition(&mut self);
    fn delete_applicand_of(&mut self, &T);
    fn delete_argument_of(&mut self, &T);
}

pub trait Definition<T: Application<T> + Clone + PartialEq>
where
    Self: Application<T>,
{
    fn find_definition(&self, argument: &T) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for candidate in self.get_applicand_of() {
            let has_argument = argument.get_argument_of().contains(&candidate);
            let new_candidate = !candidates.contains(&candidate);
            if has_argument && new_candidate {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple definitions with the same applicand and argument pair 
				exist."
                    .to_string(),
            )),
        }
    }
}

pub trait NormalForm<T> {
    fn get_id(&self) -> usize;
    fn get_normal_form(&self) -> ZiaResult<Option<T>>;
    fn get_reduces_from(&self) -> Vec<T>;
    fn set_normal_form(&mut self, &T) -> ZiaResult<()>;
    fn add_reduces_from(&mut self, &T);
    fn remove_normal_form(&mut self);
    fn remove_reduces_from(&mut self, &T);
}

pub trait Reduction
where
    Self: NormalForm<Self> + Clone,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        try!(self.set_normal_form(normal_form));
        normal_form.add_reduces_from(self);
        Ok(())
    }
}

pub trait Label<T: Application<T> + NormalForm<T> + Clone>
where
    Self: NormalForm<T>,
{
    fn get_labellee(&self) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for label in self.get_reduces_from() {
            match label.get_definition() {
                None => continue,
                Some((r, x)) => if r.get_id() == LABEL {
                    candidates.push(x)
                } else {
                    continue;
                },
            };
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple concepts are labelled with the same string".to_string(),
            )),
        }
    }
}

pub trait Labeller<T: NormalForm<T> + fmt::Display> 
{
	fn get_label_concept(&self, &T) -> ZiaResult<Option<T>>;
	fn get_label(&self, concept: &T) -> ZiaResult<Option<String>> {
		Ok(match try!(self.get_label_concept(concept)) {
            None => None,
            Some(d) => match try!(d.get_normal_form()) {
                None => None,
                Some(n) => Some(n.to_string()),
            },
        })
	}
}
