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
use ast::AbstractSyntaxTree;
use concepts::{ConceptRef, StringRef};
use constants::{DEFINE, LABEL, REDUCTION};
use std::collections::HashMap;
use token::{parse_line, parse_tokens, Token};
use traits::{
    AbstractMaker, Application, ConceptAdder, ConceptMaker, ConceptNumber, ConceptTidyer, Definer, Definer2,
    DefinitionModifier, Expander, HasToken, Id, LabelGetter, LabelledAbstractMaker, Labeller,
    MaybeConcept, MightExpand, NormalForm, NormalFormModifier, Refactor, RefactorId, StringMaker,
    SyntaxFinder, TokenHandler, Unlabeller,
};
use utils::{ZiaError, ZiaResult};

pub struct Context {
    string_map: HashMap<String, StringRef>,
    concepts: Vec<ConceptRef>,
}

impl Context {
    pub fn new() -> ZiaResult<Context> {
        let mut cont = Context {
            string_map: HashMap::new(),
            concepts: Vec::new(),
        };
        try!(cont.setup());
        Ok(cont)
    }
    pub fn call(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((ref app, ref arg)) => if let Some(c) = arg.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(self.recursively_reduce(app)).get_token().as_string()),
                    DEFINE => Ok(try!(self.expand_ast_token(app)).as_string()),
                    _ => self.call_as_applicand(app, arg),
                }
            } else {
                self.call_as_applicand(app, arg)
            },
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
    fn call_as_applicand(
        &mut self,
        app: &AbstractSyntaxTree,
        arg: &AbstractSyntaxTree,
    ) -> ZiaResult<String> {
        match app.get_expansion() {
            Some((ref ap, ref ar)) => if let Some(arc) = ar.get_concept() {
                match arc.get_id() {
                    REDUCTION => if arg.contains(&ap) {
                        Err(ZiaError::Loop("Reduction rule is infinite".to_string()))
                    } else if arg == ap {
                        if let Some(mut c) = arg.get_concept() {
                            try!(c.delete_normal_form());
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
                            try!(self.concept_from_ast(&ap))
                                .update_normal_form(&mut try!(self.concept_from_ast(arg)))
                        );
                        Ok("".to_string())
                    },
                    DEFINE => {
                        if arg.contains(&ap) {
                            Err(ZiaError::Loop("Definition is infinite".to_string()))
                        } else {
                            try!(self.define(arg, &ap));
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
    fn define(&mut self, before: &AbstractSyntaxTree, after: &AbstractSyntaxTree) -> ZiaResult<()> {
        if let Some(mut before_c) = before.get_concept() {
            if before == after {
                before_c.remove_definition();
                Ok(())
            } else {
                self.define2(&mut before_c, after)
            }
        } else if let Some((app, arg)) = before.get_expansion() {
            if let Some(mut after_c) = after.get_concept() {
                if let Some((mut ap, mut ar)) = after_c.get_definition() {
                    try!(self.define2(&mut ap, &app));
                    self.define2(&mut ar, &arg)
                } else {
                    after_c.insert_definition(
                        &mut try!(self.concept_from_ast(&app)),
                        &mut try!(self.concept_from_ast(&arg)),
                    );
                    Ok(())
                }
            } else {
                try!(self.concept_from_ast(&try!(AbstractSyntaxTree::from_monad(
                    after.get_token(),
                    app.clone(),
                    arg.clone(),
                ))));
                Ok(())
            }
        } else {
            return Err(ZiaError::Redundancy(
                "Refactoring a symbol that was never previously used is redundant".to_string(),
            ));
        }
    }
    fn add_string(&mut self, string_ref: &StringRef) {
        self.string_map
            .insert(string_ref.borrow().to_string(), string_ref.clone());
    }
    pub fn ast_from_expression(&mut self, s: &str) -> ZiaResult<AbstractSyntaxTree> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::Syntax(
                "Parentheses need to contain an expression".to_string(),
            )),
            1 => self.ast_from_atom(&tokens[0]),
            2 => {
                let parsed_tokens = parse_tokens(&tokens);
                self.ast_from_monad(parsed_tokens[0].clone(), parsed_tokens[1].clone())
            }
            _ => Err(ZiaError::Syntax(
                "Expression composed of more than 2 tokens has not been implemented yet"
                    .to_string(),
            )),
        }
    }
    fn ast_from_atom(&mut self, s: &str) -> ZiaResult<AbstractSyntaxTree> {
        let concept_if_exists = try!(self.concept_from_label(s));
        match concept_if_exists {
            None => Ok(AbstractSyntaxTree::from_atom(s)),
            Some(c) => Ok(AbstractSyntaxTree::from_token_and_concept(
                &Token::Atom(s.to_string()),
                &c,
            )),
        }
    }
    fn ast_from_monad(&mut self, app: Token, arg: Token) -> ZiaResult<AbstractSyntaxTree> {
        let applicand = try!(self.ast_from_token(&app));
        let argument = try!(self.ast_from_token(&arg));
        AbstractSyntaxTree::from_monad(app + arg, applicand, argument)
    }
    fn ast_from_token(&mut self, t: &Token) -> ZiaResult<AbstractSyntaxTree> {
        match *t {
            Token::Atom(ref s) => self.ast_from_atom(s),
            Token::Expression(ref s) => self.ast_from_expression(s),
        }
    }
    fn recursively_reduce(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<AbstractSyntaxTree> {
        match try!(self.reduce(ast)) {
            Some(ref a) => self.recursively_reduce(a),
            None => Ok(ast.clone()),
        }
    }
    fn reduce(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<Option<AbstractSyntaxTree>> {
        match ast.get_concept() {
            Some(ref c) => self.reduce_concept(c),
            None => match ast.get_expansion() {
                None => Ok(None),
                Some((app, arg)) => Context::match_app_arg(
                    try!(self.reduce(&app)),
                    try!(self.reduce(&arg)),
                    app.clone(),
                    arg.clone(),
                ),
            },
        }
    }
    fn reduce_concept(&mut self, c: &ConceptRef) -> ZiaResult<Option<AbstractSyntaxTree>> {
        match try!(c.get_normal_form()) {
            None => match c.get_definition() {
                Some((mut app, mut arg)) => {
                    let app_result = try!(self.reduce_concept(&app));
                    let arg_result = try!(self.reduce_concept(&arg));
                    Context::match_app_arg(
                        app_result,
                        arg_result,
                        try!(self.ast_from_concept(&app)),
                        try!(self.ast_from_concept(&arg)),
                    )
                }
                None => Ok(None),
            },
            Some(n) => Ok(Some(try!(self.ast_from_concept(&n)))),
        }
    }
    // Quite an ugly static method that I made to save myself from having to
    // write the same pattern twice in reduce and reduce_concept methods.
    fn match_app_arg(
        app: Option<AbstractSyntaxTree>,
        arg: Option<AbstractSyntaxTree>,
        original_app: AbstractSyntaxTree,
        original_arg: AbstractSyntaxTree,
    ) -> ZiaResult<Option<AbstractSyntaxTree>> {
        match (app, arg) {
            (None, None) => Ok(None),
            (Some(new_app), None) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                new_app.get_token() + original_arg.get_token(),
                new_app,
                original_arg,
            )))),
            (None, Some(new_arg)) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                original_app.get_token() + new_arg.get_token(),
                original_app,
                new_arg,
            )))),
            (Some(new_app), Some(new_arg)) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                new_app.get_token() + new_arg.get_token(),
                new_app,
                new_arg,
            )))),
        }
    }
    fn ast_from_concept(&self, c: &ConceptRef) -> ZiaResult<AbstractSyntaxTree> {
        Ok(AbstractSyntaxTree::from_token_and_concept(
            &try!(self.get_token(c)),
            c,
        ))
    }
}

impl ConceptTidyer<ConceptRef> for Context {
    fn remove_concept(&mut self, concept: &ConceptRef) {
        self.concepts.remove(concept.get_id());
    }
    fn correct_id(&mut self, id: usize) {
        self.concepts[id].set_id(id);
    }
}

impl ConceptNumber for Context {
    fn number_of_concepts(&self) -> usize {
        self.concepts.len()
    }
}

impl RefactorId<ConceptRef> for Context {}

impl LabelGetter<ConceptRef> for Context {
    fn get_label_concept(&self) -> ConceptRef {
        self.concepts[LABEL].clone()
    }
}

impl Unlabeller<ConceptRef> for Context {}

impl Refactor<ConceptRef> for Context {}

impl ConceptAdder<ConceptRef> for Context {
    fn add_concept(&mut self, concept: &ConceptRef) {
        self.concepts.push(concept.clone());
        if let ConceptRef::String(ref s) = concept {
            self.add_string(s);
        }
    }
}

impl StringMaker<ConceptRef> for Context {}

impl AbstractMaker<ConceptRef> for Context {}

impl Definer<ConceptRef> for Context {}

impl Labeller<ConceptRef> for Context {}

impl LabelledAbstractMaker<ConceptRef> for Context {}

impl SyntaxFinder<ConceptRef> for Context {
    fn get_string_concept(&self, s: &str) -> Option<ConceptRef> {
        match self.string_map.get(s) {
            None => None,
            Some(sc) => Some(ConceptRef::String(sc.clone())),
        }
    }
}

impl Definer2<ConceptRef, AbstractSyntaxTree> for Context {}

impl TokenHandler<ConceptRef> for Context {}

impl Expander<ConceptRef, AbstractSyntaxTree> for Context {}

impl ConceptMaker<ConceptRef, AbstractSyntaxTree> for Context {}

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new().unwrap();
    }
}
