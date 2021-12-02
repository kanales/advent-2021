pub mod advent;
pub mod solutions;

use advent::AdventError;
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
pub struct Output {
    pub first: i32,
    pub second: i32,
}
#[wasm_bindgen]
pub fn options() -> Vec<JsValue> {
    solutions::PUZZLES
        .iter()
        .enumerate()
        .map(|(idx, &s)| format!("{}: {}", idx, s).into())
        .collect()
}

#[wasm_bindgen]
pub fn run(input: &str, day: u32) -> Result<Output, JsValue> {
    match solutions::run_puzzle(input, day) {
        Ok((a, b)) => Ok(Output {
            first: a,
            second: b,
        }),
        Err(e) => Err(format!("{}", e).into()),
    }
}
