use super::generate_strategies::generate_strategies;
use super::generate_strategies::Variations;
use super::rules::Rules;
use crate::ai::AI;
use crate::engine as GameEngine;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn run_strategies_save_results<F>(set: &[Variations], create_ai: F, n: u32, filename: &str)
where
    F: Fn(Rules) -> Box<dyn AI>,
{
    GameEngine::create_stores();
    let strategies = generate_strategies(set);
    assert!(strategies.len() == 65);
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    strategies.iter().for_each(|strategy| {
        println!("Evaluating strategy: {:?}", strategy);
        f.write_fmt(format_args!("{:?}->", strategy))
            .expect("Failed to write strategy information to file");
        let results = run_strategy(create_ai(strategy.clone()), n);
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

fn run_strategy(mut ai: Box<dyn AI>, n: u32) -> Vec<u64> {
    (0..n).fold(vec![], |mut results, _| {
        let mut board = GameEngine::new_board();
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

pub fn run_strategy_save_results(mut ai: Box<dyn AI>, n: u32, filename: &str) {
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    (0..n).for_each(|_| {
        let mut board = GameEngine::new_board();
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
