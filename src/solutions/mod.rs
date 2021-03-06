mod binary_diagnostic;
mod dive;
mod giant_squid;
mod hydrotermal_adventure;
mod lanternfish;
mod report_repair;
mod sonar_sweep;
mod whale_treachery;

use crate::advent::{AdventError, AdventResult, Puzzle};

#[macro_export]
macro_rules! parse_error {
    ($($arg:tt)*) => {
        AdventError::ParseError(format!($($arg)*))
    };
}

#[derive(Debug)]
pub enum Solution {
    Day1(sonar_sweep::SonarSweep),
    Day2(dive::Dive),
    Day3(binary_diagnostic::BinaryDiagnostic),
    Day4(giant_squid::GiantSquid),
    Day5(hydrotermal_adventure::HydrothermalVenture),
    Day6(lanternfish::Lanternfish),
    Day7(whale_treachery::CrabSwarm),
}

impl Solution {
    pub fn puzzles() -> &'static [&'static str] {
        &[
            "Sonar Sweep",
            "Dive!",
            "Binary Diagnostic",
            "Giant Squid",
            "Hydrotermal Venture",
            "Lanternfish",
            "The Treachery of Whales",
        ]
    }

    fn exec<F, B>(&self, f: F) -> B
    where
        F: FnOnce(&dyn Puzzle) -> B,
    {
        match self {
            Self::Day1(p) => f(p),
            Self::Day2(p) => f(p),
            Self::Day3(p) => f(p),
            Self::Day4(p) => f(p),
            Self::Day5(p) => f(p),
            Self::Day6(p) => f(p),
            Self::Day7(p) => f(p),
        }
    }

    pub fn parse(day: u32, input: &str) -> AdventResult<Self> {
        use Solution::*;
        macro_rules! puzzle {
            { $($p:pat => $i:ident ($t:path) ,)* } => {
                match day {
                    $($p => Ok($i(input.parse::<$t>()?)),)*
                    _ => return Err(AdventError::UnknownDay(day)),
                }
            }
        }

        puzzle! {
            1 => Day1(sonar_sweep::SonarSweep),
            2 => Day2(dive::Dive),
            3 => Day3(binary_diagnostic::BinaryDiagnostic),
            4 => Day4(giant_squid::GiantSquid),
            5 => Day5(hydrotermal_adventure::HydrothermalVenture),
            6 => Day6(lanternfish::Lanternfish),
            7 => Day7(whale_treachery::CrabSwarm),
        }
    }
}

impl Puzzle for Solution {
    fn first(&self) -> AdventResult<i64> {
        self.exec(|p| p.first())
    }

    fn second(&self) -> AdventResult<i64> {
        self.exec(|p| p.second())
    }
}
