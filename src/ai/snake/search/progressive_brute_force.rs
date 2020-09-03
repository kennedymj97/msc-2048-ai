use super::get_count_mod;
use super::median;
use super::run_strategy;
use super::save_results;
use super::strategy_duel;
use super::Confidence;
use super::Runs;
use super::Snake;
use super::SnakeData;
use super::StrategyDuelResult;
use crate::ai::snake::evaluate_strategies::compare_strategies;
use crate::ai::snake::evaluate_strategies::compare_strategy_to_best;
use crate::ai::snake::evaluate_strategies::StrategyDataStore;
use crate::ai::snake::generate_strategies::generate_snakes;
use crate::ai::snake::generate_strategies::get_snake_iterator;
use crate::ai::snake::generate_strategies::get_snake_iterator_fixed_fallback;
use crate::ai::snake::generate_strategies::Iter;
use crate::ai::snake::generate_strategies::IterFixedFallback;
use crate::engine::{GameEngine, GameEngineStores, Move};
use std::fs::DirBuilder;
use std::path::Path;

pub fn new_progressive_brute_force(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) -> SnakeData {
    println!("Generating snakes...");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let mut count = 0;
    let max_runs = 1000;
    let total_count = snakes_iter.len();
    println!("Getting data...");
    let best_snake = Snake::new(
        &Vec::new(),
        &Vec::new(),
        &vec![Move::Left, Move::Down, Move::Up, Move::Right],
    )
    .expect("Failed to create initial snake");
    let mut best_snake_data = SnakeData {
        strategy: best_snake,
        results: Vec::new(),
    };
    let confidence = Confidence::P05;
    snakes_iter.for_each(|snake| {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut challenger = SnakeData {
            strategy: snake,
            results: Vec::new(),
        };
        let duel_results = strategy_duel(
            engine,
            &mut best_snake_data,
            &mut challenger,
            Runs {
                current: 10,
                max: max_runs,
            },
            confidence,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_data = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake_data = results;
            }
        }
    });
    best_snake_data
}

pub fn progressive_brute_force_no_save(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) {
    println!("Getting snakes iterator...");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let best_initial_strategies = progressive_brute_force_initial_run(snakes_iter, engine, 10);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, engine, 10 * 5);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn progressive_brute_force_initial_run<T: GameEngine>(
    snake_iter: Iter,
    engine: &T,
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

pub fn progressive_brute_force_no_save_fixed_fallback(
    max_ban_length: usize,
    max_try_length: usize,
) {
    println!("Creating engine...");
    let engine = GameEngineStores::new();
    println!("Getting snakes iterator");
    let snakes_iter = get_snake_iterator_fixed_fallback(max_ban_length, max_try_length);
    let best_initial_strategies =
        progressive_brute_force_fixed_fallback_initial_run(snakes_iter, &engine, 5);
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

fn progressive_brute_force_fixed_fallback_initial_run<T: GameEngine>(
    snake_iter: IterFixedFallback,
    engine: &T,
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
fn progressive_brute_force_no_save_aux<T: GameEngine>(
    current_best: StrategyDataStore<Snake>,
    engine: &T,
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
    progressive_brute_force_no_save_aux(compare_strategies(best), engine, runs * 5)
}

pub fn progressive_brute_force(max_ban_length: usize, max_try_length: usize, foldername: &str) {
    println!("engine...");
    let engine = GameEngineStores::new();
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

fn progressive_brute_force_aux<T: GameEngine>(
    data: StrategyDataStore<Snake>,
    engine: &T,
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
