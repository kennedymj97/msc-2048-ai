use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;
use rand::Rng;

pub struct Random;

impl AI for Random {
    fn get_next_move(&self, board: GameEngine::Board) -> Option<Move> {
        if GameEngine::is_game_over(board) {
            return None;
        }
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0, 4);
        match num {
            0 => return Some(Move::Left),
            1 => return Some(Move::Right),
            2 => return Some(Move::Up),
            3 => return Some(Move::Down),
            _ => panic!("The number generated to select the move was invalid"),
        }
    }
}
