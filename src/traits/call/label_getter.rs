use std::fmt;
use traits::GetNormalForm;
use utils::{ZiaError, ZiaResult};

pub trait LabelGetter<T>
where
    T: GetNormalForm<T> + FindDefinition<T> + Clone + PartialEq + fmt::Display,
{
    fn get_label_concept(&self) -> T;
    fn get_concept_of_label(&self, concept: &T) -> ZiaResult<Option<T>> {
        self.get_label_concept().find_definition(concept)
    }
    fn get_label(&self, concept: &T) -> ZiaResult<Option<String>> {
        Ok(match try!(self.get_concept_of_label(concept)) {
            None => None,
            Some(d) => match try!(d.get_normal_form()) {
                None => None,
                Some(n) => Some(n.to_string()),
            },
        })
    }
}

pub trait FindDefinition<T>
where
    T: GetDefinitionOf<T> + Clone + PartialEq,
    Self: GetDefinitionOf<T>,
{
    fn find_definition(&self, righthand: &T) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for candidate in self.get_lefthand_of() {
            let has_righthand = righthand.get_righthand_of().contains(&candidate);
            let new_candidate = !candidates.contains(&candidate);
            if has_righthand && new_candidate {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple definitions with the same lefthand and righthand pair 
				exist."
                    .to_string(),
            )),
        }
    }
}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}
