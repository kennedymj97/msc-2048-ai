use super::evaluate_strategies::compare_strategies;
use super::evaluate_strategies::compare_strategy_to_best;
use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::generate_snakes;
use super::generate_strategies::get_snake_iterator;
use super::generate_strategies::Iter;
use super::Snake;
use crate::ai::AI;
use crate::engine::GameEngine;
use crate::engine::Score;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn progressive_brute_force_no_save(max_ban_length: usize, max_try_length: usize) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Getting snakes iterator");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let best_initial_strategies = progressive_brute_force_initial_run(snakes_iter, &engine, 5);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, &engine, 5 * 10);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn progressive_brute_force_initial_run(
    snake_iter: Iter,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Snake> {
    println!("Initial @ {} runs", runs);
    let mut count = 0;
    let total_count = snake_iter.len();
    let mut best_strategies = Vec::new();
    for mut snake in snake_iter {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut results = Vec::new();
        run_strategy(&mut snake, engine, &mut results, runs);
        best_strategies = compare_strategy_to_best((snake, results), best_strategies);
    }
    best_strategies
}

fn progressive_brute_force_no_save_aux(
    current_best: StrategyDataStore<Snake>,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Snake> {
    if runs >= 100000 {
        return current_best;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = current_best.len();
    let best = current_best
        .into_iter()
        .map(|(mut snake, mut results)| {
            count += 1;
            if count % get_count_mod(total_count) == 0 {
                println!("{}/{}", count, total_count);
            }
            run_strategy(&mut snake, engine, &mut results, runs);
            (snake, results)
        })
        .collect::<StrategyDataStore<Snake>>();
    progressive_brute_force_no_save_aux(compare_strategies(best), engine, runs * 10)
}

fn get_count_mod(len: usize) -> u64 {
    if len < 100 {
        return 1;
    }

    if len < 1000 {
        return 10;
    }

    if len < 10_000 {
        return 100;
    }

    if len < 100_000 {
        return 1000;
    }

    if len < 1_000_000 {
        return 10000;
    }

    100000
}

pub fn progressive_brute_force(max_ban_length: usize, max_try_length: usize, foldername: &str) {
    println!("engine...");
    let engine = GameEngine::new();
    let path = Path::new(foldername);
    let dir_builder = DirBuilder::new();
    dir_builder.create(path).expect("Failed to create folder");
    println!("generating snakes");
    let snakes = generate_snakes(max_ban_length, max_try_length);
    println!("creating data");
    let data = snakes
        .into_iter()
        .map(|snake| (snake, Vec::new()))
        .collect::<StrategyDataStore<Snake>>();
    println!("calling aux function");
    let best_strategies = progressive_brute_force_aux(data, &engine, 10, path);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

fn progressive_brute_force_aux(
    data: StrategyDataStore<Snake>,
    engine: &GameEngine,
    runs: usize,
    foldername: &Path,
) -> StrategyDataStore<Snake> {
    if runs > 1000 {
        return data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = data.len();
    let data = data
        .into_iter()
        .map(|(mut snake, mut results)| {
            count += 1;
            println!("{}/{}", count, total_count);
            run_strategy(&mut snake, engine, &mut results, runs);
            (snake, results)
        })
        .collect::<StrategyDataStore<Snake>>();
    save_results(&data, foldername, runs);
    progressive_brute_force_aux(compare_strategies(data), engine, runs * 10, foldername)
}

fn save_results(data: &StrategyDataStore<Snake>, foldername: &Path, runs: usize) {
    println!("Saving data @ {} runs...", runs);
    let path = foldername.join(format!("{}_runs.csv", runs));
    let mut f = File::create(path).expect("Failed to create file");
    data.iter().for_each(|(snake, results)| {
        f.write_fmt(format_args!("{},", snake))
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

pub fn brute_force(max_ban_length: usize, max_try_length: usize, runs: usize, filename: &str) {
    let engine = GameEngine::new();
    let snakes = generate_snakes(max_ban_length, max_try_length);
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    let mut count = 0;
    let total_count = snakes.len();
    snakes.into_iter().for_each(|mut snake| {
        count += 1;
        println!("{}/{}", count, total_count);
        f.write_fmt(format_args!("{},", snake))
            .expect("Failed to write strategy information to file");
        let mut results = Vec::new();
        run_strategy(&mut snake, &engine, &mut results, runs);
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

fn run_strategy<T: AI>(
    ai: &mut T,
    engine: &GameEngine,
    current_results: &mut Vec<Score>,
    runs: usize,
) {
    let mut current_runs = current_results.len();
    while current_runs <= runs {
        let mut board = GameEngine::new_board();
        loop {
            let best_move = ai.get_next_move(engine, board);
            match best_move {
                Some(direction) => {
                    board = engine.make_move(board, direction);
                }
                None => break,
            }
        }
        current_results.push(engine.get_score(board));
        current_runs += 1;
    }
}

pub fn run_strategy_save_results(mut ai: Snake) {
    let engine = GameEngine::new();
    let mut f = File::create(Path::new("strategy.csv")).expect("Failed to create file");
    f.write("score,highest tile\n".as_bytes())
        .expect("Failed to write strategy");
    (0..10000).for_each(|_| {
        let mut board = GameEngine::new_board();
        loop {
            let best_move = ai.get_next_move(&engine, board);
            match best_move {
                Some(direction) => {
                    board = engine.make_move(board, direction);
                }
                None => break,
            }
        }
        let score = engine.get_score(board);
        let highest_tile = GameEngine::get_highest_tile_val(board);
        f.write_fmt(format_args!("{},{}\n", score, highest_tile))
            .expect("failed to write data to file");
    });
}
