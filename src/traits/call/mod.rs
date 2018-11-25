mod reduce;
mod expander;
mod left_hand_call;

pub use self::expander::{Expander, TokenHandler};
pub use self::reduce::{Reduce, MatchLeftRight, SyntaxFromConcept};
pub use self::left_hand_call::{LeftHandCall, Container, Definer3, Definer2, Refactor, Unlabeller, RefactorId, ConceptTidyer, RefactorFrom, ConceptMaker, LabelledAbstractMaker, AbstractMaker, StringMaker, StringFactory, AbstractFactory, FindDefinition, DeleteNormalForm, ConceptNumber, ConceptAdder, InsertDefinition, UpdateNormalForm, DeleteDefinition, Pair, Labeller, Definer, LabelGetter, SetDefinition, RemoveNormalForm, SetNormalForm, RemoveDefinition, GetDefinitionOf};
use std::{fmt, marker};
use token::Token;
use traits::{GetDefinition, Id};
use traits::syntax_converter::GetNormalFormOf;
use utils::{ZiaError, ZiaResult};
use traits::SyntaxFactory;
use constants::{REDUCTION, DEFINE};

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
        + GetNormalFormOf<T>
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
