mod definer3;

pub use self::definer3::{
    AbstractFactory, AbstractMaker, ConceptMaker, ConceptNumber, ConceptTidyer, Definer, Definer2,
    Definer3, DeleteDefinition, DeleteNormalForm, InsertDefinition, Labeller, Pair, Refactor,
    RefactorFrom, RefactorId, RemoveDefinition, RemoveNormalForm, SetDefinition, SetNormalForm,
    StringFactory, StringMaker, Unlabeller, UpdateNormalForm,
};
use constants::{DEFINE, REDUCTION};
use std::fmt;
use traits::{FindDefinition, HasToken, Id, MaybeConcept, MightExpand};
use utils::{ZiaError, ZiaResult};

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, &T);
}

pub trait LeftHandCall<T, U>
where
    T: DeleteNormalForm
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + Id
        + AbstractFactory
        + StringFactory
        + RefactorFrom<T>
        + fmt::Display
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: MaybeConcept<T> + Container + Pair + HasToken,
    Self: Definer3<T, U>,
{
    fn call_as_lefthand(&mut self, left: &U, right: &U) -> ZiaResult<String> {
        match left.get_expansion() {
            Some((ref leftleft, ref leftright)) => if let Some(lrc) = leftright.get_concept() {
                match lrc.get_id() {
                    REDUCTION => if right.contains(leftleft) {
                        Err(ZiaError::Loop("Reduction rule is infinite".to_string()))
                    } else if right == leftleft {
                        if let Some(mut rc) = right.get_concept() {
                            try!(rc.delete_normal_form());
                            Ok("".to_string())
                        } else {
                            Err(ZiaError::Redundancy(
                                "Removing the normal form a symbol that was never previously used \
                                 is redundant"
                                    .to_string(),
                            ))
                        }
                    } else {
                        try!(
                            try!(self.concept_from_ast(leftleft))
                                .update_normal_form(&mut try!(self.concept_from_ast(right)))
                        );
                        Ok("".to_string())
                    },
                    DEFINE => {
                        if right.contains(leftleft) {
                            Err(ZiaError::Loop("Definition is infinite".to_string()))
                        } else {
                            try!(self.define(right, leftleft));
                            Ok("".to_string())
                        }
                    }
                    _ => Err(ZiaError::Absence(
                        "This concept is not a program".to_string(),
                    )),
                }
            } else {
                Err(ZiaError::Absence(
                    "This concept is not a program".to_string(),
                ))
            },
            None => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
}

pub trait Container
where
    Self: PartialEq + MightExpand,
{
    fn contains(&self, other: &Self) -> bool {
        if let Some((ref left, ref right)) = self.get_expansion() {
            left == other || right == other || left.contains(other) || right.contains(other)
        } else {
            false
        }
    }
}
