use crate::ai::expectimax::Expectimax;
use crate::ai::AI;
use crate::engine as GameEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInterface(GameEngine::Board);

#[wasm_bindgen]
impl WasmInterface {
    pub fn new() -> Self {
        Expectimax::new();
        WasmInterface(GameEngine::new_game())
    }

    pub fn make_move(&mut self) {
        let best_move = Expectimax.get_next_move(self.0);
        match best_move {
            Some(direction) => {
                self.0 = GameEngine::make_move(self.0, direction);
            }
            None => (),
        }
    }

    pub fn get_tile_val(&self, row_idx: usize, col_idx: usize) -> u16 {
        let idx = (row_idx * 4) + col_idx;
        GameEngine::get_tile_val(self.0, idx)
    }

    pub fn is_game_over(&self) -> bool {
        GameEngine::is_game_over(self.0)
    }
}
