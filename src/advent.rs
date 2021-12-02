use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventError {
    ParseError(String),
    ExecutionError,
    EofError,
    UnknownDay(u32),
}

use std::fmt;
impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use AdventError::*;
        match self {
            ParseError(e) => write!(f, "could not parse input, {}", e),
            ExecutionError => write!(f, "executing function"),
            EofError => write!(f, "could not derive solution from input"),
            UnknownDay(d) => write!(f, "unknown day '{}'", d),
        }
    }
}

pub type AdventResult<T> = Result<T, AdventError>;

pub trait Puzzle<'a>: TryFrom<&'a str> {
    fn first(&self) -> AdventResult<i32>;
    fn second(&self) -> AdventResult<i32>;

    fn all(&self) -> AdventResult<(i32, i32)> {
        Ok((self.first()?, self.second()?))
    }
}
