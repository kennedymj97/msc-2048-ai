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
    let average_score = (0..num_iters).fold(0., |acc, _| {
        let score = run(ai);
        acc + (score / num_iters as f64)
    });
    println!("Average score: {}", average_score);
    println!("Final board: {}", GameEngine::to_str(ai.get_board()));
}

pub fn run(ai: impl AI) -> f64 {
    let mut ai = ai.restart();
    let mut num_moves = 0;
    let start_time = SystemTime::now();
    loop {
        let best_move = ai.get_next_move();
        match best_move {
            Some(Move::Left) => ai = ai.update_board(GameEngine::move_left(ai.get_board())),
            Some(Move::Right) => ai = ai.update_board(GameEngine::move_right(ai.get_board())),
            Some(Move::Up) => ai = ai.update_board(GameEngine::move_up(ai.get_board())),
            Some(Move::Down) => ai = ai.update_board(GameEngine::move_down(ai.get_board())),
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
    GameEngine::get_score(ai.get_board())
}

pub fn debug(ai: impl AI) {
    let mut ai = ai;
    loop {
        println!("{}", GameEngine::to_str(ai.get_board()));
        let best_move = ai.get_next_move();
        match best_move {
            Some(Move::Left) => ai = ai.update_board(GameEngine::move_left(ai.get_board())),
            Some(Move::Right) => ai = ai.update_board(GameEngine::move_right(ai.get_board())),
            Some(Move::Up) => ai = ai.update_board(GameEngine::move_up(ai.get_board())),
            Some(Move::Down) => ai = ai.update_board(GameEngine::move_down(ai.get_board())),
            None => break,
        }
        if GameEngine::is_game_over(ai.get_board()) {
            println!("{}", GameEngine::to_str(ai.get_board()));
            break;
        }

        //std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
