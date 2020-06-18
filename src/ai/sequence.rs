use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;

pub struct Sequence {
    move_sequence: Vec<Move>,
    sequence_idx: usize,
    starting_board: GameEngine::Board,
}

impl Sequence {
    pub fn new(move_sequence: Vec<Move>) -> Self {
        Sequence {
            move_sequence,
            sequence_idx: 0,
            starting_board: 0,
        }
    }
}

impl AI for Sequence {
    fn get_next_move(&mut self, board: GameEngine::Board) -> Option<Move> {
        if GameEngine::is_game_over(board) {
            return None;
        }

        // terminate if full sequence complete with no change to board
        if self.sequence_idx == 0 {
            if self.starting_board == board {
                return None;
            }
            self.starting_board = board;
        }

        let next_move = self.move_sequence[self.sequence_idx];
        self.sequence_idx += 1;
        if self.sequence_idx >= self.move_sequence.len() {
            self.sequence_idx = 0;
        }
        Some(next_move)
    }
}
