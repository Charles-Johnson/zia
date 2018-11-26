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

pub mod call;
pub mod syntax_converter;

use self::call::{
    GetNormalForm, HasToken,
    MaybeConcept, MightExpand,
};
use self::call::label_getter::{FindDefinition, LabelGetter};
use self::call::left_hand_call::{ConceptAdder, ConceptNumber};

pub trait Id {
    fn get_id(&self) -> usize;
}

pub trait SyntaxFactory<T> {
    fn new(&str, Option<T>) -> Self;
}

pub trait GetDefinition<T> {
    fn get_definition(&self) -> Option<(T, T)>;
}
