use crate::engine as GameEngine;
use crate::engine::Move;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

pub mod expectimax;
pub mod random;
pub mod sequence;
pub mod snake;

pub trait AI {
    fn get_next_move(&mut self, board: GameEngine::Board) -> Option<Move>;
}

pub fn run_ai(mut ai: Box<dyn AI>) {
    let mut num_moves = 0;
    let start_time = SystemTime::now();
    let mut board = GameEngine::new_game();
    loop {
        //println!("Score: {}", GameEngine::get_score(board));
        //println!("{}", GameEngine::to_str(board));
        let best_move = ai.get_next_move(board);
        match best_move {
            Some(direction) => {
                board = GameEngine::make_move(board, direction);
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

pub fn record_ai_game(mut ai: Box<dyn AI>, filename: &str) {
    let mut file = File::create(format!("./{}.txt", filename)).expect("failed to create file");
    let mut board = GameEngine::new_game();
    loop {
        if GameEngine::get_score(board) > 25000 {
            break;
        }
        println!("Score: {}", GameEngine::get_score(board));
        println!("{}", GameEngine::to_str(board));
        let best_move = ai.get_next_move(board);
        match best_move {
            Some(direction) => {
                file.write_fmt(format_args!(
                    "Board: {}Move: {}\n\n",
                    GameEngine::to_str(board),
                    direction
                ))
                .expect("failed to write to file");
                board = GameEngine::make_move(board, direction);
            }
            None => break,
        }
    }
}

use permutohedron::Heap;

pub fn generate_strategies<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
    let power_set = set.iter().fold(vec![vec![]], |mut power_set, set_item| {
        let i = power_set.clone().into_iter().map(|mut sub_set| {
            sub_set.push(set_item.clone());
            sub_set
        });
        power_set.extend(i);
        power_set
    });

    let mut all_strategies = Vec::new();
    for mut set in power_set {
        let heap = Heap::new(&mut set);
        for data in heap {
            all_strategies.push(data.clone());
        }
    }
    all_strategies
}

pub fn evaluate_strategies<T, F>(set: &[T], create_ai: F, n: u32, filename: &str)
where
    T: fmt::Debug + Clone,
    F: Fn(Vec<T>) -> Box<dyn AI>,
{
    let strategies = generate_strategies(set);
    assert!(strategies.len() == 65);
    let mut f = File::create(format!("{}", filename)).expect("Failed to create file");
    strategies.iter().for_each(|strategy| {
        println!("Evaluating strategy: {:?}", strategy);
        f.write_fmt(format_args!("{:?}->", strategy))
            .expect("Failed to write strategy information to file");
        let results = evaluate_strategy(create_ai(strategy.clone()), n);
        results.iter().enumerate().for_each(|(idx, score)| {
            if idx == results.len() - 1 {
                f.write_fmt(format_args!("{}", score))
                    .expect("failed to write final score");
            } else {
                f.write_fmt(format_args!("{};", score))
                    .expect("Failed to write score");
            }
        });
        f.write("\n".as_bytes()).expect("Failed to write new line");
    });
}

fn evaluate_strategy(mut ai: Box<dyn AI>, n: u32) -> Vec<u64> {
    (0..n).fold(vec![], |mut results, _| {
        let mut board = GameEngine::new_game();
        loop {
            let best_move = ai.get_next_move(board);
            match best_move {
                Some(direction) => {
                    board = GameEngine::make_move(board, direction);
                }
                None => break,
            }
        }
        results.push(GameEngine::get_score(board));
        results
    })
}

fn average(items: &Vec<u64>) -> f32 {
    (items.iter().sum::<u64>() as f32) / (items.len() as f32)
}

pub fn get_strategy_data(mut ai: Box<dyn AI>, n: u32, filename: &str) {
    let mut f = File::create(format!("{}", filename)).expect("Failed to create file");
    (0..n).for_each(|_| {
        let mut board = GameEngine::new_game();
        loop {
            let best_move = ai.get_next_move(board);
            match best_move {
                Some(direction) => {
                    board = GameEngine::make_move(board, direction);
                }
                None => break,
            }
        }
        f.write_fmt(format_args!("{},", GameEngine::get_score(board)))
            .expect("failed to write data to file");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_median() {
        assert_eq!(median(&vec![5, 1, 1, 3, 4]), 3);
        assert_eq!(median(&vec![5, 1, 2, 4, 1, 2, 3, 5, 8, 7, 6, 5]), 5);
    }

    #[test]
    fn it_average() {
        assert_eq!(average(&vec![1, 2, 3, 4, 5]), 3.0);
    }
}
