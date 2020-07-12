use super::evaluate_strategies::compare_strategies;
use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::generate_strategies;
use super::generate_strategies::Variations;
use super::rules::strategy_to_str;
use super::rules::Strategy;
use crate::ai::AI;
use crate::engine as GameEngine;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn progressive_brute_force<F>(
    set: &[Variations],
    create_ai: F,
    max_length: usize,
    foldername: &str,
) where
    F: Fn(Strategy) -> Box<dyn AI>,
{
    GameEngine::create_stores();
    let path = Path::new(foldername);
    let dir_builder = DirBuilder::new();
    dir_builder.create(path).expect("Failed to create folder");
    let strategies = generate_strategies(set, max_length);
    let strategies = strategies
        .into_iter()
        .map(|strategy| (strategy, Vec::new()))
        .collect::<StrategyDataStore<Strategy>>();
    let best_strategies = progressive_brute_force_aux(strategies, create_ai, 10, path);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(strategy, median)| println!("{}: {}", strategy_to_str(strategy), median));
}

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

fn progressive_brute_force_aux<F>(
    strategies_data: StrategyDataStore<Strategy>,
    create_ai: F,
    runs: usize,
    foldername: &Path,
) -> StrategyDataStore<Strategy>
where
    F: Fn(Strategy) -> Box<dyn AI>,
{
    if runs > 1000 {
        return strategies_data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = strategies_data.len();
    let strategies_data = strategies_data
        .iter()
        .map(|(strategy, results)| {
            count += 1;
            println!("{}/{}", count, total_count);
            (
                strategy.clone(),
                run_strategy(create_ai(strategy.clone()), results.clone(), runs),
            )
        })
        .collect::<StrategyDataStore<Strategy>>();
    save_results(&strategies_data, foldername, runs);
    progressive_brute_force_aux(
        compare_strategies(strategies_data),
        create_ai,
        runs * 10,
        foldername,
    )
}

fn save_results(strategies_data: &StrategyDataStore<Strategy>, foldername: &Path, runs: usize) {
    println!("Saving data @ {} runs...", runs);
    let path = foldername.join(format!("{}_runs.csv", runs));
    let mut f = File::create(path).expect("Failed to create file");
    strategies_data.iter().for_each(|(strategy, results)| {
        f.write_fmt(format_args!("{},", strategy_to_str(strategy)))
            .expect("Failed to write strategy information to file");
        let mut results_iter = results.iter().peekable();
        while let Some(score) = results_iter.next() {
            f.write_fmt(format_args!("{}", score))
                .expect("Failed to write score");
            if results_iter.peek().is_some() {
                f.write(",".as_bytes()).expect("Failed to write comma");
            }
        }
        f.write("\n".as_bytes()).expect("Failed to write new line");
    });
}

pub fn run_strategies_save_results<F>(
    set: &[Variations],
    create_ai: F,
    max_length: usize,
    runs: usize,
    filename: &str,
) where
    F: Fn(Strategy) -> Box<dyn AI>,
{
    GameEngine::create_stores();
    let strategies = generate_strategies(set, max_length);
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    let mut count = 0;
    let total_count = strategies.len();
    strategies.iter().for_each(|strategy| {
        count += 1;
        println!("{}/{}", count, total_count);
        f.write_fmt(format_args!("{},", strategy_to_str(strategy)))
            .expect("Failed to write strategy information to file");
        let results = run_strategy(create_ai(strategy.clone()), Vec::new(), runs);
        let mut results_iter = results.iter().peekable();
        while let Some(score) = results_iter.next() {
            f.write_fmt(format_args!("{}", score))
                .expect("failed to write final score");
            if results_iter.peek().is_some() {
                f.write(",".as_bytes()).expect("Failed to write comma");
            }
        }
        f.write("\n".as_bytes()).expect("Failed to write new line");
    });
}

fn run_strategy(mut ai: Box<dyn AI>, mut current_results: Vec<u64>, runs: usize) -> Vec<u64> {
    let mut current_runs = current_results.len();
    while current_runs <= runs {
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
        current_results.push(GameEngine::get_score(board));
        current_runs += 1;
    }
    current_results
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
