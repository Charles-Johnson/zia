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
use concepts::traits::{AbstractFactory, StringFactory, UpdateNormalForm, ConvertTo, DeleteDefinition, GetNormalForm, GetDefinitionOf, MaybeString, FindDefinition, DeleteReduction, Unlabeller, MaybeDisconnected, GetLabel};
use ast::traits::{Container, Display, Pair, MaybeConcept, MightExpand};
use concept_and_ast_traits::{Combine, Expander, InsertDefinition, Reduce, SyntaxFromConcept};
use context::traits::{BlindConceptAdder, StringAdder, ConceptNumber, LabelConcept};
use std::{cell::RefCell, rc::Rc};
use token::parse_line;
use utils::{ZiaError, ZiaResult};
use constants::{DEFINE, REDUCTION};
use self::concept_tidyer::{ConceptTidyer, GetId, SetId};
use self::syntax_finder::{Label, SyntaxFinder, SyntaxFactory};

pub trait ContextMaker<T, V>
where
	Self: Labeller<T, V> + Sized + Default,
    T: InsertDefinition
        + UpdateNormalForm
        + GetDefinitionOf<T>
        + StringFactory
        + AbstractFactory
        + ConvertTo<Rc<RefCell<V>>>,
	V: MaybeString,
{
	fn new() -> Self {
        let mut cont = Self::default();
        cont.setup().unwrap();
        cont
    }
}

impl<S, T, V> ContextMaker<T, V> for S
where
	S: Labeller<T, V> + Sized + Default,
    T: InsertDefinition
        + UpdateNormalForm
        + GetDefinitionOf<T>
        + StringFactory
        + AbstractFactory
        + ConvertTo<Rc<RefCell<V>>>,
	V: MaybeString,
{}

pub trait Execute<T, V>
where
    Self: Call<T, V> + SyntaxConverter<T>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
{
    fn execute<U: Reduce<T> + Expander<T> + Container + Display>(
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

impl<S, T, V> Execute<T, V> for S
where
    T: AbstractFactory
        + StringFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
        + From<Rc<RefCell<V>>>
        + ConvertTo<Rc<RefCell<V>>>
        + SetId
		+ GetLabel,
    S: Call<T, V> + SyntaxConverter<T>,
	V: MaybeString,
{
}

pub trait SyntaxConverter<T>
where
    Self: SyntaxFinder<T>,
    T: Label + GetDefinitionOf<T> + PartialEq,
{
    fn ast_from_expression<U: SyntaxFactory<T> + Combine<T>>(&mut self, s: &str) -> ZiaResult<U> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::EmptyParentheses),
            1 => self.ast_from_token::<U>(&tokens[0]),
            2 => self.ast_from_pair::<U>(&tokens[0], &tokens[1]),
            _ => Err(ZiaError::AmbiguousExpression),
        }
    }
    fn ast_from_pair<U: SyntaxFactory<T> + Combine<T>>(
        &mut self,
        left: &str,
        right: &str,
    ) -> ZiaResult<U> {
        let lefthand = try!(self.ast_from_token::<U>(left));
        let righthand = try!(self.ast_from_token::<U>(right));
        Ok(lefthand.combine_with(&righthand))
    }
    fn ast_from_token<U: SyntaxFactory<T> + Combine<T>>(&mut self, t: &str) -> ZiaResult<U> {
        if t.contains(' ') {
            self.ast_from_expression::<U>(t)
        } else {
            Ok(self.ast_from_symbol::<U>(t))
        }
    }
}

impl<S, T> SyntaxConverter<T> for S
where
    S: SyntaxFinder<T>,
    T: Label + GetDefinitionOf<T> + PartialEq,
{
}

mod syntax_finder {
	pub use concepts::traits::Label;
	pub use ast::traits::SyntaxFactory;
	use context::traits::StringConcept;

	pub trait SyntaxFinder<T>
	where
		T: Label,
		Self: StringConcept<T>,
	{
		fn concept_from_label(&self, s: &str) -> Option<T> {
		    match self.get_string_concept(s) {
		        None => None,
		        Some(c) => c.get_labellee(),
		    }
		}
		fn ast_from_symbol<U: SyntaxFactory<T>>(&mut self, s: &str) -> U {
        	let concept_if_exists = self.concept_from_label(s);
        	U::new(s, concept_if_exists)
    	}
	}

	impl<S, T> SyntaxFinder<T> for S
	where
		S: StringConcept<T>,
		T: Label,
	{
	}
}

pub trait Call<T, V>
where
    Self: RightHandCall<T, V>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
{
    fn call<U: Reduce<T> + Expander<T> + Container + Display>(
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
    fn call_pair<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        left: &mut U,
        right: &U,
    ) -> ZiaResult<String> {
        match right.get_concept() {
            Some(c) => match c.get_id() {
                REDUCTION => Ok(left.recursively_reduce().to_string()),
                DEFINE => Ok(left.expand().to_string()),
                _ => {
                    let right_reduction = c.get_reduction();
                    if let Some(r) = right_reduction {
                        self.call_pair(left, &r.to_ast())
                    } else {
                        self.call_as_righthand(left, right)
                    }
                }
            },
            None => self.call_as_righthand(left, right),
        }
    }
    fn try_expanding_then_call<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let expansion = &ast.expand();
        if expansion != ast {
            self.call(expansion)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
    fn try_reducing_then_call<U: Reduce<T> + Expander<T> + Container + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<String> {
        let normal_form = &ast.recursively_reduce();
        if normal_form != ast {
            self.call(normal_form)
        } else {
            Err(ZiaError::NotAProgram)
        }
    }
}

impl<S, T, V> Call<T, V> for S
where
    S: RightHandCall<T, V>,
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + DeleteDefinition
        + DeleteReduction
        + UpdateNormalForm
        + SyntaxFromConcept
        + MaybeDisconnected
        + Display
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
{
}

pub trait RightHandCall<T, V>
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + AbstractFactory
        + StringFactory
        + MaybeDisconnected
        + SyntaxFromConcept
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
    Self: Definer<T, V> + ExecuteReduction<T, V>,
{
    fn call_as_righthand<
        U: MaybeConcept<T> + Container + Pair<T, U> + Display + Clone + Combine<T> + SyntaxFactory<T>,
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
        U: MaybeConcept<T> + Container + Pair<T, U> + Display + Clone + Combine<T> + SyntaxFactory<T>,
    >(
        &mut self,
        left: &mut U,
        rightleft: &U,
        rightright: &mut U,
    ) -> ZiaResult<String> {
        match rightleft.get_concept() {
            Some(c) => match c.get_id() {
                REDUCTION => self.execute_reduction::<U>(left, rightright),
                DEFINE => self.try_definition::<U>(left, rightright),
                _ => {
                    let rightleft_reduction = c.get_reduction();
                    if let Some(r) = rightleft_reduction {
                        self.match_righthand_pair::<U>(left, &r.to_ast(), rightright)
                    } else {
                        Err(ZiaError::NotAProgram)
                    }
                }
            },
            None => Err(ZiaError::NotAProgram),
        }
    }
    fn try_definition<
        U: MaybeConcept<T> + Container + Pair<T, U> + Display + Clone + Combine<T> + SyntaxFactory<T>,
    >(
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
}

pub trait ExecuteReduction<T, V>
where
	Self: ConceptMaker<T, V>,
	T: DeleteReduction + UpdateNormalForm + InsertDefinition + ConvertTo<Rc<RefCell<V>>> + GetDefinitionOf<T> + AbstractFactory + StringFactory,
	V: MaybeString,
{
	fn execute_reduction<U: Container + MaybeConcept<T> + Display>(
		&mut self,
        syntax: &mut U,
        normal_form: &U,
	) -> ZiaResult<String> {
		if normal_form.contains(syntax) {
            Err(ZiaError::ExpandingReduction)
        } else if syntax == normal_form {
            if let Some(mut c) = syntax.get_concept() {
                c.delete_reduction();
                Ok("".to_string())
            } else {
                Err(ZiaError::RedundantReduction)
            }
        } else {
            let mut syntax_concept = try!(self.concept_from_ast::<U>(syntax));
            let mut normal_form_concept = try!(self.concept_from_ast::<U>(normal_form));
            try!(syntax_concept.update_normal_form(&mut normal_form_concept));
            Ok("".to_string())
        }
	}
}

impl<S, T, V> ExecuteReduction<T, V> for S
where
	S: ConceptMaker<T, V>,
	T: DeleteReduction + UpdateNormalForm + InsertDefinition + ConvertTo<Rc<RefCell<V>>> + GetDefinitionOf<T> + AbstractFactory + StringFactory,
	V: MaybeString,
{}

impl<S, T, V> RightHandCall<T, V> for S
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + AbstractFactory
        + StringFactory
        + MaybeDisconnected
        + SyntaxFromConcept
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
    Self: Definer<T, V> + ExecuteReduction<T, V>,
{
}

pub trait Definer<T, V>
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + Unlabeller
        + MaybeDisconnected
        + FindDefinition<T>
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
    Self: ConceptMaker<T, V> + ConceptCleaner<T>,
{
    fn define<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(
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
                (None, Some(ref mut b), None) => self.relabel(b, &after.to_string()),
                (None, Some(ref mut b), Some(_)) => {
                    if b.get_label().is_none() {
                        self.label(b, &after.to_string())
                    } else {
                        self.relabel(b, &after.to_string())
                    }
                }
                (None, None, Some((ref left, ref right))) => {
                    self.define_new_syntax(&after.to_string(), left, right)
                }
                (Some(ref mut a), Some(ref mut b), None) => self.check_to_delete_definition(b, a),
                (Some(ref mut a), Some(ref mut b), Some(_)) => {
                    self.check_for_redundant_definition(b, a)
                }
                (Some(ref mut a), None, Some((ref left, ref right))) => {
                    self.redefine(a, left, right)
                }
            }
        }
    }
    fn check_to_delete_definition(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            self.delete_definition(before);
            Ok(())
        } else {
            Err(ZiaError::DefinitionCollision)
        }
    }
    fn check_for_redundant_definition(&mut self, before: &mut T, after: &mut T) -> ZiaResult<()> {
        if before == after {
            Err(ZiaError::RedundantDefinition)
        } else {
            Err(ZiaError::DefinitionCollision)
        }
    }
    fn delete_definition(&mut self, concept: &mut T) {
        let mut definition = concept.get_definition();
        concept.delete_definition();
        self.try_delete_concept(concept);
        if let Some((ref mut left, ref mut right)) = definition {
            self.try_delete_concept(left);
            self.try_delete_concept(right);
        }
    }
    fn try_delete_concept(&mut self, concept: &mut T) {
        if concept.is_disconnected() {
            concept.unlabel();
            self.cleanly_remove_concept(concept);
        }
    }
    fn redefine<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(
        &mut self,
        concept: &mut T,
        left: &U,
        right: &U,
    ) -> ZiaResult<()> {
        if let Some((ref mut left_concept, ref mut right_concept)) = concept.get_definition() {
            try!(self.relabel(left_concept, &left.to_string()));
            self.relabel(right_concept, &right.to_string())
        } else {
            let mut left_concept = try!(self.concept_from_ast(left));
            let mut right_concept = try!(self.concept_from_ast(right));
            try!(concept.insert_definition(&mut left_concept, &mut right_concept));
            Ok(())
        }
    }
    fn relabel(&mut self, concept: &mut T, new_label: &str) -> ZiaResult<()> {
        concept.unlabel();
        self.label(concept, new_label)
    }
    fn define_new_syntax<U: MightExpand + MaybeConcept<T> + Pair<T, U> + PartialEq + Display>(
        &mut self,
        syntax: &str,
        left: &U,
        right: &U,
    ) -> ZiaResult<()> {
        let mut definition_concept: Option<T> = None;
        if let (Some(ref l), Some(ref r)) = (left.get_concept(), right.get_concept()) {
            definition_concept = l.find_definition(r);
        }
        let new_syntax_tree = U::from_pair(syntax, definition_concept, left, right);
        try!(self.concept_from_ast(&new_syntax_tree));
        Ok(())
    }
}

impl<S, T, V> Definer<T, V> for S
where
    T: DeleteReduction
        + UpdateNormalForm
        + InsertDefinition
        + DeleteDefinition
        + StringFactory
        + AbstractFactory
        + MaybeDisconnected
        + Unlabeller
        + FindDefinition<T>
		+ SetId
		+ ConvertTo<Rc<RefCell<V>>>
		+ GetLabel,
	V: MaybeString,
    S: ConceptMaker<T, V> + ConceptCleaner<T>,
{
}

pub trait ConceptMaker<T, V>
where
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + GetNormalForm
        + UpdateNormalForm
        + GetDefinitionOf<T>
		+ ConvertTo<Rc<RefCell<V>>>,
	V: MaybeString,
    Self: Labeller<T, V>,
{
    fn concept_from_ast<U: MaybeConcept<T> + MightExpand + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<T> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let string = &ast.to_string();
            match ast.get_expansion() {
                None => self.new_labelled_abstract(string),
                Some((ref left, ref right)) => {
                    let mut appc = try!(self.concept_from_ast(left));
                    let mut argc = try!(self.concept_from_ast(right));
                    let mut concept = try!(self.find_or_insert_definition(&mut appc, &mut argc));
                    if !string.contains(' ') {
                        try!(self.label(&mut concept, string));
                    }
                    Ok(concept)
                }
            }
        }
    }
}

impl<S, T, V> ConceptMaker<T, V> for S
where
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + GetNormalForm
        + UpdateNormalForm
        + GetDefinitionOf<T>
		+ ConvertTo<Rc<RefCell<V>>>,
    S: Labeller<T, V>,
	V: MaybeString,
{
}

pub trait ConceptCleaner<T>
where
    Self: ConceptTidyer<T> + ConceptNumber,
    T: GetId + SetId,
{
    fn cleanly_remove_concept(&mut self, concept: &T) {
        self.remove_concept(concept);
        for id in concept.get_id()..self.number_of_concepts() {
            self.correct_id(id);
        }
    }
}

impl<S, T> ConceptCleaner<T> for S
where
    S: ConceptTidyer<T> + ConceptNumber,
    T: GetId + SetId,
{
}

mod concept_tidyer {
	pub use concepts::traits::{GetId, SetId};
	use context::traits::ConceptHandler;	
	pub trait ConceptTidyer<T>
	where
		T: SetId + GetId,
		Self: ConceptHandler<T>,
	{
		fn remove_concept(&mut self, concept: &T) {
		    self.remove_concept_by_id(concept.get_id());
		}
		fn correct_id(&mut self, id: usize) {
		    self.get_concept(id).set_id(id);
		}
	}

impl<S, T> ConceptTidyer<T> for S 
where
	T: SetId + GetId,
	S: ConceptHandler<T>,
{
}
}

pub trait Labeller<T, V>
where
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + GetDefinitionOf<T> + ConvertTo<Rc<RefCell<V>>>,
    Self: StringMaker<T, V> + FindOrInsertDefinition<T> + LabelConcept<T>,
	V: MaybeString,
{
    fn label(&mut self, concept: &mut T, string: &str) -> ZiaResult<()> {
        let mut label_concept = self.get_label_concept();
        let mut definition = try!(self.find_or_insert_definition(&mut label_concept, concept));
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

impl<S, T, V> Labeller<T, V> for S
where
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + GetDefinitionOf<T> + ConvertTo<Rc<RefCell<V>>>,
    S: StringMaker<T, V> + FindOrInsertDefinition<T> + LabelConcept<T>,
	V: MaybeString,
{
}

pub trait StringMaker<T, V>
where
    T: StringFactory + ConvertTo<Rc<RefCell<V>>>,
    Self: ConceptAdder<T, V> + ConceptNumber,
	V: MaybeString,
{
    fn new_string(&mut self, string: &str) -> T {
        let new_id = self.number_of_concepts();
        let string_ref = T::new_string(new_id, string);
        self.add_concept(&string_ref);
        string_ref
    }
}

impl<S, T, V> StringMaker<T, V> for S
where
    T: StringFactory + ConvertTo<Rc<RefCell<V>>>,
    S: ConceptAdder<T, V> + ConceptNumber,
	V: MaybeString,
{
}

pub trait FindOrInsertDefinition<T>
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    Self: AbstractMaker<T>,
{
    fn find_or_insert_definition(&mut self, lefthand: &mut T, righthand: &mut T) -> ZiaResult<T> {
        let application = lefthand.find_definition(righthand);
        match application {
            None => {
                let mut definition = self.new_abstract();
                try!(definition.insert_definition(lefthand, righthand));
                Ok(definition.clone())
            }
            Some(def) => Ok(def),
        }
    }
}

impl<S, T> FindOrInsertDefinition<T> for S
where
    T: AbstractFactory + FindDefinition<T> + InsertDefinition + PartialEq + Clone,
    S: AbstractMaker<T>,
{
}

pub trait AbstractMaker<T>
where
    T: AbstractFactory,
    Self: BlindConceptAdder<T> + ConceptNumber,
{
    fn new_abstract(&mut self) -> T {
        let new_id = self.number_of_concepts();
        let concept_ref = T::new_abstract(new_id);
        self.blindly_add_concept(&concept_ref);
        concept_ref
    }
}

impl<S, T> AbstractMaker<T> for S
where
    T: AbstractFactory,
    S: BlindConceptAdder<T> + ConceptNumber,
{
}

pub trait ConceptAdder<T, V> 
where
	Self: BlindConceptAdder<T> + StringAdder<V>,
	T: ConvertTo<Rc<RefCell<V>>>,
	V: MaybeString, 
{
    fn add_concept(&mut self, concept: &T) {
        self.blindly_add_concept(concept);
        if let Some(ref sr) = concept.convert() {
            self.add_string(sr, &match sr.borrow().get_string() {
				Some(s) => s.clone(), 
				None => panic!("Concept can be converted into a string but has no string!"),
			});
        }
	}
}

impl<S, T, V> ConceptAdder<T, V> for S
where
	S: BlindConceptAdder<T> + StringAdder<V>,
	T: ConvertTo<Rc<RefCell<V>>>,
	V: MaybeString, 
{}

