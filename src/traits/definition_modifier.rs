use std::marker;
use traits::base::{GetDefinition, GetDefinitionOf, RemoveDefinition, SetDefinition};
use utils::{ZiaError, ZiaResult};

pub trait InsertDefinition
where
    Self: SetDefinition<Self> + marker::Sized,
{
    fn insert_definition(&mut self, lefthand: &mut Self, righthand: &mut Self) {
        self.set_definition(lefthand, righthand);
        lefthand.add_lefthand_of(self);
        righthand.add_righthand_of(self);
    }
}
pub trait DeleteDefinition
where
    Self: GetDefinition<Self> + RemoveDefinition<Self> + marker::Sized,
{
    fn delete_definition(&mut self) {
        match self.get_definition() {
            None => panic!("No definition to remove!"),
            Some((mut app, mut arg)) => {
                app.remove_lefthand_of(self);
                arg.remove_righthand_of(self);
                self.remove_definition();
            }
        };
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
