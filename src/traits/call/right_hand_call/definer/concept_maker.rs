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
use concepts::Display;
use traits::call::label_getter::GetDefinitionOf;
use traits::call::right_hand_call::definer::labeller::{
    AbstractFactory, InsertDefinition, Labeller, StringFactory, UpdateNormalForm,
};
use traits::call::{GetNormalForm, MaybeConcept, MightExpand};
use utils::ZiaResult;

pub trait ConceptMaker<T>
where
    T: StringFactory
        + AbstractFactory
        + InsertDefinition
        + GetNormalForm
        + UpdateNormalForm
        + GetDefinitionOf<T>,
    Self: Labeller<T>,
{
    fn concept_from_ast<U: MaybeConcept<T> + MightExpand + Display>(
        &mut self,
        ast: &U,
    ) -> ZiaResult<T> {
        if let Some(c) = ast.get_concept() {
            Ok(c)
        } else {
            let string = &ast.to_string();
            match ast.get_expansion() {
                None => self.new_labelled_abstract(string),
                Some((ref left, ref right)) => {
                    let mut appc = try!(self.concept_from_ast(left));
                    let mut argc = try!(self.concept_from_ast(right));
                    let mut concept = try!(self.find_or_insert_definition(&mut appc, &mut argc));
                    if !string.contains(' ') {
                        try!(self.label(&mut concept, string));
                    }
                    Ok(concept)
                }
            }
        }
    }
}
