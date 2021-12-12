use std::str::FromStr;

// Day 0: Example
use crate::advent::{AdventError, AdventResult, Puzzle};
pub struct ReportRepair(Vec<u32>);

impl FromStr for ReportRepair {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let numbers = value.lines().map(u32::from_str).collect::<Result<_, _>>()?;
        Ok(ReportRepair(numbers))
    }
}

impl Puzzle for ReportRepair {
    fn first(&self) -> AdventResult<i64> {
        for x in self.0.iter() {
            for y in self.0.iter() {
                if x + y == 2020 {
                    return Ok((x * y) as i64);
                }
            }
        }
        Err(AdventError::EofError)
    }
    fn second(&self) -> AdventResult<i64> {
        Ok(0)
    }
}
