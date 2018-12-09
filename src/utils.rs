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

pub type ZiaResult<T> = Result<T, ZiaError>;

#[derive(Debug)]
pub enum ZiaError {
    RedundantReduction,
    RedundantDefinition,
    RedundantRefactor,
    NotAProgram,
    BadDefinition,
    CyclicReduction,
    ExpandingReduction,
    InfiniteDefinition,
    EmptyParentheses,
    AmbiguousExpression,
    DefinitionCollision,
}

impl Display for ZiaError {
    fn to_string(&self) -> String {
        match *self {
            ZiaError::RedundantReduction => "That reduction rule already exists.".to_string(),
			ZiaError::RedundantDefinition => "That definition already exists.".to_string(),
			ZiaError::RedundantRefactor => "Relabelling something that doesn't yet exist has no effect.".to_string(),
            ZiaError::NotAProgram => "No program exists for this syntax.".to_string(),
            ZiaError::BadDefinition => "Cannot define expressions.".to_string(),
            ZiaError::CyclicReduction => "Cannot allow a chain of reduction rules to loop.".to_string(),
            ZiaError::ExpandingReduction => "Cannot reduce a concept to an expression containing itself.".to_string(),
            ZiaError::InfiniteDefinition => "Cannot define a concept as an expression containing itself.".to_string(),
			ZiaError::EmptyParentheses => "Parentheses need to contain a symbol or expression.".to_string(),
			ZiaError::AmbiguousExpression => "Ambiguity due to lack of precedence or associativity defined for the symbols in that expression.".to_string(),
			ZiaError::DefinitionCollision => "Cannot define a used symbol as another used symbol or expression.".to_string(),
        }
    }
}
