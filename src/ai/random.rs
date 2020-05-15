use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Random(GameEngine::Board);

impl AI for Random {
    fn new() -> Self {
        Random(GameEngine::new_game())
    }

    fn restart(self) -> Self {
        Random(GameEngine::start_new_game())
    }

    fn get_board(self) -> GameEngine::Board {
        self.0
    }

    fn update_board(self, new_board: GameEngine::Board) -> Self {
        Random(new_board)
    }

    fn get_next_move(self) -> Option<Move> {
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
