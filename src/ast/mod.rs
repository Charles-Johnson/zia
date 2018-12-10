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
mod expression;
mod symbol;

use self::expression::Expression;
use self::symbol::Symbol;
use concepts::traits::Display;
use traits::{
    call::{
        label_getter::FindDefinition, right_hand_call::definer::Pair, MaybeConcept, MightExpand,
    },
    SyntaxFactory,
};

pub enum AbstractSyntaxTree<T> {
    Symbol(Symbol<T>),
    Expression(Expression<AbstractSyntaxTree<T>, T>),
}

impl<T: Clone> MaybeConcept<T> for AbstractSyntaxTree<T> {
    fn get_concept(&self) -> Option<T> {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => a.get_concept(),
            AbstractSyntaxTree::Expression(ref e) => e.get_concept(),
        }
    }
}

impl<T> PartialEq for AbstractSyntaxTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<T: Clone> Clone for AbstractSyntaxTree<T> {
    fn clone(&self) -> AbstractSyntaxTree<T> {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => AbstractSyntaxTree::Symbol(a.clone()),
            AbstractSyntaxTree::Expression(ref e) => AbstractSyntaxTree::Expression(e.clone()),
        }
    }
}

impl<T: Clone> MightExpand for AbstractSyntaxTree<T> {
    fn get_expansion(&self) -> Option<(AbstractSyntaxTree<T>, AbstractSyntaxTree<T>)> {
        match *self {
            AbstractSyntaxTree::Symbol(_) => None,
            AbstractSyntaxTree::Expression(ref e) => Some((e.get_lefthand(), e.get_righthand())),
        }
    }
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

impl<T> DisplayJoint for AbstractSyntaxTree<T> {
    fn display_joint(&self) -> String {
        match *self {
            AbstractSyntaxTree::Expression(ref e) => "(".to_string() + &e.to_string() + ")",
            AbstractSyntaxTree::Symbol(ref a) => a.to_string(),
        }
    }
}

impl<T> Display for AbstractSyntaxTree<T> {
    fn to_string(&self) -> String {
        match *self {
            AbstractSyntaxTree::Symbol(ref a) => a.to_string(),
            AbstractSyntaxTree::Expression(ref e) => e.to_string(),
        }
    }
}

impl<T: Clone> Pair<T, AbstractSyntaxTree<T>> for AbstractSyntaxTree<T> {
    fn from_pair(
        syntax: &str,
        concept: Option<T>,
        lefthand: &AbstractSyntaxTree<T>,
        righthand: &AbstractSyntaxTree<T>,
    ) -> AbstractSyntaxTree<T> {
        AbstractSyntaxTree::Expression(Expression::<AbstractSyntaxTree<T>, T>::from_pair(
            syntax, concept, lefthand, righthand,
        ))
    }
}

impl<T> SyntaxFactory<T> for AbstractSyntaxTree<T> {
    fn new(s: &str, concept: Option<T>) -> AbstractSyntaxTree<T> {
        AbstractSyntaxTree::Symbol(Symbol::<T>::new(s, concept))
    }
}
