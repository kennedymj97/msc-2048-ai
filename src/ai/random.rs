use crate::ai::AI;
use crate::engine::Basic;
use crate::engine::GameEngine;
use crate::engine::Move;
use rand::Rng;

pub struct BasicRandom(Basic);

impl AI for BasicRandom {
    type Engine = Basic;

    fn new() -> Self {
        BasicRandom(Basic::new())
    }

    fn restart(&mut self) {
        self.0 = Basic::new();
    }

    fn get_engine(&mut self) -> &Self::Engine {
        &self.0
    }

    fn get_mut_engine(&mut self) -> &mut Self::Engine {
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
