pub enum AdventError {
    ParseError,
    ExecutionError,
    EofError,
    UnknownDay(i32),
}

use std::fmt;
impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use AdventError::*;
        match self {
            ParseError => write!(f, "could not parse input"),
            ExecutionError => write!(f, "executing function"),
            EofError => write!(f, "could not derive solution from input"),
            UnknownDay(d) => write!(f, "unknown day '{}'", d),
        }
    }
}

pub type AdventResult = Result<i32, AdventError>;

pub trait Puzzle {
    fn first(&self) -> AdventResult;
    fn second(&self) -> AdventResult;
}
