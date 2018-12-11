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
use utils::{ZiaError, ZiaResult};

pub trait Label
where
    Self: FindWhatItsANormalFormOf + GetDefinition<Self> + Clone + GetId,
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

impl<S> Label for S where S: GetDefinition<S> + FindWhatItsANormalFormOf + Clone + GetId {}

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

impl<T> FindWhatItsANormalFormOf for T where T: FindWhatReducesToIt<T> + Sized + Clone {}

pub trait Unlabeller
where
    Self: LabelGetter + DeleteReduction,
{
    fn unlabel(&mut self) {
        match self.get_concept_of_label() {
            None => panic!("No label to remove"),
            Some(mut d) => d.delete_reduction(),
        }
    }
}

impl<S> Unlabeller for S where S: LabelGetter + DeleteReduction {}

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
{
}

pub trait LabelGetter
where
    Self: GetId + GetNormalForm + GetDefinition<Self> + GetDefinitionOf<Self> + Clone + MaybeString,
{
    fn get_concept_of_label(&self) -> Option<Self> {
        for candidate in self.get_righthand_of() {
            match candidate.get_definition() {
                None => panic!("Candidate should have a definition!"),
                Some((ref left, _)) => {
                    if left.get_id() == LABEL {
                        return Some(candidate.clone());
                    }
                }
            };
        }
        None
    }
    fn get_label(&self) -> Option<String> {
        match self.get_concept_of_label() {
            None => None,
            Some(d) => match d.get_normal_form() {
                None => None,
                Some(n) => n.get_string(),
            },
        }
    }
}

impl<T> LabelGetter for T where
    T: GetId + GetNormalForm + GetDefinition<T> + GetDefinitionOf<T> + Clone + MaybeString
{
}

pub trait DeleteReduction
where
    Self: GetReduction<Self> + RemoveReduction<Self> + Sized,
{
    fn delete_reduction(&mut self) {
        match self.get_reduction() {
            None => panic!("No normal form to delete"),
            Some(mut n) => {
                n.no_longer_reduces_from(self);
                self.make_reduce_to_none();
            }
        };
    }
}

impl<T> DeleteReduction for T where T: GetReduction<T> + RemoveReduction<T> {}

pub trait UpdateNormalForm
where
    Self: GetNormalForm + SetReduction<Self> + PartialEq,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        if let Some(n) = normal_form.get_normal_form() {
            if *self == n {
                return Err(ZiaError::CyclicReduction);
            }
        }
        if let Some(ref n) = self.get_reduction() {
            if n == normal_form {
                return Err(ZiaError::RedundantReduction);
            }
        }
        self.make_reduce_to(normal_form);
        normal_form.make_reduce_from(self);
        Ok(())
    }
}

impl<T> UpdateNormalForm for T where T: GetNormalForm + SetReduction<Self> + PartialEq {}

pub trait DeleteDefinition
where
    Self: GetDefinition<Self> + RemoveDefinition<Self> + Sized,
{
    fn delete_definition(&mut self) {
        match self.get_definition() {
            None => panic!("No definition to remove!"),
            Some((mut app, mut arg)) => {
                app.remove_as_lefthand_of(self);
                arg.remove_as_righthand_of(self);
                self.remove_definition();
            }
        };
    }
}

impl<T> DeleteDefinition for T where T: GetDefinition<T> + RemoveDefinition<T> + Sized {}

pub trait MaybeDisconnected
where
    Self: GetReduction<Self>
        + FindWhatReducesToIt<Self>
        + GetDefinition<Self>
        + GetDefinitionOf<Self>
        + GetId
        + Sized,
{
    fn is_disconnected(&self) -> bool {
        self.get_reduction().is_none()
            && self.get_definition().is_none()
            && self.get_lefthand_of().is_empty()
            && self.righthand_of_without_label_is_empty()
            && self.find_what_reduces_to_it().is_empty()
    }
    fn righthand_of_without_label_is_empty(&self) -> bool {
        for concept in self.get_righthand_of() {
            if let Some((left, _)) = concept.get_definition() {
                if left.get_id() != LABEL {
                    return false;
                }
            }
        }
        true
    }
}

impl<T> MaybeDisconnected for T where
    T: GetReduction<T> + FindWhatReducesToIt<T> + GetDefinition<T> + GetDefinitionOf<T> + GetId
{
}

pub trait GetNormalForm
where
    Self: GetReduction<Self> + Sized + Clone,
{
    fn get_normal_form(&self) -> Option<Self> {
        match self.get_reduction() {
            None => None,
            Some(ref n) => match n.get_normal_form() {
                None => Some(n.clone()),
                Some(ref m) => Some(m.clone()),
            },
        }
    }
}

impl<S> GetNormalForm for S where S: GetReduction<S> + Sized + Clone {}

pub trait GetId {
    fn get_id(&self) -> usize;
}

pub trait SetId {
    fn set_id(&mut self, id: usize);
}

pub trait GetDefinition<T> {
    fn get_definition(&self) -> Option<(T, T)>;
}

pub trait GetReduction<T> {
    fn get_reduction(&self) -> Option<T>;
}

pub trait FindWhatReducesToIt<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T>;
}

pub trait RemoveReduction<T> {
    fn make_reduce_to_none(&mut self);
    fn no_longer_reduces_from(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_as_lefthand_of(&mut self, &T);
    fn add_as_righthand_of(&mut self, &T);
}

pub trait SetReduction<T> {
    fn make_reduce_to(&mut self, &T);
    fn make_reduce_from(&mut self, &T);
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_as_lefthand_of(&mut self, &T);
    fn remove_as_righthand_of(&mut self, &T);
}

pub trait MaybeString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}

pub trait ConvertTo<T> {
    fn convert(&self) -> Option<T>;
}
