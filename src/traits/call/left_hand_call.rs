use constants::{DEFINE, REDUCTION};
use std::{fmt, marker};
use token::Token;
use traits::{GetDefinition, GetNormalForm, HasToken, Id, MaybeConcept, MightExpand};
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

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
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

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
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

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
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

pub trait Pair
where
    Self: marker::Sized + Clone,
{
    fn from_pair(Token, &Self, &Self) -> ZiaResult<Self>;
}

pub trait FindDefinition<T>
where
    T: GetDefinitionOf<T> + Clone + PartialEq,
    Self: GetDefinitionOf<T>,
{
    fn find_definition(&self, righthand: &T) -> ZiaResult<Option<T>> {
        let mut candidates: Vec<T> = Vec::new();
        for candidate in self.get_lefthand_of() {
            let has_righthand = righthand.get_righthand_of().contains(&candidate);
            let new_candidate = !candidates.contains(&candidate);
            if has_righthand && new_candidate {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple definitions with the same lefthand and righthand pair 
				exist."
                    .to_string(),
            )),
        }
    }
}
