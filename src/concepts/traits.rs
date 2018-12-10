use traits::call::label_getter::LabelGetter;

impl<T: LabelGetter> Display for T {
    fn to_string(&self) -> String {
        match self.get_string() {
            Some(s) => "\"".to_string() + &s + "\"",
            None => match self.get_label() {
                Some(l) => l,
                None => match self.get_definition() {
                    Some((left, right)) => {
                        let mut left_string = left.to_string();
                        if left_string.contains(' ') {
                            left_string = "(".to_string() + &left_string;
                        }
                        let mut right_string = right.to_string();
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

pub trait GetId {
    fn get_id(&self) -> usize;
}

pub trait SetId {
    fn set_id(&mut self, id: usize);
}

pub trait GetDefinition<T> {
    fn get_definition(&self) -> Option<(T, T)>;
}

pub trait GetReduction<T> {
    fn get_reduction(&self) -> Option<T>;
}

pub trait FindWhatReducesToIt<T> {
    fn find_what_reduces_to_it(&self) -> Vec<T>;
}

pub trait RemoveReduction<T> {
    fn make_reduce_to_none(&mut self);
    fn no_longer_reduces_from(&mut self, &T);
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_as_lefthand_of(&mut self, &T);
    fn add_as_righthand_of(&mut self, &T);
}

pub trait SetReduction<T> {
    fn make_reduce_to(&mut self, &T);
    fn make_reduce_from(&mut self, &T);
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_as_lefthand_of(&mut self, &T);
    fn remove_as_righthand_of(&mut self, &T);
}

pub trait MaybeString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}

pub trait ConvertTo<T> {
    fn convert(&self) -> Option<T>;
}

pub trait Display {
    fn to_string(&self) -> String;
}
