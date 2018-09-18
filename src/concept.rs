use constants::{LUID, LABEL};
use std::cell::{RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use utils::{ZiaError, ZiaResult};

pub type ConceptRef = Rc<RefCell<Concept>>;

pub struct Concept {
    pub id: usize,
    pub definition: Option<(ConceptRef, ConceptRef)>,
    pub applicand_of: Vec<ConceptRef>,
    pub argument_of: Vec<ConceptRef>,
    pub normal_form: Option<ConceptRef>,
    pub reduces_from: Vec<ConceptRef>,
    pub string: Option<String>,
    pub integer: Option<usize>,
}

impl PartialEq for Concept {
    fn eq(&self, other: &Concept) -> bool {
        self.id == other.id
    }
}

impl Eq for Concept {}

impl Hash for Concept {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Concept {
    pub fn new(id: usize) -> Concept {
        Concept {
            id,
            definition: None,
            applicand_of: Vec::new(),
            argument_of: Vec::new(),
            normal_form: None,
            reduces_from: Vec::new(),
            string: None,
            integer: None,
        }
    }
    pub fn find_definition(&self, argument: &Concept) -> ZiaResult<Option<ConceptRef>> {
        let mut candidates: Vec<ConceptRef> = Vec::new();
        for candidate in self.applicand_of.clone() {
            if argument.argument_of.contains(&candidate)
                && !candidates.contains(&candidate)
            {
                candidates.push(candidate);
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple definitions with the same applicand and argument pair exist.".to_string(),
            )),
        }
    }
    pub fn insert_reduction(concept: &ConceptRef, normal_form: &ConceptRef, bm_normal_form: &mut RefMut<Concept>) -> ZiaResult<()> {
        let mut bm_concept = concept.borrow_mut();
        match bm_concept.normal_form {
            None => (),
            Some(_) => {
                return Err(ZiaError::Redundancy(format!(
                    "Reduction rule already exists for concept {:?}",
                    bm_concept.id
                )))
            }
        };
        let mut new_normal_form = normal_form.clone();
        match bm_normal_form.normal_form.clone() {
            None => (),
            Some(n) => {if n == concept // !Error! concept must be borrowed twice for this to true but not necessary
                {return Err(ZiaError::Loop("Cannot create a reduction loop".to_string()));
                } new_normal_form = n.clone()},
        };
        let prereductions = bm_concept.reduces_from.clone();
        for prereduction in prereductions {
            try!(Concept::update_normal_form(&prereduction, &mut prereduction.borrow_mut(), &new_normal_form, bm_normal_form));
        }
        Concept::insert_normal_form(&concept, &mut bm_concept, &new_normal_form, bm_normal_form) // !Error!
    }
    fn insert_normal_form(concept: &ConceptRef, bm_concept: &mut RefMut<Concept>, normal_form: &ConceptRef, bm_normal_form: &mut RefMut<Concept>) -> ZiaResult<()> {
        match bm_concept.normal_form {
            None => {
                if bm_normal_form.reduces_from.contains(concept) { // !Error! already mutably borrowed
                    Err(ZiaError::Redundancy(
                        "Normal form already reduces from this concept".to_string(),
                    ))
                } else {
                    Concept::update_normal_form(concept, bm_concept, normal_form, bm_normal_form)
                }
            }
            Some(_) => Err(ZiaError::Ambiguity(
                "Normal form already exists for this concept".to_string(),
            )),
        }
    }
    fn update_normal_form(concept: &ConceptRef, bm_concept: &mut RefMut<Concept>, normal_form: &ConceptRef, bm_normal_form: &mut RefMut<Concept>) -> ZiaResult<()> {
        bm_normal_form.reduces_from.push(concept.clone());
        bm_concept.normal_form = Some(normal_form.clone());
        Ok(())
    }
    pub fn get_labellee(&self) -> ZiaResult<Option<ConceptRef>> {
        let labels = &self.reduces_from;
        let mut candidates: Vec<ConceptRef> = Vec::new();
        for label in labels {
            match label.borrow().definition.clone() {
                None => continue,
                Some((r, x)) => {
                    match r.borrow().id {
                        LABEL => candidates.push(x),
                        _ => continue,
                    };
                }
            };
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(Some(candidates[0].clone())),
            _ => Err(ZiaError::Ambiguity(
                "Multiple concepts are labelled with the same string".to_string(),
            )),
        }
    }
    pub fn delete_normal_form(concept: &ConceptRef) -> ZiaResult<()> {
        let mut bm_concept = concept.borrow_mut();
        match bm_concept.normal_form.clone() {
            None => (),
            Some(n) => {
                n.borrow_mut().reduces_from.remove_item(concept);
                bm_concept.normal_form = None;
            }
        };
        Ok(())
    }
    pub fn new_luid() -> ConceptRef {
        let mut luid = Concept::new(LUID);
        luid.integer = Some(1);
        Rc::new(RefCell::new(luid))
    }
}
