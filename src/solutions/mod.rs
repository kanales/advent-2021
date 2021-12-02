mod dive;
mod report_repair;
mod sonar_sweep;

use crate::advent::{AdventError, AdventResult, Puzzle};
use std::convert::TryFrom;

// Puzzle solutions in order
pub const PUZZLES: &[&str] = &["SonarSweep", "Dive!"];

pub fn run_puzzle(input: &str, day: u32) -> AdventResult<(i32, i32)> {
    match day {
        1 => sonar_sweep::SonarSweep::try_from(input)?.all(),
        2 => dive::Dive::try_from(input)?.all(),
        _ => Err(AdventError::UnknownDay(day)),
    }
}
