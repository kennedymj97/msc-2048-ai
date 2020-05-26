use wasm_bindgen::prelude::*;

pub mod ai;
pub mod engine;
//pub mod ui;

// https://dev.to/robertorojasr/rust-project-structure-example-step-by-step-3ee
// ^^ good summary of how the module system works in rust.

#[wasm_bindgen]
pub fn mult10(num: i32) -> i32 {
    num * 10
}
