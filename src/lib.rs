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

mod ast;
mod combined_traits;
mod concept_and_ast_traits;
mod concepts;
mod constants;
mod context;
mod token;
mod traits;
mod utils;

use ast::AbstractSyntaxTree as GenericAbstractSyntaxTree;
pub use ast::traits::Display;
use concepts::{ConceptRef, StringConcept};
use context::Context as GenericContext;
pub use combined_traits::{ContextMaker, Execute};
pub use utils::ZiaError;

pub type AbstractSyntaxTree = GenericAbstractSyntaxTree<ConceptRef>;
pub type Context = GenericContext<ConceptRef, StringConcept<ConceptRef>>;
