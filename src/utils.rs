use std::cell::{BorrowError, BorrowMutError};
use std::fmt;

pub type ZiaResult<T> = Result<T, ZiaError>;

#[derive(Debug)]
pub enum ZiaError {
    Borrow(BorrowError),
    BorrowMut(BorrowMutError),
    Ambiguity(String),
    Redundancy(String),
    Absence(String),
    Syntax(String),
    Loop(String),
}

impl From<BorrowError> for ZiaError {
    fn from(error: BorrowError) -> Self {
        ZiaError::Borrow(error)
    }
}

impl From<BorrowMutError> for ZiaError {
    fn from(error: BorrowMutError) -> Self {
        ZiaError::BorrowMut(error)
    }
}

impl fmt::Display for ZiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZiaError::Borrow(ref b) => b.fmt(f),
            ZiaError::BorrowMut(ref b) => b.fmt(f),
            ZiaError::Ambiguity(ref s) => write!(f, "{}", s),
            ZiaError::Redundancy(ref s) => write!(f, "{}", s),
            ZiaError::Absence(ref s) => write!(f, "{}", s),
            ZiaError::Syntax(ref s) => write!(f, "{}", s),
            ZiaError::Loop(ref s) => write!(f, "{}", s),
        }
    }
}
