#[derive(Debug, PartialEq, Eq)]
pub enum AdventError {
    ParseError(String),
    ExecutionError,
    EofError,
    NotImplemented,
    UnknownDay(u32),
}

use AdventError::*;
impl From<ParseIntError> for AdventError {
    fn from(e: ParseIntError) -> Self {
        ParseError(e.to_string())
    }
}

use wasm_bindgen::JsValue;
impl From<AdventError> for JsValue {
    fn from(err: AdventError) -> Self {
        err.to_string().into()
    }
}

use std::{fmt, num::ParseIntError, string::ParseError};
impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use AdventError::*;
        match self {
            ParseError(e) => write!(f, "could not parse input, {}", e),
            ExecutionError => write!(f, "executing function"),
            EofError => write!(f, "could not derive solution from input"),
            NotImplemented => write!(f, "not implemented"),
            UnknownDay(d) => write!(f, "unknown day '{}'", d),
        }
    }
}

pub type AdventResult<T> = Result<T, AdventError>;

pub trait Puzzle {
    fn first(&self) -> AdventResult<i32>;
    fn second(&self) -> AdventResult<i32>;

    fn all(&self) -> AdventResult<(i32, i32)> {
        Ok((self.first()?, self.second()?))
    }
}
