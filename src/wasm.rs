use crate::ai::Expectimax;
use crate::ai::AI;
use crate::engine as GameEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInterface(Expectimax);

#[wasm_bindgen]
impl WasmInterface {
    pub fn new() -> Self {
        WasmInterface(Expectimax::new())
    }

    pub fn make_move(&mut self) {
        let best_move = self.0.get_next_move();
        match best_move {
            Some(direction) => {
                self.0 = self
                    .0
                    .update_board(GameEngine::make_move(self.0.get_board(), direction));
            }
            None => (),
        }
    }

    pub fn get_tile_val(&self, row_idx: usize, col_idx: usize) -> u16 {
        let idx = (row_idx * 4) + col_idx;
        GameEngine::get_tile_val(self.0.get_board(), idx)
    }

    pub fn is_game_over(&self) -> bool {
        GameEngine::is_game_over(self.0.get_board())
    }
}
