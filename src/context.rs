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
use concept::{Concept, ConceptRef};
use constants::{DEFINE, LABEL, REDUCTION};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use token::{parse_line, parse_tokens, Token};
use utils::{ZiaError, ZiaResult};

pub struct Context {
    pub string_map: HashMap<String, ConceptRef>,
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
        self.new_concept(); // for LABEL
        self.new_concept(); // for DEFINE
        self.new_concept(); // for REDUCTION
        let concepts = self.concepts.clone();
        try!(self.label(&concepts[DEFINE], ":=")); //two more ids occupied
        try!(self.label(&concepts[REDUCTION], "->")); //two more ids occupied
        Ok(())
    }
    fn label_safe(
        &mut self,
        concept: &ConceptRef,
        definition: &ConceptRef,
        string: &str,
    ) -> ZiaResult<()> {
        let concepts = self.concepts.clone();
        let definition = try!(self.insert_definition_safe(&concepts[LABEL], concept, definition));
        let text = try!(self.insert_new_reduction(&definition));
        try!(self.insert_text(&text, string));
        Ok(())
    }
    fn insert_definition_safe(
        &mut self,
        applicand: &ConceptRef,
        argument: &ConceptRef,
        definition: &ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let mut bm_applicand = applicand.borrow_mut();
        let mut bm_argument = argument.borrow_mut();
        let application = try!(bm_applicand.find_definition(&bm_argument));
        match application {
            None => {
                let mut bm_definition = definition.borrow_mut();
                bm_definition.definition = Some((applicand.clone(), argument.clone()));
                bm_applicand.applicand_of.push(definition.clone());
                bm_argument.argument_of.push(definition.clone());
                Ok(definition.clone())
            }
            Some(id) => Ok(id),
        }
    }
    fn label(&mut self, concept: &ConceptRef, string: &str) -> ZiaResult<()> {
        let definition = self.new_concept();
        self.label_safe(concept, &definition, string)
    }
    fn insert_definition(
        &mut self,
        applicand: &ConceptRef,
        argument: &ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let definition = self.new_concept();
        self.insert_definition_safe(applicand, argument, &definition)
    }
    fn insert_new_reduction(&mut self, concept: &ConceptRef) -> ZiaResult<ConceptRef> {
        let normal_form = self.new_concept();
        try!(Concept::insert_reduction(
            concept,
            &normal_form,
            &mut normal_form.borrow_mut()
        ));
        Ok(normal_form)
    }
    fn new_concept(&mut self) -> ConceptRef {
        let new_id = self.assign_new_id();
        let concept = Concept::new(new_id);
        self.concepts.push(Rc::new(RefCell::new(concept)));
        self.concepts[new_id].clone()
    }
    fn assign_new_id(&self) -> usize {
        self.concepts.len()
    }
    fn insert_text(&mut self, concept: &ConceptRef, result: &str) -> ZiaResult<()> {
        let mut bm_concept = concept.borrow_mut();
        match bm_concept.string {
            None => {
                bm_concept.string = Some(result.to_string());
                self.string_map.insert(result.to_string(), concept.clone());
                Ok(())
            }
            Some(_) => Err(ZiaError::Ambiguity("String already defined".to_string())),
        }
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
                let concept = self.new_concept();
                try!(self.label(&concept, s));
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
        let applicand = try!(self.concept_from_token(app));
        let argument = try!(self.concept_from_token(arg));
        self.insert_definition(&applicand, &argument)
    }
    fn concept_from_token(&mut self, t: &Token) -> ZiaResult<ConceptRef> {
        match *t {
            Token::Atom(ref s) => self.concept_from_atom(s),
            Token::Expression(ref s) => self.concept_from_expression(s),
        }
    }
    pub fn call(&mut self, c: &ConceptRef) -> ZiaResult<String> {
        let b_c = c.borrow();
        println!("{:?}", b_c.id);
        match b_c.definition.clone() {
            Some((app, arg)) => {
                let mut bm_arg = arg.borrow_mut();
                match bm_arg.id {
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
                    _ => self.call_as_applicand(&app, &arg, &mut bm_arg),
                }
            }
            _ => Err(ZiaError::Absence(
                "This concept is not a program".to_string(),
            )),
        }
    }
    fn reduce(&mut self, c: &ConceptRef) -> ZiaResult<Option<ConceptRef>> {
        let bc = c.borrow();
        match bc.normal_form.clone() {
            None => match bc.definition.clone() {
                Some((app, arg)) => {
                    let app_result = try!(self.reduce(&app));
                    let arg_result = try!(self.reduce(&arg));
                    match (app_result.clone(), arg_result.clone()) {
                        (None, None) => Ok(None),
                        (None, Some(ar)) => {
                            let application = try!(self.insert_definition(&app, &ar));
                            self.reduce(&application)
                        }
                        (Some(ap), None) => {
                            let application = try!(self.insert_definition(&ap, &arg));
                            self.reduce(&application)
                        }
                        (Some(ap), Some(ar)) => {
                            let application = try!(self.insert_definition(&ap, &ar));
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
        match c.borrow().definition.clone() {
            Some((app, arg)) => self.join_tokens(&app, &arg),
            None => self.get_token(c),
        }
    }
    fn get_token(&self, c: &ConceptRef) -> ZiaResult<Token> {
        match try!(self.get_label(c)) {
            None => match c.borrow().definition.clone() {
                Some((app, arg)) => self.join_tokens(&app, &arg),
                None => Err(ZiaError::Absence(
                    "Unlabelled concept with no definition".to_string(),
                )),
            },
            Some(s) => Ok(Token::Atom(s)),
        }
    }
    fn join_tokens(&self, app: &ConceptRef, arg: &ConceptRef) -> ZiaResult<Token> {
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
        Ok(
            match try!(self.concepts[LABEL].borrow().find_definition(&c.borrow())) {
                None => None,
                Some(d) => match d.borrow().normal_form.clone() {
                    None => None,
                    Some(n) => n.borrow().string.clone(),
                },
            },
        )
    }
    fn call_as_applicand(
        &mut self,
        app: &ConceptRef,
        arg: &ConceptRef,
        bm_arg: &mut RefMut<Concept>,
    ) -> ZiaResult<String> {
        let bapp = app.borrow();
        match bapp.definition.clone() {
            Some((ap, ar)) => match ar.borrow().id {
                REDUCTION => {
                    try!(Concept::insert_reduction(&ap, arg, bm_arg));
                    Ok("".to_string())
                }
                DEFINE => {
                    let mut bm_ap = ap.borrow_mut();
                    println!("{:?}", DEFINE);
                    try!(self.refactor(bm_arg, &mut bm_ap)); // This refactoring doesn't work apart from refactoring the ids and removing the previous label. ap needs to inherit the definition of arg.
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
    fn refactor(&mut self, before: &mut RefMut<Concept>, after: &mut RefMut<Concept>) -> ZiaResult<()> {
        try!(self.unlabel(before));
        println!("{:?},{:?}", before.id, after.id);
        if before.id < after.id {
            Ok(self.refactor_id(after, before.id))
        } else {
            Ok(self.refactor_id(before, after.id))
        }
    }
    fn unlabel(&mut self, concept: &Concept) -> ZiaResult<()> {
        let label = self.concepts[LABEL].borrow();
        match try!(label.find_definition(concept)).clone() {
            None => Ok(()),
            Some(d) => Concept::delete_normal_form(&d),
        }
    }
    fn refactor_id(&mut self, before: &mut RefMut<Concept>, after: usize) {
        if self.concepts.len() > before.id {
            before.id = after;
            let concepts = self.concepts.clone();
            self.concepts[after] = concepts[before.id].clone(); 
            self.concepts.remove(before.id);
            for id in before.id .. concepts.len() {
                let mut concept = concepts[id].borrow_mut();
                concept.id = id;
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
        cont.new_concept(); // LABEL
        cont.new_concept(); // DEFINE
        cont.new_concept(); // REDUCTION
        let concepts = cont.concepts.clone();
        cont.label(&concepts[DEFINE], ":=").unwrap(); //two more ids occupied
        cont.label(&concepts[REDUCTION], "->").unwrap(); //two more ids occupied
    }
}
