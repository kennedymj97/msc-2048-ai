use crate::engine;
use crate::engine::{Board, GameEngine, GameEngineStores, Move};
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

pub mod default;
pub mod expectimax;
pub mod expectimax_old;
pub mod random;
pub mod sequence;
pub mod strategy;

pub trait AI {
    fn get_next_move<T: GameEngine>(&mut self, engine: &T, board: Board) -> Option<Move>;
}

pub trait AII {
    fn get_next_move(&self, board: Board) -> Option<crate::engine_unsafe::Move>;
}

pub fn run_ai<T: AI>(ai: &mut T) {
    let mut num_moves = 0;
    let start_time = SystemTime::now();
    let engine = GameEngineStores::new();
    let mut board = engine::new_board();
    loop {
        println!("Score: {}", engine.get_score(board));
        println!("{}", engine::to_str(board));
        let best_move = ai.get_next_move(&engine, board);
        match best_move {
            Some(direction) => {
                board = engine.make_move(board, direction);
            }
            None => break,
        }
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
    println!("Final board: {}", engine::to_str(board));
}

pub fn run_ai_with_delay<T: AI>(ai: &mut T, delay: u64) {
    let engine = GameEngineStores::new();
    let mut board = engine::new_board();
    loop {
        let best_move = ai.get_next_move(&engine, board);
        match best_move {
            Some(direction) => {
                board = engine.make_move(board, direction);
            }
            None => break,
        }
        println!("Score: {}", engine.get_score(board));
        println!("{}", engine::to_str(board));
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    println!("Final board: {}", engine::to_str(board));
}

pub fn record_ai_game<T: AI>(ai: &mut T, filename: &str) {
    let mut file = File::create(format!("./{}.txt", filename)).expect("failed to create file");
    let engine = GameEngineStores::new();
    let mut board = engine::new_board();
    loop {
        println!("Score: {}", engine.get_score(board));
        println!("{}", engine::to_str(board));
        let best_move = ai.get_next_move(&engine, board);
        match best_move {
            Some(direction) => {
                file.write_fmt(format_args!(
                    "Board: {}Move: {}\n\n",
                    engine::to_str(board),
                    direction
                ))
                .expect("failed to write to file");
                board = engine.make_move(board, direction);
            }
            None => break,
        }
    }
    println!(
        "\nFinal state:\nScore: {}\n{}",
        engine.get_score(board),
        engine::to_str(board)
    );
}
