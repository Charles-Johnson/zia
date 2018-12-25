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
along with this program. If not, see <http://www.gnu.org/licenses/>.*/

use std::fmt;

pub type ZiaResult<T> = Result<T, ZiaError>;

/// All the expected ways a Zia command could be invalid. 
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

impl fmt::Display for ZiaError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    match *self {
	        ZiaError::RedundantReduction => write!(f, "That reduction rule already exists."),
			ZiaError::RedundantDefinition => write!(f, "That definition already exists."),
			ZiaError::RedundantRefactor => write!(f, "Relabelling something that doesn't yet exist has no effect."),
	        ZiaError::NotAProgram => write!(f, "No program exists for this syntax."),
	        ZiaError::BadDefinition => write!(f, "Cannot define expressions."),
	        ZiaError::CyclicReduction => write!(f, "Cannot allow a chain of reduction rules to loop."),
	        ZiaError::ExpandingReduction => write!(f, "Cannot reduce a concept to an expression containing itself."),
	        ZiaError::InfiniteDefinition => write!(f, "Cannot define a concept as an expression containing itself."),
			ZiaError::EmptyParentheses => write!(f, "Parentheses need to contain a symbol or expression."),
			ZiaError::AmbiguousExpression => write!(f, "Ambiguity due to lack of precedence or associativity defined for the symbols in that expression."),
			ZiaError::DefinitionCollision => write!(f, "Cannot define a used symbol as another used symbol or expression."),
	    }
	}
}
