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
use std::fmt;

pub type ZiaResult<T> = Result<T, ZiaError>;

#[derive(Debug)]
pub enum ZiaError {
    Borrow(String),
    BorrowMut(String),
    Ambiguity(String),
    Redundancy(String),
    Absence(String),
    Syntax(String),
    Loop(String),
}

impl fmt::Display for ZiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZiaError::Borrow(ref s) => write!(f, "{}", s),
            ZiaError::BorrowMut(ref s) => write!(f, "{}", s),
            ZiaError::Ambiguity(ref s) => write!(f, "{}", s),
            ZiaError::Redundancy(ref s) => write!(f, "{}", s),
            ZiaError::Absence(ref s) => write!(f, "{}", s),
            ZiaError::Syntax(ref s) => write!(f, "{}", s),
            ZiaError::Loop(ref s) => write!(f, "{}", s),
        }
    }
}
