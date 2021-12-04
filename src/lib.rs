pub mod advent;
pub mod solutions;

use advent::Puzzle;
use solutions::factory;
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
pub struct PuzzleJs;

impl PuzzleJs {
    pub fn first() -> Result<i32, JsValue> {
        todo!()
    }

    pub fn second() -> Result<i32, JsValue> {
        todo!()
    }
}

#[wasm_bindgen]
pub fn options() -> Vec<JsValue> {
    solutions::PUZZLES.iter().map(|&s| s.into()).collect()
}

#[wasm_bindgen]
pub struct JsPuzzle {
    puzzle: Box<dyn Puzzle>,
}

impl JsPuzzle {
    pub fn new(puzzle: Box<dyn Puzzle>) -> Self {
        JsPuzzle { puzzle }
    }
}

#[wasm_bindgen]
impl JsPuzzle {
    pub fn first(&self) -> Result<i32, JsValue> {
        match self.puzzle.first() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string().into()),
        }
    }

    pub fn second(&self) -> Result<i32, JsValue> {
        match self.puzzle.second() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string().into()),
        }
    }
}

#[wasm_bindgen]
pub fn puzzle(day: u32, input: &str) -> JsValue {
    match factory(day, input) {
        Ok(x) => JsPuzzle::new(x).into(),
        Err(e) => e.into(),
    }
}
