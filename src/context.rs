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
use concept::{AbstractConcept, ConceptRef, StringConcept, StringRef};
use constants::{DEFINE, LABEL, REDUCTION};
use std::collections::HashMap;
use std::rc::Rc;
use token::{parse_line, parse_tokens, Token};
use traits::{Application, Definition, Label, NormalForm, Reduction};
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
    fn setup(&mut self) -> ZiaResult<()> {
        self.new_abstract(); // for LABEL
        self.new_abstract(); // for DEFINE
        self.new_abstract(); // for REDUCTION
        let mut concepts = self.concepts.clone();
        try!(self.label(&mut concepts[DEFINE], ":=")); //two more ids occupied
        try!(self.label(&mut concepts[REDUCTION], "->")); //two more ids occupied
        Ok(())
    }
    pub fn call(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<String> {
        match ast.get_expansion() {
            Some((app, arg)) => if let Some(c) = arg.get_concept() {
                match c.get_id() {
                    REDUCTION => Ok(try!(self.recursively_reduce(&app)).get_token().as_string()),
                    DEFINE => Ok(try!(self.expand_ast_token(&app)).as_string()),
                    _ => self.call_as_applicand(&app, &arg),
                }
            } else {
                self.call_as_applicand(&app, &arg)
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
            Some((ap, ar)) => if let Some(arc) = ar.get_concept() {
                match arc.get_id() {
                    REDUCTION => {
                        try!(
                            try!(self.concept_from_ast(&ap))
                                .insert_reduction(&mut try!(self.concept_from_ast(arg)))
                        );
                        Ok("".to_string())
                    }
                    DEFINE => {
                        let mut argc = try!(self.concept_from_ast(arg));
                        let mut apc = try!(self.concept_from_ast(&ap));
                        try!(self.refactor(&mut argc, &mut apc,));
                        Ok("".to_string())
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
    fn concept_from_ast(&mut self, ast: &AbstractSyntaxTree) -> ZiaResult<ConceptRef> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            match ast.get_token() {
                Token::Atom(s) => {
                    let c = try!(self.new_labelled_abstract(&s));
                    Ok(c)
                }
                Token::Expression(_) => if let Some((mut app, mut arg)) = ast.get_expansion() {
                    let mut appc = try!(self.concept_from_ast(&app));
                    let mut argc = try!(self.concept_from_ast(&arg));
                    let def = try!(self.insert_definition(&mut appc, &mut argc));
                    Ok(def)
                } else {
                    panic!("AST with a expression token has no expansion!")
                },
            }
        }
    }
    fn new_labelled_abstract(&mut self, string: &str) -> ZiaResult<ConceptRef> {
        let mut new_abstract = self.new_abstract();
        try!(self.label(&mut new_abstract, string));
        Ok(new_abstract)
    }
    fn label(&mut self, concept: &mut ConceptRef, string: &str) -> ZiaResult<()> {
        let mut definition = self.new_abstract();
        self.label_safe(concept, &mut definition, string)
    }
    fn label_safe(
        &mut self,
        concept: &mut ConceptRef,
        definition: &mut ConceptRef,
        string: &str,
    ) -> ZiaResult<()> {
        let mut concepts = self.concepts.clone();
        let mut definition =
            try!(self.insert_definition_safe(&mut concepts[LABEL], concept, definition));
        let string_ref = self.new_string(string);
        definition.insert_reduction(&mut ConceptRef::String(string_ref))
    }
    fn insert_definition(
        &mut self,
        applicand: &mut ConceptRef,
        argument: &mut ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let mut definition = self.new_abstract();
        self.insert_definition_safe(applicand, argument, &mut definition)
    }
    fn insert_definition_safe(
        &mut self,
        applicand: &mut ConceptRef,
        argument: &mut ConceptRef,
        definition: &mut ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let application = try!(applicand.find_definition(&argument));
        match application {
            None => {
                definition.set_definition(applicand, argument);
                applicand.add_applicand_of(definition);
                argument.add_argument_of(definition);
                Ok(definition.clone())
            }
            Some(id) => Ok(id),
        }
    }
    fn new_abstract(&mut self) -> ConceptRef {
        let new_id = self.assign_new_id();
        let concept_ref = AbstractConcept::new_ref(new_id);
        self.concepts.push(ConceptRef::Abstract(concept_ref));
        self.concepts[new_id].clone()
    }
    fn new_string(&mut self, string: &str) -> StringRef {
        let new_id = self.assign_new_id();
        let string_ref = StringConcept::new_ref(new_id, string);
        self.string_map
            .insert(string.to_string(), string_ref.clone());
        self.concepts.push(ConceptRef::String(string_ref.clone()));
        string_ref
    }
    fn assign_new_id(&self) -> usize {
        self.concepts.len()
    }
    pub fn ast_from_expression(&mut self, s: &str) -> ZiaResult<Rc<AbstractSyntaxTree>> {
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
    fn ast_from_atom(&mut self, s: &str) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        let concept_if_exists = try!(self.concept_from_label(s));
        match concept_if_exists {
            None => Ok(AbstractSyntaxTree::from_atom(s)),
            Some(c) => Ok(AbstractSyntaxTree::from_token_and_concept(
                &Token::Atom(s.to_string()),
                &c,
            )),
        }
    }
    fn concept_from_label(&self, s: &str) -> ZiaResult<Option<ConceptRef>> {
        match self.string_map.get(s) {
            None => Ok(None),
            Some(c) => c.borrow().get_labellee(),
        }
    }
    fn ast_from_monad(&mut self, app: Token, arg: Token) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        let applicand = try!(self.ast_from_token(&app));
        let argument = try!(self.ast_from_token(&arg));
        AbstractSyntaxTree::from_monad(app + arg, &applicand, &argument)
    }
    fn ast_from_token(&mut self, t: &Token) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        match *t {
            Token::Atom(ref s) => self.ast_from_atom(s),
            Token::Expression(ref s) => self.ast_from_expression(s),
        }
    }
    fn recursively_reduce(
        &mut self,
        ast: &Rc<AbstractSyntaxTree>,
    ) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        match try!(self.reduce(ast)) {
            Some(a) => self.recursively_reduce(&a),
            None => Ok(ast.clone()),
        }
    }
    fn reduce(
        &mut self,
        ast: &Rc<AbstractSyntaxTree>,
    ) -> ZiaResult<Option<Rc<AbstractSyntaxTree>>> {
        match ast.get_concept() {
            Some(ref c) => {
                self.reduce_concept(c)
            }
            None => match ast.get_expansion() {
                None => Ok(None),
                Some((app, arg)) => Context::match_app_arg(
                    try!(self.reduce(&app)),
                    try!(self.reduce(&arg)),
                    &app,
                    &arg,
                ),
            },
        }
    }
    fn reduce_concept(&mut self, c: &ConceptRef) -> ZiaResult<Option<Rc<AbstractSyntaxTree>>> {
        match c.get_normal_form() {
            None => match c.get_definition() {
                Some((mut app, mut arg)) => {
                    let app_result = try!(self.reduce_concept(&app));
                    let arg_result = try!(self.reduce_concept(&arg));
                    Context::match_app_arg(
                        app_result.clone(),
                        arg_result.clone(),
                        &try!(self.ast_from_concept(&app)),
                        &try!(self.ast_from_concept(&arg)),
                    )
                }
                None => Ok(None),
            },
            Some(n) => {
                Ok(Some(try!(self.ast_from_concept(&n))))
            }
        }
    }
    // Quite an ugly static method that I made to save myself from having to write
    // the same pattern twice in reduce and reduce_concept methods.
    // Need to check whether a definition exists of the reduced application.
    fn match_app_arg(
        app: Option<Rc<AbstractSyntaxTree>>,
        arg: Option<Rc<AbstractSyntaxTree>>,
        original_app: &Rc<AbstractSyntaxTree>,
        original_arg: &Rc<AbstractSyntaxTree>,
    ) -> ZiaResult<Option<Rc<AbstractSyntaxTree>>> {
        match (app, arg) {
            (None, None) => Ok(None),
            (Some(new_app), None) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                new_app.get_token() + original_app.get_token(),
                &new_app,
                &original_arg,
            )))),
            (None, Some(new_arg)) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                original_app.get_token() + new_arg.get_token(),
                &original_app,
                &new_arg,
            )))),
            (Some(new_app), Some(new_arg)) => Ok(Some(try!(AbstractSyntaxTree::from_monad(
                new_app.get_token() + new_arg.get_token(),
                &new_app,
                &new_arg,
            )))),
        }
    }
    fn ast_from_concept(&self, c: &ConceptRef) -> ZiaResult<Rc<AbstractSyntaxTree>> {
        Ok(AbstractSyntaxTree::from_token_and_concept(
            &try!(self.get_token(c)),
            c,
        ))
    }
    fn expand_ast_token(&self, ast: &Rc<AbstractSyntaxTree>) -> ZiaResult<Token> {
        if let Some(con) = ast.get_concept() {
            self.expand_as_token(&con)
        } else if let Some((app2, arg2)) = ast.get_expansion() {
            Ok(try!(self.expand_ast_token(&app2)) + try!(self.expand_ast_token(&arg2)))
        } else {
            Ok(ast.get_token())
        }
    }
    fn expand_as_token(&self, c: &ConceptRef) -> ZiaResult<Token> {
        match c.get_definition() {
            Some((app, arg)) => self.join_tokens(&app, &arg),
            None => self.get_token(c),
        }
    }
    fn get_token(&self, c: &ConceptRef) -> ZiaResult<Token> {
        match try!(self.get_label(c)) {
            None => match c.get_definition() {
                Some((app, arg)) => self.join_tokens(&app, &arg),
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
            Some(s) => Ok(Token::Atom(s)),
        }
    }
    fn join_tokens(&self, app: &ConceptRef, arg: &ConceptRef) -> ZiaResult<Token> {
        Ok(try!(self.get_token(&app)) + try!(self.get_token(&arg)))
    }
    fn get_label(&self, c: &ConceptRef) -> ZiaResult<Option<String>> {
        Ok(match try!(self.concepts[LABEL].find_definition(c)) {
            None => None,
            Some(d) => match d.get_normal_form() {
                None => None,
                Some(n) => match n {
                    ConceptRef::String(s) => Some(s.borrow().get_string()),
                    _ => None,
                },
            },
        })
    }
    fn refactor(&mut self, before: &mut ConceptRef, after: &mut ConceptRef) -> ZiaResult<()> {
        try!(self.unlabel(before));
        self.refactor_id(before, after);
        Ok(())
    }
    fn unlabel(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        match try!(self.concepts[LABEL].find_definition(concept)) {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
    fn refactor_id(&mut self, before: &mut ConceptRef, after: &mut ConceptRef) {
        if self.concepts.len() > before.get_id() {
            after.refactor_from(before);
            self.concepts.remove(before.get_id());
            for id in before.get_id()..self.concepts.len() {
                self.concepts[id].set_id(id);
            }
        } else {
            panic!("refactoring id has gone wrong!")
        }
    }
}

#[cfg(test)]
mod context {
    use Context;
    #[test]
    fn new_context() {
        let _cont = Context::new();
    }
}
