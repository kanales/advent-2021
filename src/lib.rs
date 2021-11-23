pub mod advent;
pub mod solutions;

use advent::AdventError;
use std::convert::{TryFrom, TryInto};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use advent::*;

#[wasm_bindgen]
pub fn report_repair(input: &str) -> Option<i32> {
    let rr: solutions::ReportRepair = input.try_into().ok()?;

    rr.first().ok()
}

macro_rules! solutions {
    ($day:ident,$input:expr => { $($ex:pat => $id:path),+ $(,)?} ) => {
        Box::new(match $day {
            $($ex => <$id>::try_from($input),)+
            _ => Err(advent::AdventError::UnknownDay($day))
        }?)
    };
}

#[wasm_bindgen]
pub struct Output {
    pub first: i32,
    pub second: i32,
}

fn run_helper(input: &str, day: i32) -> Result<Output, AdventError> {
    let b = solutions!(day, input => {
        0 => solutions::ReportRepair,
    });

    let first = b.first()?;
    let second = b.second()?;
    Ok(Output { first, second })
}

#[wasm_bindgen]
pub fn run(input: &str, day: i32) -> Result<Output, JsValue> {
    match run_helper(input, day) {
        Ok(r) => Ok(r),
        Err(_) => Err("Execution Error".into()),
    }
}
