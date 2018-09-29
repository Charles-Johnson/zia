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
use utils::{ZiaError, ZiaResult};

pub trait Application<T> {
    fn get_applicand_of(&self) -> Vec<T>;
    fn get_argument_of(&self) -> Vec<T>;
    fn get_definition(&self) -> Option<(T, T)>;
    fn set_definition(&mut self, applicand: &T, argument: &T);
    fn add_applicand_of(&mut self, applicand: &T);
    fn add_argument_of(&mut self, argument: &T); 
}

pub trait Definition<T: Application<T> + Clone + PartialEq> where Self: Application<T>{
    fn find_definition(&self, argument: &T) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for candidate in self.get_applicand_of() {
            if argument.get_argument_of().contains(&candidate) && !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple definitions with the same applicand and argument pair exist.".to_string(),
            )),
        }
    }
}

pub trait NormalForm<T> {
    fn get_id(&self) -> usize;
    fn get_normal_form(&self) -> Option<T>;
    fn get_reduces_from(&self) -> Vec<T>;
    fn set_normal_form(&mut self, &T);
    fn add_reduces_from(&mut self, &T);
    fn remove_normal_form(&mut self);
    fn remove_reduces_from(&mut self, &T);
}

pub trait Reduction where Self: NormalForm<Self> + Clone {
    fn insert_reduction(
        &mut self,
        normal_form: &mut Self,
    ) -> ZiaResult<()> {
        match self.get_normal_form() {
            None => (),
            Some(_) => {
                return Err(ZiaError::Redundancy(
                    "Reduction rule already exists for concept".to_string()
                ))
            }
        };
        let mut new_normal_form = normal_form.clone();
        match normal_form.get_normal_form() {
            None => (),
            Some(n) => 
                if n.get_id() != self.get_id() {
                    new_normal_form = n.clone()
                } else {
                    return Err(ZiaError::Loop("Cannot create a reduction loop".to_string()))
                },
        };
        let prereductions = self.get_reduces_from();
        for mut prereduction in prereductions {
            try!(prereduction.update_normal_form(
                &mut new_normal_form,
            ));
        }
        self.insert_normal_form(&mut new_normal_form)
    }
    fn insert_normal_form(
        &mut self,
        normal_form: &mut Self,
    ) -> ZiaResult<()> {
        match self.get_normal_form() {
            None => {
                for reduces_from_item in normal_form.get_reduces_from() {
                    if reduces_from_item.get_id() == self.get_id() {
                        return Err(ZiaError::Redundancy(
                            "Normal form already reduces from this concept".to_string(),
                        ));
                    }
                }
                self.update_normal_form(normal_form)
            }
            Some(_) => Err(ZiaError::Ambiguity(
                "Normal form already exists for this concept".to_string(),
            )),
        }
    }
    fn update_normal_form(
        &mut self,
        normal_form: &mut Self,
    ) -> ZiaResult<()> {
        normal_form.add_reduces_from(self);
        self.set_normal_form(normal_form);
        Ok(())
    }
}

pub trait Label<T: Application<T> + NormalForm<T> + Clone> where Self: NormalForm<T> {
    fn get_labellee(&self) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for label in self.get_reduces_from() {
            match label.get_definition() {
                None => continue,
                Some((r, x)) => {
                    match r.get_id() {
                        LABEL => candidates.push(x),
                        _ => continue,
                    };
                }
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
