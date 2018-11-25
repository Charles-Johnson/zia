mod expander;
mod label_getter;
mod left_hand_call;
mod reduce;

pub use self::expander::{Expander, TokenHandler};
pub use self::label_getter::{FindDefinition, GetDefinitionOf, LabelGetter};
pub use self::left_hand_call::{
    AbstractFactory, AbstractMaker, ConceptAdder, ConceptMaker, ConceptNumber, ConceptTidyer,
    Container, Definer, Definer2, Definer3, DeleteDefinition, DeleteNormalForm, InsertDefinition,
    Labeller, LeftHandCall, Pair, Refactor, RefactorFrom, RefactorId, RemoveDefinition,
    RemoveNormalForm, SetDefinition, SetNormalForm, StringFactory, StringMaker, Unlabeller,
    UpdateNormalForm,
};
pub use self::reduce::{MatchLeftRight, Reduce, SyntaxFromConcept};
use constants::{DEFINE, REDUCTION};
use std::{fmt, marker};
use token::Token;
use traits::SyntaxFactory;
use traits::{GetDefinition, Id};
use utils::{ZiaError, ZiaResult};

pub trait MightExpand
where
    Self: marker::Sized,
{
    fn get_expansion(&self) -> Option<(Self, Self)>;
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait HasToken {
    fn get_token(&self) -> Token;
}

pub trait GetNormalForm<T>
where
    Self: marker::Sized,
{
    fn get_normal_form(&self) -> ZiaResult<Option<T>>;
}

pub trait Call<T, U>
where
    Self: Reduce<T, U> + LeftHandCall<T, U> + Expander<T, U>,
    T: RefactorFrom<T>
        + StringFactory
        + AbstractFactory
        + Id
        + InsertDefinition
        + DeleteDefinition
        + DeleteNormalForm
        + UpdateNormalForm
        + fmt::Display
        + GetDefinition<T>
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: HasToken + Pair + Container + MaybeConcept<T> + MatchLeftRight + SyntaxFactory<T>,
{
    fn call(&mut self, ast: &U) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref left, ref right)) => if let Some(c) = right.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(self.recursively_reduce(left)).get_token().as_string()),
                    DEFINE => Ok(try!(self.expand_ast_token(left)).as_string()),
                    _ => self.call_as_lefthand(left, right),
                }
            } else {
                self.call_as_lefthand(left, right)
            },
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
}
