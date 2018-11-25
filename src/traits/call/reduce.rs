use std::fmt;
use std::ops::Add;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, LabelGetter, MaybeConcept, MightExpand,
    SyntaxFactory,
};
use utils::{ZiaError, ZiaResult};

pub trait Reduce<T, U>
where
    Self: SyntaxFromConcept<T, U>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + MatchLeftRight + MaybeConcept<T> + MightExpand,
{
    fn reduce_concept(&mut self, c: &T) -> ZiaResult<Option<U>> {
        match try!(c.get_normal_form()) {
            None => match c.get_definition() {
                Some((mut left, mut right)) => {
                    let left_result = try!(self.reduce_concept(&left));
                    let right_result = try!(self.reduce_concept(&right));
                    U::match_left_right(
                        left_result,
                        right_result,
                        &try!(self.ast_from_concept(&left)),
                        &try!(self.ast_from_concept(&right)),
                    )
                }
                None => Ok(None),
            },
            Some(n) => Ok(Some(try!(self.ast_from_concept(&n)))),
        }
    }
    fn recursively_reduce(&mut self, ast: &U) -> ZiaResult<U> {
        match try!(self.reduce(ast)) {
            Some(ref a) => self.recursively_reduce(a),
            None => Ok(ast.clone()),
        }
    }
    fn reduce(&mut self, ast: &U) -> ZiaResult<Option<U>> {
        match ast.get_concept() {
            Some(ref c) => self.reduce_concept(c),
            None => match ast.get_expansion() {
                None => Ok(None),
                Some((left, right)) => U::match_left_right(
                    try!(self.reduce(&left)),
                    try!(self.reduce(&right)),
                    &left,
                    &right,
                ),
            },
        }
    }
}

pub trait SyntaxFromConcept<T, U>
where
    Self: LabelGetter<T>,
    T: Clone + GetDefinition<T> + fmt::Display + PartialEq + FindDefinition<T> + GetNormalForm<T>,
    U: SyntaxFactory<T> + Add<U, Output = ZiaResult<U>>,
{
    fn ast_from_concept(&self, c: &T) -> ZiaResult<U> {
        match try!(self.get_label(c)) {
            Some(ref s) => Ok(U::new(s, Some(c.clone()))),
            None => match c.get_definition() {
                Some((ref left, ref right)) => {
                    try!(self.ast_from_concept(left)) + try!(self.ast_from_concept(right))
                }
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
        }
    }
}

pub trait MatchLeftRight
where
    Self: Clone + Add<Self, Output = ZiaResult<Self>>,
{
    fn match_left_right(
        left: Option<Self>,
        right: Option<Self>,
        original_left: &Self,
        original_right: &Self,
    ) -> ZiaResult<Option<Self>> {
        match (left, right) {
            (None, None) => Ok(None),
            (Some(new_left), None) => Ok(Some(try!(new_left + original_right.clone()))),
            (None, Some(new_right)) => Ok(Some(try!(original_left.clone() + new_right))),
            (Some(new_left), Some(new_right)) => Ok(Some(try!(new_left + new_right))),
        }
    }
}
