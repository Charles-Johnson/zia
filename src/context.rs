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
use concept::{AbstractConcept, ConceptRef, StringConcept, StringRef};
use constants::{DEFINE, LABEL, REDUCTION};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use token::{parse_line, parse_tokens, Token};
use traits::{Application, Definition, Label, NormalForm, Reduction};
use utils::{ZiaError, ZiaResult};

pub struct Context {
    pub string_map: HashMap<String, Rc<RefCell<StringConcept>>>,
    pub concepts: Vec<ConceptRef>,
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
    fn insert_definition_safe(
        &mut self,
        applicand: &mut ConceptRef,
        argument: &mut ConceptRef,
        definition: &mut ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let application = try!(applicand.find_definition(&argument));
        match application {
            None => {
                println!(
                    "Setting definition of concept {:?} as ({:?}, {:?})",
                    definition.get_id(),
                    applicand.get_id(),
                    argument.get_id()
                );
                definition.set_definition(applicand, argument);
                applicand.add_applicand_of(definition);
                argument.add_argument_of(definition);
                Ok(definition.clone())
            }
            Some(id) => Ok(id),
        }
    }
    fn label(&mut self, concept: &mut ConceptRef, string: &str) -> ZiaResult<()> {
        let mut definition = self.new_abstract();
        self.label_safe(concept, &mut definition, string)
    }
    fn insert_definition(
        &mut self,
        applicand: &mut ConceptRef,
        argument: &mut ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let mut definition = self.new_abstract();
        self.insert_definition_safe(applicand, argument, &mut definition)
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
    pub fn concept_from_expression(&mut self, s: &str) -> ZiaResult<ConceptRef> {
        let tokens: Vec<String> = parse_line(s);
        match tokens.len() {
            0 => Err(ZiaError::Syntax(
                "Parentheses need to contain an expression".to_string(),
            )),
            1 => self.concept_from_atom(&tokens[0]),
            2 => {
                let parsed_tokens = parse_tokens(&tokens);
                self.concept_from_monad(&parsed_tokens[0], &parsed_tokens[1])
            }
            _ => Err(ZiaError::Syntax(
                "Expression composed of more than 2 tokens has not been implemented yet"
                    .to_string(),
            )),
        }
    }
    fn concept_from_atom(&mut self, s: &str) -> ZiaResult<ConceptRef> {
        let concept_if_exists = try!(self.concept_from_label(s));
        match concept_if_exists {
            None => {
                let mut concept = self.new_abstract();
                try!(self.label(&mut concept, s));
                Ok(concept)
            }
            Some(c) => Ok(c),
        }
    }
    fn concept_from_label(&self, s: &str) -> ZiaResult<Option<ConceptRef>> {
        match self.string_map.get(s) {
            None => Ok(None),
            Some(c) => c.borrow().get_labellee(),
        }
    }
    fn concept_from_monad(&mut self, app: &Token, arg: &Token) -> ZiaResult<ConceptRef> {
        let mut applicand = try!(self.concept_from_token(app));
        let mut argument = try!(self.concept_from_token(arg));
        self.insert_definition(&mut applicand, &mut argument)
    }
    fn concept_from_token(&mut self, t: &Token) -> ZiaResult<ConceptRef> {
        match *t {
            Token::Atom(ref s) => self.concept_from_atom(s),
            Token::Expression(ref s) => self.concept_from_expression(s),
        }
    }
    pub fn call(&mut self, c: &ConceptRef) -> ZiaResult<String> {
        match c.get_definition() {
            Some((app, mut arg)) => match arg.get_id() {
                REDUCTION => {
                    let reduced_app = match try!(self.reduce(&app.clone())) {
                        None => app,
                        Some(a) => a,
                    };
                    match try!(self.get_token(&reduced_app)) {
                        Token::Expression(s) | Token::Atom(s) => Ok(s),
                    }
                }
                DEFINE => match try!(self.expand_as_token(&app)) {
                    Token::Expression(s) | Token::Atom(s) => Ok(s),
                },
                _ => self.call_as_applicand(&app, &mut arg),
            },
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
    fn reduce(&mut self, c: &ConceptRef) -> ZiaResult<Option<ConceptRef>> {
        match c.get_normal_form() {
            None => match c.get_definition() {
                Some((mut app, mut arg)) => {
                    let app_result = try!(self.reduce(&app));
                    let arg_result = try!(self.reduce(&arg));
                    match (app_result.clone(), arg_result.clone()) {
                        (None, None) => Ok(None),
                        (None, Some(mut ar)) => {
                            let application = try!(self.insert_definition(&mut app, &mut ar));
                            self.reduce(&application)
                        }
                        (Some(mut ap), None) => {
                            let application = try!(self.insert_definition(&mut ap, &mut arg));
                            self.reduce(&application)
                        }
                        (Some(mut ap), Some(mut ar)) => {
                            let application = try!(self.insert_definition(&mut ap, &mut ar));
                            self.reduce(&application)
                        }
                    }
                }
                None => Ok(None),
            },
            Some(n) => Ok(Some(n)),
        }
    }
    fn expand_as_token(&self, c: &ConceptRef) -> ZiaResult<Token> {
        println!("Expanding token for concept {:?}", c.get_id());
        match c.get_definition() {
            Some((app, arg)) => self.join_tokens(&app, &arg),
            None => self.get_token(c),
        }
    }
    fn get_token(&self, c: &ConceptRef) -> ZiaResult<Token> {
        println!("Getting token for concept {:?}", c.get_id());
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
        println!(
            "Joining tokens of concepts {:?} and {:?}",
            app.get_id(),
            arg.get_id()
        );
        Ok(Token::Expression(
            try!(self.add_token(app)) + " " + &try!(self.add_token(arg)),
        ))
    }
    fn add_token(&self, concept: &ConceptRef) -> ZiaResult<String> {
        Ok(match try!(self.get_token(concept)) {
            Token::Atom(s) => s,
            Token::Expression(s) => "(".to_string() + &s + ")",
        })
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
    fn call_as_applicand(&mut self, app: &ConceptRef, arg: &mut ConceptRef) -> ZiaResult<String> {
        match app.get_definition() {
            Some((mut ap, ar)) => match ar.get_id() {
                REDUCTION => {
                    try!(ConceptRef::insert_reduction(&mut ap, arg));
                    Ok("".to_string())
                }
                DEFINE => {
                    try!(self.refactor(arg, &mut ap)); // This refactoring doesn't work apart from refactoring the ids and removing the previous label. ap needs to inherit the definition of arg.
                    Ok("".to_string())
                }
                _ => Err(ZiaError::Absence(
                    "This concept is not a program".to_string(),
                )),
            },
            None => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
    fn refactor(&mut self, before: &mut ConceptRef, after: &mut ConceptRef) -> ZiaResult<()> {
        try!(self.unlabel(before));
        if before.get_id() < after.get_id() {
            self.refactor_id(after, before.get_id());
            Ok(())
        } else {
            self.refactor_id(before, after.get_id());
            Ok(())
        }
    }
    fn unlabel(&mut self, concept: &ConceptRef) -> ZiaResult<()> {
        match try!(self.concepts[LABEL].find_definition(concept)) {
            None => Ok(()),
            Some(mut d) => d.delete_normal_form(),
        }
    }
    fn refactor_id(&mut self, before: &mut ConceptRef, after: usize) {
        if self.concepts.len() > before.get_id() {
            before.set_id(after);
            let mut concepts = self.concepts.clone();
            self.concepts[after] = concepts[before.get_id()].clone();
            self.concepts.remove(before.get_id());
            for (id, concept) in concepts.iter_mut().enumerate().skip(before.get_id()) {
                concept.set_id(id);
            }
        } else {
            panic!("refactoring id has gone wrong!")
        }
    }
}

#[cfg(test)]
mod context {
    use constants::{DEFINE, REDUCTION};
    use std::collections::HashMap;
    use Context;
    #[test]
    fn new_context() {
        let mut cont = Context {
            string_map: HashMap::new(),
            concepts: Vec::new(),
        };
        cont.new_abstract(); // LABEL
        cont.new_abstract(); // DEFINE
        cont.new_abstract(); // REDUCTION
        let mut concepts = cont.concepts.clone();
        cont.label(&mut concepts[DEFINE], ":=").unwrap(); //two more ids occupied
        cont.label(&mut concepts[REDUCTION], "->").unwrap(); //two more ids occupied
    }
}
