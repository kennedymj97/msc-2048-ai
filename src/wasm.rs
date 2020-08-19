use crate::ai::expectimax_old::Expectimaxx;
use crate::ai::AII;
use crate::engine_unsafe as GameEngine;
use crate::engine_unsafe::Board;
use crate::engine_unsafe::Move;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInterface;

#[wasm_bindgen]
impl WasmInterface {
    pub fn new() -> Self {
        GameEngine::new();
        Expectimaxx::new();
        WasmInterface
    }
    #[wasm_bindgen]
    pub fn get_next_move(&self, board: Board) -> i32 {
        match Expectimaxx.get_next_move(board) {
            Some(direction) => match direction {
                Move::Up => 0,
                Move::Right => 1,
                Move::Down => 2,
                Move::Left => 3,
            },
            None => -1,
        }
    }
}

#[wasm_bindgen]
pub fn wasm_test_fn() -> String {
    String::from("This function is being called from wasm")
}
