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
use self::concept_maker::{AbstractFactory, AbstractMaker, StringFactory, StringMaker};
use self::concept_reader::{
    Combine, ConceptReader, Container, DisplayJoint, Expander, FindDefinition, FindWhatReducesToIt,
    GetConceptOfLabel, GetDefinition, GetDefinitionOf, GetLabel, GetNormalForm, GetReduction,
    Label, MaybeConcept, MaybeDisconnected, MaybeString, MightExpand, Pair, Reduce, SyntaxFactory,
};
use ast::traits::Container as SyntaxContainer;
use concepts::traits::{RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};
use constants::{DEFINE, LABEL, REDUCTION};
use context::traits::{BlindConceptRemover, ConceptWriter, StringConcept, StringRemover};
use std::fmt;
use token::parse_line;
use utils::{ZiaError, ZiaResult};

pub trait ContextMaker<T>
where
    Self: Labeller<T> + Default,
    T: GetDefinitionOf
        + StringFactory
        + AbstractFactory
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
        + StringFactory
        + AbstractFactory
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
    T: StringFactory
        + AbstractFactory
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
    T: AbstractFactory
        + StringFactory
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
        &mut self,
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
        &mut self,
        left: &str,
        right: &str,
    ) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token::<U>(left));
        let righthand = try!(self.ast_from_token::<U>(right));
        Ok(self.combine(&lefthand, &righthand))
    }
    fn ast_from_token<U: SyntaxFactory + MaybeConcept + DisplayJoint + Pair<U>>(
        &mut self,
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
    fn ast_from_symbol<U: SyntaxFactory>(&mut self, s: &str) -> U {
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
    T: StringFactory
        + AbstractFactory
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
    T: StringFactory
        + AbstractFactory
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
    T: AbstractFactory
        + StringFactory
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
    T: AbstractFactory
        + StringFactory
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
        + AbstractFactory
        + StringFactory
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
        + AbstractFactory
        + StringFactory
        + RemoveReduction
        + GetReduction
        + SetDefinition
        + GetDefinition
        + MaybeString,
{
}

pub trait Definer<T>
where
    T: StringFactory
        + AbstractFactory
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
    T: StringFactory
        + AbstractFactory
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

pub trait ConceptMaker<T>
where
    T: StringFactory
        + AbstractFactory
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
    T: StringFactory
        + AbstractFactory
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
        + StringFactory
        + AbstractFactory
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
        + StringFactory
        + AbstractFactory
        + GetDefinitionOf
        + SetDefinition
        + GetReduction
        + GetDefinition
        + GetReduction
        + MaybeString,
    S: StringMaker<T> + FindOrInsertDefinition<T> + UpdateNormalForm<T>,
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

pub trait FindOrInsertDefinition<T>
where
    T: AbstractFactory + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
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
    T: AbstractFactory + GetDefinition + GetReduction + SetDefinition + GetDefinitionOf,
    S: AbstractMaker<T> + InsertDefinition<T> + FindDefinition<T>,
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

mod concept_maker {
    pub use concepts::traits::{AbstractFactory, StringFactory};
    use context::traits::{ConceptAdder, StringAdder};
    pub trait StringMaker<T>
    where
        T: StringFactory,
        Self: ConceptAdder<T> + StringAdder,
    {
        fn new_string(&mut self, string: &str) -> usize {
            let string_concept = T::new_string(string);
            let index = self.add_concept(string_concept);
            self.add_string(index, string);
            index
        }
    }

    impl<S, T> StringMaker<T> for S
    where
        T: StringFactory,
        S: ConceptAdder<T> + StringAdder,
    {
    }
    pub trait AbstractMaker<T>
    where
        T: AbstractFactory,
        Self: ConceptAdder<T>,
    {
        fn new_abstract(&mut self) -> usize {
            let concept = T::new_abstract();
            self.add_concept(concept)
        }
    }

    impl<S, T> AbstractMaker<T> for S
    where
        T: AbstractFactory,
        S: ConceptAdder<T>,
    {
    }
}

mod concept_reader {
    pub use ast::traits::{DisplayJoint, MaybeConcept, MightExpand, Pair, SyntaxFactory};
    pub use concepts::traits::{
        FindWhatReducesToIt, GetDefinition, GetDefinitionOf, GetReduction, MaybeString,
    };
    use constants::LABEL;
    pub use context::traits::ConceptReader;
    use std::fmt;
    pub trait Expander<T>
    where
        Self: Reduce<T>,
        T: GetReduction + GetDefinition + GetDefinitionOf + MaybeString,
    {
        fn expand<
            U: MaybeConcept
                + MightExpand<U>
                + fmt::Display
                + Clone
                + Pair<U>
                + DisplayJoint
                + SyntaxFactory,
        >(
            &self,
            ast: &U,
        ) -> U {
            if let Some(con) = ast.get_concept() {
                if let Some((left, right)) = self.read_concept(con).get_definition() {
                    self.combine(
                        &self.expand(&self.to_ast::<U>(left)),
                        &self.expand(&self.to_ast::<U>(right)),
                    )
                } else {
                    self.to_ast::<U>(con)
                }
            } else if let Some((ref left, ref right)) = ast.get_expansion() {
                self.combine(&self.expand(left), &self.expand(right))
            } else {
                ast.clone()
            }
        }
    }

    impl<S, T> Expander<T> for S
    where
        S: Reduce<T>,
        T: GetReduction + GetDefinition + GetDefinitionOf + MaybeString,
    {
    }
    pub trait Reduce<T>
    where
        Self: GetLabel<T> + Combine<T>,
        T: GetDefinitionOf + GetDefinition + GetReduction + MaybeString,
    {
        fn recursively_reduce<
            U: SyntaxFactory + MightExpand<U> + Clone + Pair<U> + MaybeConcept + DisplayJoint,
        >(
            &self,
            ast: &U,
        ) -> U {
            match self.reduce(ast) {
                Some(ref a) => self.recursively_reduce(a),
                None => ast.clone(),
            }
        }
        fn reduce<
            U: SyntaxFactory + MightExpand<U> + Clone + Pair<U> + MaybeConcept + DisplayJoint,
        >(
            &self,
            ast: &U,
        ) -> Option<U> {
            match ast.get_concept() {
                Some(c) => self.reduce_concept::<U>(c),
                None => match ast.get_expansion() {
                    None => None,
                    Some((ref left, ref right)) => self.match_left_right::<U>(
                        self.reduce(left),
                        self.reduce(right),
                        left,
                        right,
                    ),
                },
            }
        }
        fn reduce_concept<U: SyntaxFactory + Clone + Pair<U> + MaybeConcept + DisplayJoint>(
            &self,
            concept: usize,
        ) -> Option<U> {
            match self.get_normal_form(concept) {
                None => match self.read_concept(concept).get_definition() {
                    Some((left, right)) => {
                        let left_result = self.reduce_concept::<U>(left);
                        let right_result = self.reduce_concept::<U>(right);
                        self.match_left_right::<U>(
                            left_result,
                            right_result,
                            &self.to_ast::<U>(left),
                            &self.to_ast::<U>(right),
                        )
                    }
                    None => None,
                },
                Some(n) => Some(self.to_ast::<U>(n)),
            }
        }
        fn to_ast<U: SyntaxFactory + Clone + Pair<U> + MaybeConcept + DisplayJoint>(
            &self,
            concept: usize,
        ) -> U {
            match self.get_label(concept) {
                Some(ref s) => U::new(s, Some(concept)),
                None => match self.read_concept(concept).get_definition() {
                    Some((left, right)) => {
                        self.combine(&self.to_ast::<U>(left), &self.to_ast::<U>(right))
                    }
                    None => panic!("Unlabelled concept with no definition"),
                },
            }
        }
        fn match_left_right<U: Pair<U> + MaybeConcept + DisplayJoint>(
            &self,
            left: Option<U>,
            right: Option<U>,
            original_left: &U,
            original_right: &U,
        ) -> Option<U> {
            match (left, right) {
                (None, None) => None,
                (Some(new_left), None) => Some(self.contract_pair::<U>(&new_left, original_right)),
                (None, Some(new_right)) => Some(self.contract_pair::<U>(original_left, &new_right)),
                (Some(new_left), Some(new_right)) => {
                    Some(self.contract_pair::<U>(&new_left, &new_right))
                }
            }
        }
        fn contract_pair<U: MaybeConcept + Pair<U> + DisplayJoint>(
            &self,
            lefthand: &U,
            righthand: &U,
        ) -> U {
            if let (Some(lc), Some(rc)) = (lefthand.get_concept(), righthand.get_concept()) {
                if let Some(def) = self.find_definition(lc, rc) {
                    if let Some(ref a) = self.get_label(def) {
                        return U::from_pair(a, Some(def), lefthand, righthand);
                    }
                }
            }
            self.combine(lefthand, righthand)
        }
    }

    impl<S, T> Reduce<T> for S
    where
        S: GetLabel<T> + Combine<T>,
        T: GetDefinitionOf + GetDefinition + MaybeString + GetReduction,
    {
    }
    pub trait Display<T>
    where
        Self: GetLabel<T>,
        T: MaybeString + GetDefinitionOf + GetDefinition + GetReduction,
    {
        fn display(&self, concept: usize) -> String {
            match self.read_concept(concept).get_string() {
                Some(s) => "\"".to_string() + &s + "\"",
                None => match self.get_label(concept) {
                    Some(l) => l,
                    None => match self.read_concept(concept).get_definition() {
                        Some((left, right)) => {
                            let mut left_string = self.display(left);
                            if left_string.contains(' ') {
                                left_string = "(".to_string() + &left_string;
                            }
                            let mut right_string = self.display(right);
                            if right_string.contains(' ') {
                                right_string += ")";
                            }
                            left_string + " " + &right_string
                        }
                        None => panic!("Unlabelled concept with no definition!"),
                    },
                },
            }
        }
    }

    impl<S, T> Display<T> for S
    where
        S: GetLabel<T>,
        T: MaybeString + GetDefinitionOf + GetDefinition + GetReduction,
    {
    }
    pub trait GetLabel<T>
    where
        T: MaybeString + GetDefinitionOf + GetDefinition + GetReduction,
        Self: GetNormalForm<T> + GetConceptOfLabel<T>,
    {
        fn get_label(&self, concept: usize) -> Option<String> {
            match self.get_concept_of_label(concept) {
                None => None,
                Some(d) => match self.get_normal_form(d) {
                    None => None,
                    Some(n) => self.read_concept(n).get_string(),
                },
            }
        }
    }

    impl<S, T> GetLabel<T> for S
    where
        T: MaybeString + GetDefinitionOf + GetDefinition + GetReduction,
        S: GetNormalForm<T> + GetConceptOfLabel<T>,
    {
    }
    pub trait Combine<T>
    where
        Self: FindDefinition<T>,
        T: GetDefinitionOf,
    {
        fn combine<U: DisplayJoint + MaybeConcept + Pair<U> + Sized>(
            &self,
            ast: &U,
            other: &U,
        ) -> U {
            let left_string = ast.display_joint();
            let right_string = other.display_joint();
            let definition = if let (Some(l), Some(r)) = (ast.get_concept(), other.get_concept()) {
                self.find_definition(l, r)
            } else {
                None
            };
            U::from_pair(&(left_string + " " + &right_string), definition, ast, other)
        }
    }

    impl<S, T> Combine<T> for S
    where
        T: GetDefinitionOf,
        S: FindDefinition<T>,
    {
    }
    pub trait Label<T>
    where
        T: GetDefinition + FindWhatReducesToIt,
        Self: FindWhatItsANormalFormOf<T>,
    {
        fn get_labellee(&self, concept: usize) -> Option<usize> {
            let mut candidates: Vec<usize> = Vec::new();
            for label in self.find_what_its_a_normal_form_of(concept) {
                match self.read_concept(label).get_definition() {
                    None => continue,
                    Some((r, x)) => {
                        if r == LABEL {
                            candidates.push(x)
                        } else {
                            continue;
                        }
                    }
                };
            }
            match candidates.len() {
                0 => None,
                1 => Some(candidates[0]),
                _ => panic!("Multiple concepts are labelled with the same string"),
            }
        }
    }

    impl<S, T> Label<T> for S
    where
        S: FindWhatItsANormalFormOf<T>,
        T: GetDefinition + FindWhatReducesToIt,
    {
    }
    pub trait GetNormalForm<T>
    where
        T: GetReduction,
        Self: ConceptReader<T>,
    {
        fn get_normal_form(&self, concept: usize) -> Option<usize> {
            match self.read_concept(concept).get_reduction() {
                None => None,
                Some(n) => match self.get_normal_form(n) {
                    None => Some(n),
                    Some(m) => Some(m),
                },
            }
        }
    }

    impl<S, T> GetNormalForm<T> for S
    where
        S: ConceptReader<T>,
        T: GetReduction,
    {
    }

    pub trait GetConceptOfLabel<T>
    where
        T: GetDefinition + GetDefinitionOf,
        Self: ConceptReader<T>,
    {
        fn get_concept_of_label(&self, concept: usize) -> Option<usize> {
            for candidate in self.read_concept(concept).get_righthand_of() {
                match self.read_concept(candidate).get_definition() {
                    None => panic!("Candidate should have a definition!"),
                    Some((left, _)) => {
                        if left == LABEL {
                            return Some(candidate);
                        }
                    }
                };
            }
            None
        }
    }

    impl<S, T> GetConceptOfLabel<T> for S
    where
        T: GetDefinition + GetDefinitionOf,
        S: ConceptReader<T>,
    {
    }

    pub trait MaybeDisconnected<T>
    where
        T: GetReduction + FindWhatReducesToIt + GetDefinition + GetDefinitionOf,
        Self: ConceptReader<T>,
    {
        fn is_disconnected(&self, concept: usize) -> bool {
            self.read_concept(concept).get_reduction().is_none()
                && self.read_concept(concept).get_definition().is_none()
                && self.read_concept(concept).get_lefthand_of().is_empty()
                && self.righthand_of_without_label_is_empty(concept)
                && self
                    .read_concept(concept)
                    .find_what_reduces_to_it()
                    .is_empty()
        }
        fn righthand_of_without_label_is_empty(&self, con: usize) -> bool {
            for concept in self.read_concept(con).get_righthand_of() {
                if let Some((left, _)) = self.read_concept(concept).get_definition() {
                    if left != LABEL {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl<S, T> MaybeDisconnected<T> for S
    where
        T: GetReduction + FindWhatReducesToIt + GetDefinition + GetDefinitionOf,
        S: ConceptReader<T>,
    {
    }

    pub trait FindDefinition<T>
    where
        T: GetDefinitionOf,
        Self: ConceptReader<T>,
    {
        fn find_definition(&self, lefthand: usize, righthand: usize) -> Option<usize> {
            let mut candidates: Vec<usize> = Vec::new();
            for candidate in self.read_concept(lefthand).get_lefthand_of() {
                let has_righthand = self
                    .read_concept(righthand)
                    .get_righthand_of()
                    .contains(&candidate);
                let new_candidate = !candidates.contains(&candidate);
                if has_righthand && new_candidate {
                    candidates.push(candidate);
                }
            }
            match candidates.len() {
                0 => None,
                1 => Some(candidates[0]),
                _ => {
                    panic!("Multiple definitions with the same lefthand and righthand pair exist.")
                }
            }
        }
    }

    impl<S, T> FindDefinition<T> for S
    where
        T: GetDefinitionOf,
        S: ConceptReader<T>,
    {
    }

    pub trait FindWhatItsANormalFormOf<T>
    where
        T: FindWhatReducesToIt,
        Self: ConceptReader<T>,
    {
        fn find_what_its_a_normal_form_of(&self, con: usize) -> Vec<usize> {
            let mut normal_form_of: Vec<usize> = Vec::new();
            for concept in self.read_concept(con).find_what_reduces_to_it() {
                normal_form_of.push(concept);
                for concept2 in self.find_what_its_a_normal_form_of(concept) {
                    normal_form_of.push(concept2);
                }
            }
            normal_form_of
        }
    }

    impl<S, T> FindWhatItsANormalFormOf<T> for S
    where
        S: ConceptReader<T>,
        T: FindWhatReducesToIt,
    {
    }

    pub trait Container<T>
    where
        Self: ConceptReader<T>,
        T: GetDefinition,
    {
        fn contains(&self, outer: usize, inner: usize) -> bool {
            if let Some((left, right)) = self.read_concept(outer).get_definition() {
                left == inner
                    || right == inner
                    || self.contains(left, inner)
                    || self.contains(right, inner)
            } else {
                false
            }
        }
    }

    impl<S, T> Container<T> for S
    where
        S: ConceptReader<T>,
        T: GetDefinition,
    {
    }
}
