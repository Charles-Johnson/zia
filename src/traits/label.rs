use constants::LABEL;
use traits::base::{Application, Id, NormalForm};
use utils::{ZiaError, ZiaResult};

pub trait SyntaxFinder<T>
where
    T: Label<T> + Application<T> + Clone + Id,
{
    fn get_string_concept(&self, &str) -> Option<T>;
    fn concept_from_label(&self, s: &str) -> ZiaResult<Option<T>> {
        match self.get_string_concept(s) {
            None => Ok(None),
            Some(c) => c.get_labellee(),
        }
    }
}

pub trait Label<T>
where
    T: Application<T> + NormalForm<T> + Clone + Id,
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
