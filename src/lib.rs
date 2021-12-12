pub mod advent;
pub mod solutions;

extern crate web_sys;
use advent::Puzzle;
use solutions::Solution;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use advent::*;

#[wasm_bindgen]
pub fn options() -> Vec<JsValue> {
    solutions::Solution::puzzles()
        .iter()
        .map(|&s| s.into())
        .collect()
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct JsPuzzle {
    puzzle: Solution,
}

impl JsPuzzle {
    pub fn new(puzzle: Solution) -> Self {
        JsPuzzle { puzzle }
    }
}

#[wasm_bindgen]
impl JsPuzzle {
    pub fn first(&self) -> Result<JsValue, JsValue> {
        match self.puzzle.first() {
            Ok(v) => Ok(v.to_string().into()),
            Err(e) => Err(e.to_string().into()),
        }
    }

    pub fn second(&self) -> Result<JsValue, JsValue> {
        match self.puzzle.second() {
            Ok(v) => Ok(v.to_string().into()),
            Err(e) => Err(e.to_string().into()),
        }
    }
}

#[wasm_bindgen]
pub fn puzzle(day: u32, input: &str) -> JsValue {
    match Solution::parse(day, input) {
        Ok(x) => {
            let p = JsPuzzle::new(x);
            p.into()
        }
        Err(e) => e.into(),
    }
}
