pub use concepts::{RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};
use context::ConceptWriter;
use reading::{
    ConceptReader, Container, GetConceptOfLabel, GetDefinition, GetDefinitionOf, GetNormalForm,
    GetReduction, MaybeConcept,
};
use utils::{ZiaError, ZiaResult};
pub trait Unlabeller<T>
where
    T: GetReduction + RemoveReduction + GetDefinition + GetDefinitionOf,
    Self: DeleteReduction<T> + GetConceptOfLabel<T>,
{
    fn unlabel(&mut self, concept: usize) {
        match self.get_concept_of_label(concept) {
            None => panic!("No label to remove"),
            Some(d) => self.delete_reduction(d),
        }
    }
}

impl<S, T> Unlabeller<T> for S
where
    T: GetReduction + RemoveReduction + GetDefinitionOf + GetDefinition,
    S: DeleteReduction<T> + GetConceptOfLabel<T>,
{
}

pub trait DeleteReduction<T>
where
    T: GetReduction + RemoveReduction,
    Self: ConceptWriter<T> + ConceptReader<T>,
{
    fn try_removing_reduction<U: MaybeConcept>(&mut self, syntax: &U) -> ZiaResult<()> {
        if let Some(c) = syntax.get_concept() {
            self.delete_reduction(c);
            Ok(())
        } else {
            Err(ZiaError::RedundantReduction)
        }
    }
    fn delete_reduction(&mut self, concept: usize) {
        match self.read_concept(concept).get_reduction() {
            None => panic!("No normal form to delete"),
            Some(n) => {
                self.write_concept(n).no_longer_reduces_from(concept);
                self.write_concept(concept).make_reduce_to_none();
            }
        };
    }
}

impl<S, T> DeleteReduction<T> for S
where
    S: ConceptWriter<T> + ConceptReader<T>,
    T: GetReduction + RemoveReduction,
{
}

pub trait DeleteDefinition<T>
where
    T: GetDefinition + RemoveDefinition + Sized,
    Self: ConceptReader<T> + ConceptWriter<T>,
{
    fn delete_definition(&mut self, concept: usize) {
        match self.read_concept(concept).get_definition() {
            None => panic!("No definition to remove!"),
            Some((left, right)) => {
                self.write_concept(left).remove_as_lefthand_of(concept);
                self.write_concept(right).remove_as_righthand_of(concept);
                self.write_concept(concept).remove_definition();
            }
        };
    }
}

impl<S, T> DeleteDefinition<T> for S
where
    T: GetDefinition + RemoveDefinition + Sized,
    S: ConceptReader<T> + ConceptWriter<T>,
{
}

pub trait UpdateNormalForm<T>
where
    T: SetReduction + GetReduction,
    Self: ConceptWriter<T> + GetNormalForm<T>,
{
    fn update_normal_form(&mut self, concept: usize, normal_form: usize) -> ZiaResult<()> {
        if let Some(n) = self.get_normal_form(normal_form) {
            if concept == n {
                return Err(ZiaError::CyclicReduction);
            }
        }
        if let Some(n) = self.read_concept(concept).get_reduction() {
            if n == normal_form {
                return Err(ZiaError::RedundantReduction);
            }
        }
        self.write_concept(concept).make_reduce_to(normal_form);
        self.write_concept(normal_form).make_reduce_from(concept);
        Ok(())
    }
}

impl<S, T> UpdateNormalForm<T> for S
where
    T: SetReduction + GetReduction,
    S: ConceptWriter<T> + GetNormalForm<T>,
{
}

pub trait InsertDefinition<T>
where
    T: SetDefinition + Sized + GetDefinition + GetReduction,
    Self: ConceptWriter<T> + Container<T>,
{
    fn insert_definition(
        &mut self,
        definition: usize,
        lefthand: usize,
        righthand: usize,
    ) -> ZiaResult<()> {
        if self.contains(lefthand, definition) || self.contains(righthand, definition) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.check_reductions(definition, lefthand));
            try!(self.check_reductions(definition, righthand));
            self.write_concept(definition)
                .set_definition(lefthand, righthand);
            self.write_concept(lefthand).add_as_lefthand_of(definition);
            self.write_concept(righthand)
                .add_as_righthand_of(definition);
            Ok(())
        }
    }
    fn check_reductions(&self, outer_concept: usize, inner_concept: usize) -> ZiaResult<()> {
        if let Some(r) = self.read_concept(inner_concept).get_reduction() {
            if r == outer_concept || self.contains(r, outer_concept) {
                Err(ZiaError::ExpandingReduction)
            } else {
                self.check_reductions(outer_concept, r)
            }
        } else {
            Ok(())
        }
    }
}

impl<S, T> InsertDefinition<T> for S
where
    T: SetDefinition + Sized + GetDefinition + GetReduction,
    S: ConceptWriter<T> + Container<T>,
{
}
