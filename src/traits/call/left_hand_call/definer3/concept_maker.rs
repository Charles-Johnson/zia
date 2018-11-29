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
use token::Token;
use traits::call::left_hand_call::definer3::labeller::{
    AbstractFactory, InsertDefinition, Labeller, StringFactory, UpdateNormalForm,
};
use traits::call::{GetNormalForm, HasToken, LabelGetter, MaybeConcept, MightExpand};
use utils::ZiaResult;

pub trait ConceptMaker<T, U>
where
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + GetNormalForm<T>
        + UpdateNormalForm
        + LabelGetter,
    U: MaybeConcept<T> + HasToken + MightExpand,
    Self: Labeller<T>,
{
    fn concept_from_ast(&mut self, ast: &U) -> ZiaResult<T> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let mut c = match ast.get_token() {
                Token::Atom(s) => try!(self.new_labelled_abstract(&s)),
                Token::Expression(_) => self.new_abstract(),
            };
            if let Some((mut app, mut arg)) = ast.get_expansion() {
                let mut appc = try!(self.concept_from_ast(&app));
                let mut argc = try!(self.concept_from_ast(&arg));
                c.insert_definition(&mut appc, &mut argc);
            }
            Ok(c)
        }
    }
}

impl<S, T, U> ConceptMaker<T, U> for S
where
    T: StringFactory + AbstractFactory + InsertDefinition + UpdateNormalForm + LabelGetter,
    U: MaybeConcept<T> + HasToken + MightExpand,
    S: Labeller<T>,
{}
