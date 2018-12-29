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

/// Traits for adding concepts to the context.
mod adding;

/// Abstract syntax tree. Relates syntax to concepts.
mod ast;

/// The units that make up the context. Defined in terms of their relationship with other concepts.
mod concepts;

/// Integers that represent concrete concepts.
mod constants;

/// The container of concepts that coordinates adding, reading, writing and removing of concepts.
mod context;

/// The errors that the users could make when making commands.
mod errors;

/// Traits for reading concepts within the context.
mod reading;

/// Traits for removing concepts from the context.
mod removing;

/// Traits for the context to translate strings into abstract syntax trees.
mod translating;

/// Traits for writing concepts within the context.
mod writing;

pub use adding::ContextMaker;
use adding::{ConceptMaker, Container, ExecuteReduction, FindOrInsertDefinition, Labeller};
pub use ast::AbstractSyntaxTree;
use concepts::{AbstractConcept, Concept, ConcreteConcept};
use constants::{DEFINE, REDUCTION};
use context::Context as GenericContext;
pub use errors::ZiaError;
use errors::ZiaResult;
use reading::{
    DisplayJoint, Expander, FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetLabel,
    GetReduction, MaybeConcept, MaybeString, MightExpand, Pair, Reduce, SyntaxFactory,
};
use removing::DefinitionDeleter;
use translating::SyntaxConverter;
use writing::{
    MakeReduceFrom, NoLongerReducesFrom, RemoveAsDefinitionOf, RemoveDefinition, RemoveReduction,
    SetAsDefinitionOf, SetDefinition, SetReduction,
};

/// A container for reading and writing `Concept`s.
pub type Context = GenericContext<Concept>;

/// Executing a command based on a string to read or write contained concepts.  
pub trait Execute<T>
where
    Self: Call<T> + SyntaxConverter<T>,
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + GetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetReduction
        + FindWhatReducesToIt,
    Self::S: Container + Pair<Self::S> + Clone + SyntaxFactory + DisplayJoint,
{
    fn execute(&mut self, command: &str) -> String {
        let ast = match self.ast_from_expression(command) {
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
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + GetDefinition
        + MaybeString
        + GetDefinitionOf
        + GetReduction
        + FindWhatReducesToIt,
    S: Call<T> + SyntaxConverter<T>,
    S::S: Container + Pair<S::S> + Clone + SyntaxFactory + DisplayJoint,
{
}

impl FindOrInsertDefinition<Concept> for Context {
    type A = AbstractConcept<ConcreteConcept>;
}

impl Labeller<Concept> for Context {
    type C = ConcreteConcept;
}

impl ConceptMaker<Concept> for Context {
    type S = AbstractSyntaxTree;
}

/// Calling a program expressed as abstract syntax to read or write contained concepts.  
pub trait Call<T>
where
    Self: Definer<T> + ExecuteReduction<T> + Reduce<T> + Expander<T>,
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    Self::S: Container + Pair<Self::S> + Clone + SyntaxFactory + DisplayJoint,
{
    fn call(&mut self, ast: &Self::S) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref left, ref right)) => self.call_pair(left, right),
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
    fn call_pair(&mut self, left: &Self::S, right: &Self::S) -> ZiaResult<String> {
        match right.get_concept() {
            Some(c) => match c {
                REDUCTION => Ok(self.recursively_reduce(left).to_string()),
                DEFINE => Ok(self.expand(left).to_string()),
                _ => {
                    let right_reduction = self.read_concept(c).get_reduction();
                    if let Some(r) = right_reduction {
                        let ast = self.to_ast(r);
                        self.call_pair(left, &ast)
                    } else {
                        self.call_as_righthand(left, right)
                    }
                }
            },
            None => self.call_as_righthand(left, right),
        }
    }
    fn try_expanding_then_call(&mut self, ast: &Self::S) -> ZiaResult<String> {
        let expansion = &self.expand(ast);
        if expansion != ast {
            self.call(expansion)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
    fn try_reducing_then_call(&mut self, ast: &Self::S) -> ZiaResult<String> {
        let normal_form = &self.recursively_reduce(ast);
        if normal_form != ast {
            self.call(normal_form)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
    fn call_as_righthand(&mut self, left: &Self::S, right: &Self::S) -> ZiaResult<String> {
        match right.get_expansion() {
            Some((ref rightleft, ref rightright)) => {
                self.match_righthand_pair(left, rightleft, rightright)
            }
            None => Err(ZiaError::NotAProgram),
        }
    }
    fn match_righthand_pair(
        &mut self,
        left: &Self::S,
        rightleft: &Self::S,
        rightright: &Self::S,
    ) -> ZiaResult<String> {
        match rightleft.get_concept() {
            Some(c) => match c {
                REDUCTION => self.execute_reduction(left, rightright),
                DEFINE => self.execute_definition(left, rightright),
                _ => {
                    let rightleft_reduction = self.read_concept(c).get_reduction();
                    if let Some(r) = rightleft_reduction {
                        let ast = self.to_ast::<Self::S>(r);
                        self.match_righthand_pair(left, &ast, rightright)
                    } else {
                        Err(ZiaError::NotAProgram)
                    }
                }
            },
            None => Err(ZiaError::NotAProgram),
        }
    }
}

impl<S, T> Call<T> for S
where
    S: Definer<T> + ExecuteReduction<T> + Reduce<T> + Expander<T>,
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    S::S: Container + Pair<S::S> + Clone + SyntaxFactory + DisplayJoint,
{
}

/// Defining new syntax in terms of old syntax.
pub trait Definer<T>
where
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    Self: GetLabel<T> + ConceptMaker<T> + DefinitionDeleter<T>,
    Self::S: Pair<Self::S> + Container,
{
    fn execute_definition(&mut self, new: &Self::S, old: &Self::S) -> ZiaResult<String> {
        if old.contains(new) {
            Err(ZiaError::InfiniteDefinition)
        } else {
            try!(self.define(old, new));
            Ok("".to_string())
        }
    }
    fn define(&mut self, before: &Self::S, after: &Self::S) -> ZiaResult<()> {
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
                        self.cleanly_delete_definition(a)
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
    fn redefine(&mut self, concept: usize, left: &Self::S, right: &Self::S) -> ZiaResult<()> {
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
    fn define_new_syntax(
        &mut self,
        syntax: &str,
        left: &Self::S,
        right: &Self::S,
    ) -> ZiaResult<()> {
        let definition_concept =
            if let (Some(l), Some(r)) = (left.get_concept(), right.get_concept()) {
                self.find_definition(l, r)
            } else {
                None
            };
        let new_syntax_tree = Self::S::from_pair(syntax, definition_concept, left, right);
        try!(self.concept_from_ast(&new_syntax_tree));
        Ok(())
    }
}

impl<S, T> Definer<T> for S
where
    T: From<String>
        + From<Self::C>
        + From<Self::A>
        + RemoveDefinition
        + RemoveAsDefinitionOf
        + SetReduction
        + MakeReduceFrom
        + RemoveReduction
        + NoLongerReducesFrom
        + SetDefinition
        + SetAsDefinitionOf
        + FindWhatReducesToIt
        + GetReduction
        + GetDefinition
        + GetDefinitionOf
        + MaybeString,
    S: ConceptMaker<T> + GetLabel<T> + DefinitionDeleter<T>,
    S::S: Pair<S::S> + Container,
{
}
