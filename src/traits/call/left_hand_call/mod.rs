mod labeller;

use constants::{DEFINE, REDUCTION};
pub use self::labeller::{Labeller, Definer, AbstractMaker, StringMaker, AbstractFactory, StringFactory};
use std::{fmt, marker};
use token::Token;
use traits::{
    FindDefinition, GetDefinition, GetNormalForm, HasToken, Id, LabelGetter, MaybeConcept,
    MightExpand,
};
use utils::{ZiaError, ZiaResult};

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

pub trait UpdateNormalForm
where
    Self: SetNormalForm<Self>,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        try!(self.set_normal_form(normal_form));
        normal_form.add_normal_form_of(self);
        Ok(())
    }
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_lefthand_of(&mut self, &T);
    fn remove_righthand_of(&mut self, &T);
}

pub trait SetNormalForm<T>
where
    Self: marker::Sized,
{
    fn set_normal_form(&mut self, &T) -> ZiaResult<()>;
    fn add_normal_form_of(&mut self, &T);
}

pub trait RemoveNormalForm<T> {
    fn remove_normal_form(&mut self);
    fn remove_normal_form_of(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_lefthand_of(&mut self, &T);
    fn add_righthand_of(&mut self, &T);
}

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

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, &T);
}

pub trait DeleteNormalForm
where
    Self: GetNormalForm<Self> + RemoveNormalForm<Self>,
{
    fn delete_normal_form(&mut self) -> ZiaResult<()> {
        match try!(self.get_normal_form()) {
            None => (),
            Some(mut n) => {
                n.remove_normal_form_of(self);
                self.remove_normal_form();
            }
        };
        Ok(())
    }
}

pub trait RefactorFrom<T> {
    fn refactor_from(&mut self, &T) -> ZiaResult<()>;
}

pub trait ConceptTidyer<T> {
    fn remove_concept(&mut self, &T);
    fn correct_id(&mut self, usize);
}

pub trait ConceptNumber {
    fn number_of_concepts(&self) -> usize;
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

pub trait Definer3<T, U>
where
    T: fmt::Display
        + Id
        + DeleteNormalForm
        + UpdateNormalForm
        + RefactorFrom<T>
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: MightExpand + MaybeConcept<T> + HasToken + Pair + PartialEq,
    Self: Definer2<T, U> + ConceptMaker<T, U>,
{
    fn define(&mut self, before: &U, after: &U) -> ZiaResult<()> {
        if let Some(mut before_c) = before.get_concept() {
            if before == after {
                before_c.delete_definition();
                Ok(())
            } else {
                self.define2(&mut before_c, after)
            }
        } else if let Some((ref left, ref right)) = before.get_expansion() {
            if let Some(mut after_c) = after.get_concept() {
                if let Some((ref mut ap, ref mut ar)) = after_c.get_definition() {
                    try!(self.define2(ap, left));
                    self.define2(ar, right)
                } else {
                    after_c.insert_definition(
                        &mut try!(self.concept_from_ast(left)),
                        &mut try!(self.concept_from_ast(right)),
                    );
                    Ok(())
                }
            } else {
                try!(self.concept_from_ast(&try!(U::from_pair(after.get_token(), left, right,))));
                Ok(())
            }
        } else {
            return Err(ZiaError::Redundancy(
                "Refactoring a symbol that was never previously used is redundant".to_string(),
            ));
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

pub trait ConceptMaker<T, U>
where
    T: StringFactory
        + AbstractFactory
        + fmt::Display
        + InsertDefinition
        + GetNormalForm<T>
        + UpdateNormalForm
        + FindDefinition<T>
        + PartialEq
        + Clone,
    U: MaybeConcept<T> + HasToken + MightExpand,
    Self: Labeller<T>,
{
    fn concept_from_ast(&mut self, ast: &U) -> ZiaResult<T> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let mut c = match ast.get_token() {
                Token::Atom(s) => try!(self.new_labelled_abstract(&s)),
                Token::Expression(_) => self.new_abstract(),
            };
            if let Some((mut app, mut arg)) = ast.get_expansion() {
                let mut appc = try!(self.concept_from_ast(&app));
                let mut argc = try!(self.concept_from_ast(&arg));
                c.insert_definition(&mut appc, &mut argc);
            }
            Ok(c)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Definer2<T, U>
where
    T: InsertDefinition
        + StringFactory
        + AbstractFactory
        + fmt::Display
        + Id
        + RefactorFrom<T>
        + DeleteNormalForm
        + UpdateNormalForm
        + Clone
        + PartialEq
        + FindDefinition<T>,
    U: HasToken + MaybeConcept<T>,
    Self: Refactor<T> + Labeller<T>,
{
    fn define2(&mut self, before_c: &mut T, after: &U) -> ZiaResult<()> {
        if let Some(mut after_c) = after.get_concept() {
            self.refactor(before_c, &mut after_c)
        } else {
            match after.get_token() {
                Token::Atom(s) => {
                    try!(self.unlabel(before_c));
                    self.label(before_c, &s)
                }
                Token::Expression(_) => Err(ZiaError::Syntax(
                    "Only symbols can have definitions".to_string(),
                )),
            }
        }
    }
}

pub trait Refactor<T>
where
    T: RefactorFrom<T>
        + Id
        + DeleteNormalForm
        + fmt::Display
        + PartialEq
        + FindDefinition<T>
        + Clone,
    Self: RefactorId<T> + Unlabeller<T>,
{
    fn refactor(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        try!(self.unlabel(before));
        self.refactor_id(before, after)
    }
}

pub trait RefactorId<T>
where
    T: Id + RefactorFrom<T>,
    Self: ConceptTidyer<T> + ConceptNumber,
{
    fn refactor_id(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if self.number_of_concepts() > before.get_id() {
            try!(after.refactor_from(before));
            self.remove_concept(before);
            for id in before.get_id()..self.number_of_concepts() {
                self.correct_id(id);
            }
            Ok(())
        } else {
            panic!("refactoring id has gone wrong!")
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub trait Unlabeller<T>
where
    T: FindDefinition<T> + PartialEq + DeleteNormalForm + fmt::Display + Clone,
    Self: LabelGetter<T>,
{
    fn unlabel(&mut self, concept: &T) -> ZiaResult<()> {
        match try!(self.get_concept_of_label(concept)) {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
}

pub trait Pair
where
    Self: marker::Sized + Clone,
{
    fn from_pair(Token, &Self, &Self) -> ZiaResult<Self>;
}
