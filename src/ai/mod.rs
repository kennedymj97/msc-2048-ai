use crate::engine as GameEngine;
use crate::engine::Move;
use std::time::SystemTime;

mod expectimax;
mod random;

pub use self::expectimax::Expectimax;
pub use self::random::Random;

pub trait AI: Clone + Copy {
    fn new() -> Self;

    fn restart(self) -> Self;

    fn get_board(self) -> GameEngine::Board;

    fn get_next_move(self) -> Option<Move>;

    fn update_board(self, new_board: GameEngine::Board) -> Self;
}

pub fn evaluate(ai: impl AI, num_iters: u64) {
    let average_score = (0..num_iters).fold(0, |acc, _| {
        let ai = ai.restart();
        let score = run(ai);
        acc + (score / num_iters)
    });
    println!("Average score: {}", average_score);
}

fn run(mut ai: impl AI) -> u64 {
    let mut num_moves = 0;
    let start_time = SystemTime::now();
    loop {
        let best_move = ai.get_next_move();
        match best_move {
            Some(direction) => {
                ai = ai.update_board(GameEngine::make_move(ai.get_board(), direction));
            }
            None => break,
        }
        println!("Score: {}", GameEngine::get_score(ai.get_board()));
        println!("{}", GameEngine::to_str(ai.get_board()));
        num_moves += 1;
    }
    let time_elapsed = match start_time.elapsed() {
        Ok(elapsed) => elapsed.as_nanos(),
        Err(e) => panic!(e),
    };
    println!("Total number of moves made: {}", num_moves);
    println!("Total time taken: {}s", time_elapsed / 1000000000);
    println!(
        "Average move time for run was: {}ns, {}us, {}ms",
        time_elapsed / num_moves,
        time_elapsed / (num_moves * 1000),
        time_elapsed / (num_moves * 1000000)
    );
    println!("Final board: {}", GameEngine::to_str(ai.get_board()));
    GameEngine::get_score(ai.get_board())
}
