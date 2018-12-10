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
use traits::call::label_getter::{FindDefinition, LabelGetter};

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

pub trait Display {
    fn to_string(&self) -> String;
}

pub trait Combine<T>
where
    Self: DisplayJoint + MaybeConcept<T> + Pair<T, Self> + Sized,
    T: FindDefinition<T> + Clone + PartialEq,
{
    fn combine_with(&self, other: &Self) -> Self {
        let left_string = self.display_joint();
        let right_string = other.display_joint();
        let definition = if let (Some(l), Some(r)) = (self.get_concept(), other.get_concept()) {
            l.find_definition(&r)
        } else {
            None
        };
        Self::from_pair(
            &(left_string + " " + &right_string),
            definition,
            self,
            other,
        )
    }
}

impl<T, U> Combine<T> for U
where
    U: DisplayJoint + MaybeConcept<T> + Pair<T, U> + Sized,
    T: FindDefinition<T> + Clone + PartialEq,
{
}

pub trait DisplayJoint {
    fn display_joint(&self) -> String;
}

pub trait MightExpand
where
    Self: Sized,
{
    fn get_expansion(&self) -> Option<(Self, Self)>;
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait Pair<T, U> {
    fn from_pair(&str, Option<T>, &U, &U) -> Self;
}

pub trait SyntaxFactory<T> {
    fn new(&str, Option<T>) -> Self;
}
