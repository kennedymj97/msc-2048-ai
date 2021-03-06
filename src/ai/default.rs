use crate::ai::AI;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;

pub struct Default;

impl Default {
    pub fn new() -> Self {
        Default
    }
}

impl AI for Default {
    fn get_next_move<T: GameEngine>(&mut self, engine: &T, board: Board) -> Option<Move> {
        let new_board = engine.make_move(board, Move::Left);
        if new_board != board {
            return Some(Move::Left);
        }
        let new_board = engine.make_move(board, Move::Down);
        if new_board != board {
            return Some(Move::Down);
        }
        let new_board = engine.make_move(board, Move::Up);
        if new_board != board {
            return Some(Move::Up);
        }
        let new_board = engine.make_move(board, Move::Right);
        if new_board != board {
            return Some(Move::Right);
        }
        None
    }
}
