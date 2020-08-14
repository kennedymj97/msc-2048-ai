use crate::ai::expectimax::Expectimax;
use crate::ai::AI;
use crate::engine::Board;
use crate::engine::GameEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInterface(GameEngine, Board);

#[wasm_bindgen]
impl WasmInterface {
    pub fn new() -> Self {
        Expectimax::new();
        WasmInterface(GameEngine::new(), GameEngine::new_board())
    }

    pub fn make_move(&mut self) {
        let best_move = Expectimax.get_next_move(&self.0, self.1);
        match best_move {
            Some(direction) => {
                self.1 = self.0.make_move(self.1, direction);
            }
            None => (),
        }
    }

    pub fn get_tile_val(&self, row_idx: usize, col_idx: usize) -> u16 {
        let idx = (row_idx * 4) + col_idx;
        GameEngine::get_tile_val(self.1, idx)
    }

    pub fn is_game_over(&self) -> bool {
        self.0.is_game_over(self.1)
    }
}

#[wasm_bindgen]
pub fn wasm_test_fn() -> String {
    String::from("This function is being called from wasm")
}
