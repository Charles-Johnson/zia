use concept::{Concept, ConceptRef};
use constants::{DEFINE, LABEL, LUID, REDUCTION};
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
        cont.concepts.push(Concept::new_luid());
        try!(cont.setup());
        Ok(cont)
    }
    fn setup(&mut self) -> ZiaResult<()> {
        try!(self.new_concept()); // for LABEL
        try!(self.new_concept()); // for DEFINE
        try!(self.new_concept()); // for REDUCTION
        let luid_label_definition = try!(self.new_concept());
        let concepts = self.concepts.clone(); // for (LABEL LUID)
        try!(self.label_safe(&concepts[LUID], &luid_label_definition, "luid")); //one more id occupied. self.label(&concepts[LUID], "luid") requires a borrow of LUID once in insert_definition and again to get a new concept so self.label_safe is used instead.
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
        let definition = try!(self.new_concept());
        self.label_safe(concept, &definition, string)
    }
    fn insert_definition(
        &mut self,
        applicand: &ConceptRef,
        argument: &ConceptRef,
    ) -> ZiaResult<ConceptRef> {
        let definition = try!(self.new_concept());
        self.insert_definition_safe(applicand, argument, &definition)
    }
    fn insert_new_reduction(&mut self, concept: &ConceptRef) -> ZiaResult<ConceptRef> {
        let normal_form = try!(self.new_concept());
        try!(Concept::insert_reduction(
            concept,
            &normal_form,
            &mut normal_form.borrow_mut()
        ));
        Ok(normal_form)
    }
    fn new_concept(&mut self) -> ZiaResult<ConceptRef> {
        let new_id = try!(self.assign_new_id());
        let concept = Concept::new(new_id);
        self.concepts.push(Rc::new(RefCell::new(concept)));
        Ok(self.concepts[new_id].clone())
    }
    fn assign_new_id(&mut self) -> ZiaResult<usize> {
        let mut luid = self.concepts[LUID].borrow_mut();
        match luid.integer {
            None => Err(ZiaError::Absence(
                "Don't know what to do when an integer is undefined".to_string(),
            )),
            Some(i) => {
                luid.integer = Some(i + 1);
                Ok(i)
            }
        }
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
                let concept = try!(self.new_concept());
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
        match c.borrow().definition.clone() {
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
                    _ => self.call_as_applicand(&app, &arg, &mut bm_arg), // !Error!
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
            match try!(self.concepts[LUID].borrow().find_definition(&c.borrow())) {
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
                    try!(Concept::insert_reduction(&ap, arg, bm_arg)); // !Error! arg.normal_form gets borrowed again
                    Ok("".to_string())
                }
                DEFINE => {
                    try!(self.refactor(bm_arg, &ap));
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
    fn refactor(&mut self, before: &Concept, after: &ConceptRef) -> ZiaResult<()> {
        try!(self.unlabel(before));
        let a = after.borrow();
        self.refactor_id(before.id, a.id)
    }
    fn unlabel(&mut self, concept: &Concept) -> ZiaResult<()> {
        let luid = self.concepts[LUID].borrow();
        match try!(luid.find_definition(concept)).clone() {
            None => Ok(()),
            Some(d) => Concept::delete_normal_form(&d),
        }
    }
    fn refactor_id(&mut self, before: usize, after: usize) -> ZiaResult<()> {
        let luid_ref = self.concepts[LUID].clone();
        let luid = luid_ref.borrow();
        if before < luid.id {
            let concept_ref = self.concepts[before].clone();
            let mut concept = concept_ref.borrow_mut();
            concept.id = after;
            self.concepts[after] = self.concepts[before].clone();
            try!(self.refactor_id(before + 1, before));
        }
        Ok(())
    }
}

#[cfg(test)]
mod context {
    use concept::Concept;
    use constants::{DEFINE, LUID, REDUCTION};
    use std::collections::HashMap;
    use Context;
    #[test]
    fn new_context() {
        let mut cont = Context {
            string_map: HashMap::new(),
            concepts: Vec::new(),
        };
        cont.concepts.push(Concept::new_luid());
        cont.new_concept().unwrap(); // LABEL
        cont.new_concept().unwrap(); // DEFINE
        cont.new_concept().unwrap(); // REDUCTION
        let luid_label_definition = cont.new_concept().unwrap();
        let concepts = cont.concepts.clone();
        cont.label_safe(&concepts[LUID], &luid_label_definition, "luid")
            .unwrap(); //two more ids occupied
        cont.label(&concepts[DEFINE], ":=").unwrap(); //two more ids occupied
        cont.label(&concepts[REDUCTION], "->").unwrap(); //two more ids occupied
    }
}
