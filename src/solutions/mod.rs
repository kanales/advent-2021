mod binary_diagnostic;
mod dive;
mod giant_squid;
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
pub const PUZZLES: &[&str] = &["Sonar Sweep", "Dive!", "Binary Diagnostic", "Giant Squid"];

pub fn factory(day: u32, input: &str) -> AdventResult<Box<dyn Puzzle>> {
    macro_rules! box_puzzle {
        { $($p:pat => $t:path,)* } => {
            match day {
                $($p => Ok(Box::new(<$t>::try_from(input)?)),)*
                _ => return Err(AdventError::UnknownDay(day)),
            }
        }
    }

    box_puzzle! {
        1 => sonar_sweep::SonarSweep,
        2 => dive::Dive,
        3 => binary_diagnostic::BinaryDiagnostic,
        4 => giant_squid::GiantSquid,
    }
}
