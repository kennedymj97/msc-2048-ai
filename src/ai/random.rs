use crate::ai::AI;
use crate::engine::GameEngine;
use crate::engine::Move;
use rand::Rng;

pub struct Random(GameEngine);

impl AI for Random {
    fn new() -> Self {
        Random(GameEngine::new())
    }

    fn restart(&mut self) {
        self.0 = GameEngine::new();
    }

    fn get_engine(&mut self) -> &GameEngine {
        &self.0
    }

    fn get_mut_engine(&mut self) -> &mut GameEngine {
        &mut self.0
    }

    fn get_next_move(&mut self) -> Option<Move> {
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
