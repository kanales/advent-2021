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
        let out = (self.first()?, self.second()?);
        Ok(out)
    }
}

use std::convert::TryFrom;

pub const PUZZLES: &'static [&'static str] = &["SonarSweep", "Dive!"];

pub fn run_puzzle(input: &str, day: u32) -> AdventResult<(i32, i32)> {
    use crate::solutions::*;
    match day {
        1 => SonarSweep::try_from(input)?.all(),
        2 => Dive::try_from(input)?.all(),
        _ => Err(AdventError::UnknownDay(day)),
    }
}
