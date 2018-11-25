use std::fmt;
use traits::{ConceptAdder, ConceptNumber, FindDefinition, GetNormalForm, InsertDefinition, LabelGetter, UpdateNormalForm};
use utils::ZiaResult;

pub trait Labeller<T>
where
    T: StringFactory
        + AbstractFactory
        + fmt::Display
        + InsertDefinition
        + FindDefinition<T>
        + GetNormalForm<T>
        + UpdateNormalForm
        + PartialEq
        + Clone,
    Self: StringMaker<T> + LabelGetter<T> + Definer<T>,
{
    fn label(&mut self, concept: &mut T, string: &str) -> ZiaResult<()> {
        let mut label_concept = self.get_label_concept();
        let mut definition = try!(self.insert_definition(&mut label_concept, concept));
        let mut string_ref = self.new_string(string);
        definition.update_normal_form(&mut string_ref)
    }
	fn new_labelled_abstract(&mut self, string: &str) -> ZiaResult<T> {
        let mut new_abstract = self.new_abstract();
        try!(self.label(&mut new_abstract, string));
        Ok(new_abstract)
    }
    fn setup(&mut self) -> ZiaResult<()> {
        self.new_abstract(); // for LABEL
        let mut define_concept = self.new_abstract(); // for DEFINE;
        let mut reduction_concept = self.new_abstract(); // for REDUCTION
        try!(self.label(&mut define_concept, ":=")); //two more ids occupied
        self.label(&mut reduction_concept, "->") //two more ids occupied
    }
}

pub trait StringMaker<T>
where
    T: StringFactory,
    Self: ConceptAdder<T> + ConceptNumber,
{
    fn new_string(&mut self, string: &str) -> T {
        let new_id = self.number_of_concepts();
        let string_ref = T::new_string(new_id, string);
        self.add_concept(&string_ref);
        string_ref
    }
}

pub trait Definer<T>
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    Self: AbstractMaker<T>,
{
    fn insert_definition(&mut self, lefthand: &mut T, righthand: &mut T) -> ZiaResult<T> {
        let application = try!(lefthand.find_definition(righthand));
        match application {
            None => {
                let mut definition = self.new_abstract();
                definition.insert_definition(lefthand, righthand);
                Ok(definition.clone())
            }
            Some(def) => Ok(def),
        }
    }
}

pub trait AbstractMaker<T>
where
    T: AbstractFactory,
    Self: ConceptAdder<T> + ConceptNumber,
{
    fn new_abstract(&mut self) -> T {
        let new_id = self.number_of_concepts();
        let concept_ref = T::new_abstract(new_id);
        self.add_concept(&concept_ref);
        concept_ref
    }
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}
