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
use std::marker;
use std::ops::Add;
use token::Token;
use utils::ZiaResult;

pub trait GetNormalForm<T>
where
    Self: marker::Sized,
{
    fn get_normal_form(&self) -> ZiaResult<Option<T>>;
}

pub trait GetNormalFormOf<T> {
    fn get_normal_form_of(&self) -> Vec<T>;
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

pub trait Id {
    fn get_id(&self) -> usize;
}

pub trait GetDefinition<T> {
    fn get_definition(&self) -> Option<(T, T)>;
}

pub trait GetDefinitionOf<T> {
    fn get_lefthand_of(&self) -> Vec<T>;
    fn get_righthand_of(&self) -> Vec<T>;
}

pub trait SetDefinition<T> {
    fn set_definition(&mut self, &T, &T);
    fn add_lefthand_of(&mut self, &T);
    fn add_righthand_of(&mut self, &T);
}

pub trait RemoveDefinition<T> {
    fn remove_definition(&mut self);
    fn remove_lefthand_of(&mut self, &T);
    fn remove_righthand_of(&mut self, &T);
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

pub trait StringFactory {
    fn new_string(usize, &str) -> Self;
}

pub trait ConceptAdder<T> {
    fn add_concept(&mut self, &T);
}

pub trait AbstractFactory {
    fn new_abstract(usize) -> Self;
}

pub trait HasToken {
    fn get_token(&self) -> Token;
}

pub trait MaybeConcept<T> {
    fn get_concept(&self) -> Option<T>;
}

pub trait MightExpand
where
    Self: marker::Sized,
{
    fn get_expansion(&self) -> Option<(Self, Self)>;
}

pub trait Pair
where
    Self: marker::Sized + Clone,
{
    fn from_pair(Token, &Self, &Self) -> ZiaResult<Self>;
}

pub trait SyntaxFactory<T> {
    fn new(&str, Option<T>) -> Self;
}

pub trait MatchLeftRight
where
    Self: Clone + Add<Self, Output = ZiaResult<Self>>,
{
    fn match_left_right(
        left: Option<Self>,
        right: Option<Self>,
        original_left: &Self,
        original_right: &Self,
    ) -> ZiaResult<Option<Self>> {
        match (left, right) {
            (None, None) => Ok(None),
            (Some(new_left), None) => Ok(Some(try!(new_left + original_right.clone()))),
            (None, Some(new_right)) => Ok(Some(try!(original_left.clone() + new_right))),
            (Some(new_left), Some(new_right)) => Ok(Some(try!(new_left + new_right))),
        }
    }
}
