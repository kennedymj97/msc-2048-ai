use crate::engine::GameEngine;
use crate::engine::Move;
use std::time::SystemTime;

mod expectimax;
mod random;

pub use self::expectimax::Expectimax;
pub use self::random::Random;

pub trait AI {
    fn new() -> Self;

    fn restart(&mut self);

    fn get_engine(&mut self) -> &GameEngine;

    fn get_mut_engine(&mut self) -> &mut GameEngine;

    fn get_next_move(&mut self) -> Option<Move>;

    fn evaluate(&mut self, num_iters: u64) {
        let average_score = (0..num_iters).fold(0, |acc, _| {
            let score = self.run();
            acc + (score / num_iters)
        });
        println!("Average score: {}", average_score);
    }

    fn run(&mut self) -> u64 {
        self.restart();
        let mut num_moves = 0;
        let start_time = SystemTime::now();
        loop {
            let best_move = self.get_next_move();
            match best_move {
                Some(Move::Left) => self.get_mut_engine().move_left(),
                Some(Move::Right) => self.get_mut_engine().move_right(),
                Some(Move::Up) => self.get_mut_engine().move_up(),
                Some(Move::Down) => self.get_mut_engine().move_down(),
                None => break,
            }
            num_moves += 1;
        }
        let time_elapsed = match start_time.elapsed() {
            Ok(elapsed) => elapsed.as_nanos(),
            Err(e) => panic!(e),
        };
        println!(
            "Average move time for run was: {}ns, {}us, {}ms",
            time_elapsed / num_moves,
            time_elapsed / (num_moves * 1000),
            time_elapsed / (num_moves * 1000000)
        );
        self.get_engine().get_score()
    }

    fn debug(&mut self) {
        loop {
            println!("{}", self.get_engine());
            let best_move = self.get_next_move();
            match best_move {
                Some(Move::Left) => self.get_mut_engine().move_left(),
                Some(Move::Right) => self.get_mut_engine().move_right(),
                Some(Move::Up) => self.get_mut_engine().move_up(),
                Some(Move::Down) => self.get_mut_engine().move_down(),
                None => break,
            }

            //std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
