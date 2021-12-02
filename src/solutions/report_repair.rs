// Day 0: Example
use crate::advent::{AdventError, AdventResult, Puzzle};
use std::convert::{TryFrom, TryInto};
pub struct ReportRepair(Vec<u32>);

impl TryFrom<&str> for ReportRepair {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numbers: Result<Vec<u32>, _> = value.lines().map(|x| x.parse::<u32>()).collect();
        match numbers {
            Ok(v) => Ok(ReportRepair(v)),
            Err(e) => Err(AdventError::ParseError(e.to_string())),
        }
    }
}

impl<'a> Puzzle<'a> for ReportRepair {
    fn first(&self) -> AdventResult<i32> {
        for x in self.0.iter() {
            for y in self.0.iter() {
                if x + y == 2020 {
                    return (x * y).try_into().map_err(|_| AdventError::ExecutionError);
                }
            }
        }
        Err(AdventError::EofError)
    }
    fn second(&self) -> AdventResult<i32> {
        Ok(0)
    }
}
