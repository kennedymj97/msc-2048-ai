use crate::engine as GameEngine;
use crate::engine::Move;
use std::time::SystemTime;

pub mod expectimax;
pub mod random;
pub mod sequence;

pub trait AI {
    fn get_next_move(&mut self, board: GameEngine::Board) -> Option<Move>;
}

pub fn run_ai(mut ai: Box<dyn AI>) {
    let mut num_moves = 0;
    let start_time = SystemTime::now();
    let mut board = GameEngine::new_game();
    loop {
        let best_move = ai.get_next_move(board);
        match best_move {
            Some(direction) => {
                board = GameEngine::make_move(board, direction);
            }
            None => break,
        }
        println!("Score: {}", GameEngine::get_score(board));
        println!("{}", GameEngine::to_str(board));
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
    println!("Final board: {}", GameEngine::to_str(board));
}

pub fn run_ai_with_delay(mut ai: Box<dyn AI>, delay: u64) {
    let mut board = GameEngine::new_game();
    loop {
        let best_move = ai.get_next_move(board);
        match best_move {
            Some(direction) => {
                board = GameEngine::make_move(board, direction);
            }
            None => break,
        }
        println!("Score: {}", GameEngine::get_score(board));
        println!("{}", GameEngine::to_str(board));
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    println!("Final board: {}", GameEngine::to_str(board));
}
