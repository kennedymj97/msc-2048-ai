use crate::engine::GameEngine;
use crate::engine::Move;

mod expectimax;
mod random;

pub use self::expectimax::BasicExpectimax;
pub use self::expectimax::OptimisedExpectimax;
pub use self::random::BasicRandom;

pub trait AI {
    type Engine: GameEngine;

    fn new() -> Self;

    fn restart(&mut self);

    fn get_engine(&mut self) -> &Self::Engine;

    fn get_mut_engine(&mut self) -> &mut Self::Engine;

    fn get_next_move(&mut self) -> Move;

    fn evaluate(&mut self, num_iters: u64) {
        let average_score = (0..num_iters).fold(0, |acc, _| {
            let score = self.run();
            acc + (score / num_iters)
        });
        println!("Average score: {}", average_score);
    }

    fn run(&mut self) -> u64 {
        self.restart();
        loop {
            let best_move = self.get_next_move();
            match best_move {
                Move::Left => self.get_mut_engine().move_left(),
                Move::Right => self.get_mut_engine().move_right(),
                Move::Up => self.get_mut_engine().move_up(),
                Move::Down => self.get_mut_engine().move_down(),
            }
            if self.get_mut_engine().is_game_over() {
                return self.get_engine().get_score();
            }
        }
    }

    fn debug(&mut self) {
        loop {
            let best_move = self.get_next_move();
            match best_move {
                Move::Left => self.get_mut_engine().move_left(),
                Move::Right => self.get_mut_engine().move_right(),
                Move::Up => self.get_mut_engine().move_up(),
                Move::Down => self.get_mut_engine().move_down(),
            }
            println!("{}", self.get_engine());
            if self.get_mut_engine().is_game_over() {
                break;
            }
            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
