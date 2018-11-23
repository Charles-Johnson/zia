/*  Library for the Zia programming language.
    Copyright (C) 2018  Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
	along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

mod base;
mod definition_modifier;
mod label;
mod normal_form_modifier;

pub use self::definition_modifier::{DeleteDefinition, FindDefinition, InsertDefinition};
pub use self::label::{Label, SyntaxFinder};
pub use self::normal_form_modifier::{DeleteNormalForm, UpdateNormalForm};

pub use self::base::{
    AbstractFactory, ConceptAdder, ConceptNumber, ConceptTidyer, GetDefinition, GetDefinitionOf,
    GetNormalForm, GetNormalFormOf, HasToken, Id, MatchLeftRight, MaybeConcept, MightExpand, Pair,
    RefactorFrom, RemoveDefinition, RemoveNormalForm, SetDefinition, SetNormalForm, StringFactory,
    SyntaxFactory,
};
use constants::{DEFINE, REDUCTION};
use std::fmt;
use std::ops::Add;
use token::{parse_line, parse_tokens, Token};
use utils::{ZiaError, ZiaResult};

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

//////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Reduce<T, U>
where
    Self: SyntaxFromConcept<T, U>,
    T: Clone
        + GetDefinition<T>
        + fmt::Display
        + PartialEq
        + FindDefinition<T>
        + GetNormalFormOf<T>
        + GetNormalForm<T>,
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
    T: Clone
        + GetDefinition<T>
        + fmt::Display
        + PartialEq
        + FindDefinition<T>
        + GetNormalFormOf<T>
        + GetNormalForm<T>,
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

////////////////////////////////////////////////////////////////////////////////////////////////

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

//////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Expander<T, U>
where
    T: GetNormalFormOf<T>
        + GetNormalForm<T>
        + FindDefinition<T>
        + Clone
        + PartialEq
        + fmt::Display
        + GetDefinition<T>,
    U: MaybeConcept<T> + HasToken + MightExpand,
    Self: TokenHandler<T>,
{
    fn expand_ast_token(&self, ast: &U) -> ZiaResult<Token> {
        if let Some(con) = ast.get_concept() {
            self.expand_as_token(&con)
        } else if let Some((ref app2, ref arg2)) = ast.get_expansion() {
            Ok(try!(self.expand_ast_token(app2)) + try!(self.expand_ast_token(arg2)))
        } else {
            Ok(ast.get_token())
        }
    }
}

pub trait TokenHandler<T>
where
    T: GetNormalFormOf<T>
        + GetNormalForm<T>
        + FindDefinition<T>
        + Clone
        + PartialEq
        + fmt::Display
        + GetDefinition<T>,
    Self: LabelGetter<T>,
{
    fn get_token(&self, c: &T) -> ZiaResult<Token> {
        match try!(self.get_label(c)) {
            None => match c.get_definition() {
                Some((ref left, ref right)) => self.join_tokens(left, right),
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
            Some(s) => Ok(Token::Atom(s)),
        }
    }
    fn join_tokens(&self, app: &T, arg: &T) -> ZiaResult<Token> {
        Ok(try!(self.get_token(&app)) + try!(self.get_token(&arg)))
    }
    fn expand_as_token(&self, c: &T) -> ZiaResult<Token> {
        match c.get_definition() {
            Some((app, arg)) => self.join_tokens(&app, &arg),
            None => self.get_token(c),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SyntaxConverter<T, U>
where
    Self: SyntaxFinder<T>,
    T: Clone + Id + GetDefinition<T> + Label<T>,
    U: SyntaxFactory<T> + Add<U, Output = ZiaResult<U>>,
{
    fn ast_from_expression(&mut self, s: &str) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::Syntax(
                "Parentheses need to contain an expression".to_string(),
            )),
            1 => self.ast_from_atom(&tokens[0]),
            2 => {
                let parsed_tokens = parse_tokens(&tokens);
                self.ast_from_pair(&parsed_tokens[0], &parsed_tokens[1])
            }
            _ => Err(ZiaError::Syntax(
                "Expression composed of more than 2 tokens has not been implemented yet"
                    .to_string(),
            )),
        }
    }
    fn ast_from_atom(&mut self, s: &str) -> ZiaResult<U> {
        let concept_if_exists = try!(self.concept_from_label(s));
        Ok(U::new(s, concept_if_exists))
    }
    fn ast_from_pair(&mut self, left: &Token, right: &Token) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token(left));
        let righthand = try!(self.ast_from_token(right));
        lefthand + righthand
    }
    fn ast_from_token(&mut self, t: &Token) -> ZiaResult<U> {
        match *t {
            Token::Atom(ref s) => self.ast_from_atom(s),
            Token::Expression(ref s) => self.ast_from_expression(s),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

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
    Self: LabelledAbstractMaker<T>,
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

pub trait LabelledAbstractMaker<T>
where
    T: StringFactory
        + AbstractFactory
        + fmt::Display
        + InsertDefinition
        + FindDefinition<T>
        + GetNormalForm<T>
        + UpdateNormalForm
        + Clone
        + PartialEq,
    Self: AbstractMaker<T> + Labeller<T>,
{
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

////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////////////

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
