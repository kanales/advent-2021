mod binary_diagnostic;
mod dive;
mod report_repair;
mod sonar_sweep;

use crate::advent::{AdventError, AdventResult, Puzzle};
use std::convert::TryFrom;

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        AdventError::ParseError(format!($($arg)*))
    };
}

// Puzzle solutions in order
pub const PUZZLES: &[&str] = &["Sonar Sweep", "Dive!", "Binary Diagnostic"];

pub fn run_puzzle(input: &str, day: u32) -> AdventResult<(i32, i32)> {
    match day {
        1 => sonar_sweep::SonarSweep::try_from(input)?.all(),
        2 => dive::Dive::try_from(input)?.all(),
        3 => binary_diagnostic::BinaryDiagnostic::try_from(input)?.all(),
        _ => Err(AdventError::UnknownDay(day)),
    }
}
