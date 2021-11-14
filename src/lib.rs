use std::convert::{TryFrom, TryInto};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum AdventError {
    ParseError,
    ExecutionError,
}

type AdventResult = Result<i32, AdventError>;

trait Puzzle {
    fn part1(&self) -> AdventResult;
    fn part2(&self) -> AdventResult;
}

struct ReportRepair(Vec<u32>);

impl TryFrom<&str> for ReportRepair {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numbers: Result<Vec<u32>, _> = value.lines().map(|x| x.parse::<u32>()).collect();
        match numbers {
            Ok(v) => Ok(ReportRepair(v)),
            Err(_) => Err(AdventError::ParseError),
        }
    }
}

impl Puzzle for ReportRepair {
    fn part1(&self) -> AdventResult {
        for x in self.0.iter() {
            for y in self.0.iter() {
                if x + y == 2020 {
                    return match (x * y).try_into() {
                        Ok(x) => Ok(x),
                        Err(_) => Err(AdventError::ExecutionError),
                    };
                }
            }
        }
        Err(AdventError::ExecutionError)
    }
    fn part2(&self) -> AdventResult {
        unimplemented!()
    }
}

#[wasm_bindgen]
pub fn report_repair(input: &str) -> Option<i32> {
    let rr: ReportRepair = input.try_into().ok()?;

    rr.part1().ok()
}

#[wasm_bindgen]
pub fn run(input: &str, day: i32) -> Option<i32> {
    let b = Box::new(match day {
        0 => ReportRepair::try_from(input).ok()?,
        _ => None?,
    });

    b.part1().ok()
}
