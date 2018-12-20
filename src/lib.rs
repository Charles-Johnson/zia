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

mod ast;
mod adding;
mod reading;
mod concepts;
mod constants;
mod context;
mod token;
mod utils;

use self::adding::{AbstractMaker, StringMaker};
use self::reading::{
    Combine, ConceptReader, DisplayJoint, Expander, FindDefinition, FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetLabel, GetNormalForm, GetReduction,
    Label, MaybeConcept, MaybeDisconnected, MaybeString, MightExpand, Pair, Reduce, SyntaxFactory,
};
use self::writing::{InsertDefinition, UpdateNormalForm, DeleteDefinition, DeleteReduction, Unlabeller, RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};
use ast::traits::Container as SyntaxContainer;
pub use ast::AbstractSyntaxTree;
use concepts::Concept;
use constants::{DEFINE, LABEL, REDUCTION};
use context::{BlindConceptRemover, StringConcept, StringRemover};
use context::Context as GenericContext;
use std::fmt;
use token::parse_line;
pub use utils::ZiaError;
use utils::ZiaResult;

pub type Context = GenericContext<Concept>;

pub trait ContextMaker<T>
where
    Self: Labeller<T> + Default,
    T: GetDefinitionOf
        + From<String>
        + Default
        + SetReduction
        + GetDefinition
        + GetReduction
        + SetDefinition
        + MaybeString,
{
    fn new() -> Self {
        let mut cont = Self::default();
        cont.setup().unwrap();
        cont
    }
}

impl<S, T> ContextMaker<T> for S
where
    S: Labeller<T> + Default,
    T: GetDefinitionOf
        + From<String>
        + Default
        + SetReduction
        + GetDefinition
        + GetReduction
        + SetDefinition
        + MaybeString,
{
}

pub trait Execute<T>
where
    Self: Call<T> + SyntaxConverter<T>,
    T: From<String>
        + Default
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + GetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetReduction
        + FindWhatReducesToIt,
{
    fn execute<
        U: MaybeConcept
            + SyntaxContainer
            + SyntaxFactory
            + Clone
            + fmt::Display
            + Pair<U>
            + DisplayJoint,
    >(
        &mut self,
        command: &str,
    ) -> String {
        let ast = match self.ast_from_expression::<U>(command) {
            Ok(a) => a,
            Err(e) => return e.to_string(),
        };
        match self.call(&ast) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
}

impl<S, T> Execute<T> for S
where
    T: Default
        + From<String>
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + GetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetReduction
        + FindWhatReducesToIt,
    S: Call<T> + SyntaxConverter<T>,
{
}

pub trait SyntaxConverter<T>
where
    Self: SyntaxFinder<T> + Combine<T>,
    T: GetDefinitionOf + GetDefinition + FindWhatReducesToIt,
{
    fn ast_from_expression<U: SyntaxFactory + Pair<U> + MaybeConcept + DisplayJoint>(
        &self,
        s: &str,
    ) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::EmptyParentheses),
            1 => self.ast_from_token::<U>(&tokens[0]),
            2 => self.ast_from_pair::<U>(&tokens[0], &tokens[1]),
            _ => Err(ZiaError::AmbiguousExpression),
        }
    }
    fn ast_from_pair<U: SyntaxFactory + DisplayJoint + MaybeConcept + Pair<U>>(
        &self,
        left: &str,
        right: &str,
    ) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token::<U>(left));
        let righthand = try!(self.ast_from_token::<U>(right));
        Ok(self.combine(&lefthand, &righthand))
    }
    fn ast_from_token<U: SyntaxFactory + MaybeConcept + DisplayJoint + Pair<U>>(
        &self,
        t: &str,
    ) -> ZiaResult<U> {
        if t.contains(' ') {
            self.ast_from_expression::<U>(t)
        } else {
            Ok(self.ast_from_symbol::<U>(t))
        }
    }
}

impl<S, T> SyntaxConverter<T> for S
where
    S: SyntaxFinder<T> + Combine<T>,
    T: GetDefinitionOf + GetDefinition + FindWhatReducesToIt,
{
}

pub trait SyntaxFinder<T>
where
    Self: StringConcept + Label<T>,
    T: FindWhatReducesToIt + GetDefinition,
{
    fn concept_from_label(&self, s: &str) -> Option<usize> {
        match self.get_string_concept(s) {
            None => None,
            Some(c) => self.get_labellee(c),
        }
    }
    fn ast_from_symbol<U: SyntaxFactory>(&self, s: &str) -> U {
        let concept_if_exists = self.concept_from_label(s);
        U::new(s, concept_if_exists)
    }
}

impl<S, T> SyntaxFinder<T> for S
where
    S: StringConcept + Label<T>,
    T: FindWhatReducesToIt + GetDefinition,
{
}

pub trait Call<T>
where
    Self: RightHandCall<T> + Expander<T>,
    T: From<String>
        + Default
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
{
    fn call<
        U: MaybeConcept
            + SyntaxFactory
            + Clone
            + SyntaxContainer
            + DisplayJoint
            + Pair<U>
            + fmt::Display,
    >(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref mut left, ref right)) => self.call_pair(left, right),
            None => {
                match self.try_expanding_then_call(ast) {
                    Ok(s) => return Ok(s),
                    Err(e) => {
                        if let ZiaError::NotAProgram = e {
                        } else {
                            return Err(e);
                        }
                    }
                };
                self.try_reducing_then_call(ast)
            }
        }
    }
    fn call_pair<
        U: MaybeConcept
            + Clone
            + DisplayJoint
            + SyntaxFactory
            + Pair<U>
            + SyntaxContainer
            + fmt::Display,
    >(
        &mut self,
        left: &mut U,
        right: &U,
    ) -> ZiaResult<String> {
        match right.get_concept() {
            Some(c) => match c {
                REDUCTION => Ok(self.recursively_reduce(left).to_string()),
                DEFINE => Ok(self.expand(left).to_string()),
                _ => {
                    let right_reduction = self.read_concept(c).get_reduction();
                    if let Some(r) = right_reduction {
                        let ast = self.to_ast::<U>(r);
                        self.call_pair(left, &ast)
                    } else {
                        self.call_as_righthand(left, right)
                    }
                }
            },
            None => self.call_as_righthand(left, right),
        }
    }
    fn try_expanding_then_call<
        U: MaybeConcept
            + DisplayJoint
            + Pair<U>
            + SyntaxFactory
            + Clone
            + SyntaxContainer
            + fmt::Display,
    >(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let expansion = &self.expand(ast);
        if expansion != ast {
            self.call(expansion)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
    fn try_reducing_then_call<
        U: MaybeConcept
            + Clone
            + SyntaxFactory
            + Pair<U>
            + DisplayJoint
            + SyntaxContainer
            + fmt::Display,
    >(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let normal_form = &self.recursively_reduce(ast);
        if normal_form != ast {
            self.call(normal_form)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
}

impl<S, T> Call<T> for S
where
    S: RightHandCall<T> + Expander<T>,
    T: From<String>
        + Default
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
{
}

pub trait RightHandCall<T>
where
    T: Default
        + From<String>
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetDefinition
        + GetReduction
        + FindWhatReducesToIt,
    Self: Definer<T> + ExecuteReduction<T> + Reduce<T>,
{
    fn call_as_righthand<
        U: MaybeConcept
            + SyntaxContainer
            + Pair<U>
            + DisplayJoint
            + fmt::Display
            + Clone
            + SyntaxFactory,
    >(
        &mut self,
        left: &mut U,
        right: &U,
    ) -> ZiaResult<String> {
        match right.get_expansion() {
            Some((ref rightleft, ref mut rightright)) => {
                self.match_righthand_pair::<U>(left, rightleft, rightright)
            }
            None => Err(ZiaError::NotAProgram),
        }
    }
    fn match_righthand_pair<
        U: MaybeConcept
            + SyntaxContainer
            + Pair<U>
            + fmt::Display
            + Clone
            + DisplayJoint
            + SyntaxFactory,
    >(
        &mut self,
        left: &mut U,
        rightleft: &U,
        rightright: &mut U,
    ) -> ZiaResult<String> {
        match rightleft.get_concept() {
            Some(c) => match c {
                REDUCTION => self.execute_reduction::<U>(left, rightright),
                DEFINE => self.execute_definition::<U>(left, rightright),
                _ => {
                    let rightleft_reduction = self.read_concept(c).get_reduction();
                    if let Some(r) = rightleft_reduction {
                        let ast = self.to_ast::<U>(r);
                        self.match_righthand_pair::<U>(left, &ast, rightright)
                    } else {
                        Err(ZiaError::NotAProgram)
                    }
                }
            },
            None => Err(ZiaError::NotAProgram),
        }
    }
}

impl<S, T> RightHandCall<T> for S
where
    T: Default
        + From<String>
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetDefinition
        + GetReduction
        + FindWhatReducesToIt,
    S: Definer<T> + ExecuteReduction<T> + Reduce<T>,
{
}

pub trait ExecuteReduction<T>
where
    Self: ConceptMaker<T> + DeleteReduction<T>,
    T: SetReduction
        + GetDefinitionOf
        + Default
        + From<String>
        + RemoveReduction
        + GetReduction
        + SetDefinition
        + GetDefinition
        + MaybeString,
{
    fn execute_reduction<U: SyntaxContainer + MaybeConcept + fmt::Display>(
        &mut self,
        syntax: &U,
        normal_form: &U,
    ) -> ZiaResult<String> {
        if normal_form.contains(syntax) {
            Err(ZiaError::ExpandingReduction)
        } else if syntax == normal_form {
            try!(self.try_removing_reduction::<U>(syntax));
            Ok("".to_string())
        } else {
            let syntax_concept = try!(self.concept_from_ast::<U>(syntax));
            let normal_form_concept = try!(self.concept_from_ast::<U>(normal_form));
            try!(self.update_normal_form(syntax_concept, normal_form_concept));
            Ok("".to_string())
        }
    }
}

impl<S, T> ExecuteReduction<T> for S
where
    S: ConceptMaker<T> + DeleteReduction<T>,
    T: SetReduction
        + GetDefinitionOf
        + Default
        + From<String>
        + RemoveReduction
        + GetReduction
        + SetDefinition
        + GetDefinition
        + MaybeString,
{
}

pub trait Definer<T>
where
    T: From<String>
        + Default
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    Self: GetLabel<T> + ConceptMaker<T> + DefinitionDeleter<T>,
{
    fn execute_definition<U: SyntaxContainer + MaybeConcept + Pair<U> + fmt::Display>(
        &mut self,
        new: &U,
        old: &mut U,
    ) -> ZiaResult<String> {
        if old.contains(new) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.define::<U>(old, new));
            Ok("".to_string())
        }
    }
    fn define<U: SyntaxContainer + MaybeConcept + Pair<U> + fmt::Display>(
        &mut self,
        before: &mut U,
        after: &U,
    ) -> ZiaResult<()> {
        if after.get_expansion().is_some() {
            Err(ZiaError::BadDefinition)
        } else {
            match (
                after.get_concept(),
                before.get_concept(),
                before.get_expansion(),
            ) {
                (_, None, None) => Err(ZiaError::RedundantRefactor),
                (None, Some(b), None) => self.relabel(b, &after.to_string()),
                (None, Some(b), Some(_)) => {
                    if self.get_label(b).is_none() {
                        self.label(b, &after.to_string())
                    } else {
                        self.relabel(b, &after.to_string())
                    }
                }
                (None, None, Some((ref left, ref right))) => {
                    self.define_new_syntax(&after.to_string(), left, right)
                }
                (Some(a), Some(b), None) => {
                    if a == b {
                        self.cleanly_delete_definition(a);
                        Ok(())
                    } else {
                        Err(ZiaError::DefinitionCollision)
                    }
                }
                (Some(a), Some(b), Some(_)) => {
                    if a == b {
                        Err(ZiaError::RedundantDefinition)
                    } else {
                        Err(ZiaError::DefinitionCollision)
                    }
                }
                (Some(a), None, Some((ref left, ref right))) => self.redefine(a, left, right),
            }
        }
    }
    fn redefine<U: SyntaxContainer + MaybeConcept + fmt::Display>(
        &mut self,
        concept: usize,
        left: &U,
        right: &U,
    ) -> ZiaResult<()> {
        if let Some((left_concept, right_concept)) = self.read_concept(concept).get_definition() {
            try!(self.relabel(left_concept, &left.to_string()));
            self.relabel(right_concept, &right.to_string())
        } else {
            let left_concept = try!(self.concept_from_ast(left));
            let right_concept = try!(self.concept_from_ast(right));
            try!(self.insert_definition(concept, left_concept, right_concept));
            Ok(())
        }
    }
    fn relabel(&mut self, concept: usize, new_label: &str) -> ZiaResult<()> {
        self.unlabel(concept);
        self.label(concept, new_label)
    }
    fn define_new_syntax<U: SyntaxContainer + MaybeConcept + Pair<U> + fmt::Display>(
        &mut self,
        syntax: &str,
        left: &U,
        right: &U,
    ) -> ZiaResult<()> {
        let definition_concept =
            if let (Some(l), Some(r)) = (left.get_concept(), right.get_concept()) {
                self.find_definition(l, r)
            } else {
                None
            };
        let new_syntax_tree = U::from_pair(syntax, definition_concept, left, right);
        try!(self.concept_from_ast(&new_syntax_tree));
        Ok(())
    }
}

impl<S, T> Definer<T> for S
where
    T: From<String>
        + Default
        + RemoveDefinition
        + SetReduction
        + RemoveReduction
        + SetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    S: ConceptMaker<T> + GetLabel<T> + DefinitionDeleter<T>,
{
}

pub trait DefinitionDeleter<T>
where
    Self: MaybeDisconnected<T> + ConceptRemover<T> + DeleteDefinition<T> + Unlabeller<T>,
    T: RemoveDefinition
        + RemoveReduction
        + GetDefinitionOf
        + GetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + MaybeString,
{
    fn cleanly_delete_definition(&mut self, concept: usize) {
        let definition = self.read_concept(concept).get_definition();
        self.delete_definition(concept);
        self.try_delete_concept(concept);
        if let Some((left, right)) = definition {
            self.try_delete_concept(left);
            self.try_delete_concept(right);
        }
    }
    fn try_delete_concept(&mut self, concept: usize) {
        if self.is_disconnected(concept) {
            self.unlabel(concept);
            self.remove_concept(concept);
        }
    }
}

impl<S, T> DefinitionDeleter<T> for S
where
    S: MaybeDisconnected<T> + ConceptRemover<T> + DeleteDefinition<T> + Unlabeller<T>,
    T: RemoveDefinition
        + RemoveReduction
        + GetDefinitionOf
        + GetDefinition
        + FindWhatReducesToIt
        + GetReduction
        + MaybeString,
{
}

pub trait ConceptMaker<T>
where
    T: From<String>
        + Default
        + SetReduction
        + GetDefinitionOf
        + GetDefinition
        + SetDefinition
        + MaybeString
        + GetReduction,
    Self: Labeller<T> + GetNormalForm<T>,
{
    fn concept_from_ast<U: MaybeConcept + MightExpand<U> + fmt::Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<usize> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let string = &ast.to_string();
            match ast.get_expansion() {
                None => self.new_labelled_abstract(string),
                Some((ref left, ref right)) => {
                    let mut leftc = try!(self.concept_from_ast(left));
                    let mut rightc = try!(self.concept_from_ast(right));
                    let concept = try!(self.find_or_insert_definition(leftc, rightc));
                    if !string.contains(' ') {
                        try!(self.label(concept, string));
                    }
                    Ok(concept)
                }
            }
        }
    }
}

impl<S, T> ConceptMaker<T> for S
where
    T: From<String>
        + Default
        + GetDefinitionOf
        + SetReduction
        + GetDefinition
        + SetDefinition
        + MaybeString
        + GetReduction,
    S: Labeller<T> + GetNormalForm<T>,
{
}

pub trait Labeller<T>
where
    T: SetReduction
        + From<String>
        + Default
        + GetDefinitionOf
        + SetDefinition
        + GetReduction
        + GetDefinition
        + GetReduction
        + MaybeString,
    Self: StringMaker<T> + FindOrInsertDefinition<T> + UpdateNormalForm<T>,
{
    fn label(&mut self, concept: usize, string: &str) -> ZiaResult<()> {
        let definition = try!(self.find_or_insert_definition(LABEL, concept));
        let string_id = self.new_string(string);
        self.update_normal_form(definition, string_id)
    }
    fn new_labelled_abstract(&mut self, string: &str) -> ZiaResult<usize> {
        let new_abstract = self.new_abstract();
        try!(self.label(new_abstract, string));
        Ok(new_abstract)
    }
    fn setup(&mut self) -> ZiaResult<()> {
        self.new_abstract(); // for LABEL
        let define_concept = self.new_abstract(); // for DEFINE;
        let reduction_concept = self.new_abstract(); // for REDUCTION
        try!(self.label(define_concept, ":=")); //two more ids occupied
        self.label(reduction_concept, "->") //two more ids occupied
    }
}

impl<S, T> Labeller<T> for S
where
    T: SetReduction
        + From<String>
        + Default
        + GetDefinitionOf
        + SetDefinition
        + GetReduction
        + GetDefinition
        + GetReduction
        + MaybeString,
    S: StringMaker<T> + FindOrInsertDefinition<T> + UpdateNormalForm<T>,
{
}

pub trait FindOrInsertDefinition<T>
where
    T: Default + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
    Self: AbstractMaker<T> + InsertDefinition<T> + FindDefinition<T>,
{
    fn find_or_insert_definition(&mut self, lefthand: usize, righthand: usize) -> ZiaResult<usize> {
        let pair = self.find_definition(lefthand, righthand);
        match pair {
            None => {
                let definition = self.new_abstract();
                try!(self.insert_definition(definition, lefthand, righthand));
                Ok(definition)
            }
            Some(def) => Ok(def),
        }
    }
}

impl<S, T> FindOrInsertDefinition<T> for S
where
    T: Default + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
    S: AbstractMaker<T> + InsertDefinition<T> + FindDefinition<T>,
{
}

pub trait ConceptRemover<T>
where
    Self: BlindConceptRemover + ConceptReader<T> + StringRemover,
    T: MaybeString,
{
    fn remove_concept(&mut self, concept: usize) {
        if let Some(ref s) = self.read_concept(concept).get_string() {
            self.remove_string(s);
        }
        self.blindly_remove_concept(concept);
    }
}

impl<S, T> ConceptRemover<T> for S
where
    S: BlindConceptRemover + ConceptReader<T> + StringRemover,
    T: MaybeString,
{
}

mod writing {
	use context::ConceptWriter;
    use reading::{Container, GetReduction, GetDefinition, GetNormalForm, ConceptReader, GetConceptOfLabel, GetDefinitionOf, MaybeConcept};
	pub use concepts::{SetDefinition, SetReduction, RemoveDefinition, RemoveReduction};
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
}
