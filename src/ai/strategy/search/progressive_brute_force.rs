use super::get_count_mod;
use super::median;
use super::run_strategy;
use super::save_results;
use super::strategy_duel;
use super::Confidence;
use super::Runs;
use super::Strategy;
use super::StrategyData;
use super::StrategyDuelResult;
use crate::ai::strategy::evaluate_strategies::compare_strategies;
use crate::ai::strategy::evaluate_strategies::compare_strategy_to_best;
use crate::ai::strategy::evaluate_strategies::StrategyDataStore;
use crate::ai::strategy::generate_strategies::generate_strategys;
use crate::ai::strategy::generate_strategies::get_strategy_iterator;
use crate::ai::strategy::generate_strategies::get_strategy_iterator_fixed_fallback;
use crate::ai::strategy::generate_strategies::Iter;
use crate::ai::strategy::generate_strategies::IterFixedFallback;
use crate::engine::{GameEngine, GameEngineStores, Move};
use std::fs::DirBuilder;
use std::path::Path;

pub fn new_progressive_brute_force(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) -> StrategyData {
    println!("Generating strategys...");
    let strategys_iter = get_strategy_iterator(max_ban_length, max_try_length);
    let mut count = 0;
    let max_runs = 1000;
    let total_count = strategys_iter.len();
    println!("Getting data...");
    let best_strategy = Strategy::new(
        &Vec::new(),
        &Vec::new(),
        &vec![Move::Left, Move::Down, Move::Up, Move::Right],
    )
    .expect("Failed to create initial strategy");
    let mut best_strategy_data = StrategyData {
        strategy: best_strategy,
        results: Vec::new(),
    };
    let confidence = Confidence::P05;
    strategys_iter.for_each(|strategy| {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut challenger = StrategyData {
            strategy: strategy,
            results: Vec::new(),
        };
        let duel_results = strategy_duel(
            engine,
            &mut best_strategy_data,
            &mut challenger,
            Runs {
                current: 10,
                max: max_runs,
            },
            confidence,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_strategy_data = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_strategy_data = results;
            }
        }
    });
    best_strategy_data
}

pub fn progressive_brute_force_no_save(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) {
    println!("Getting strategys iterator...");
    let strategys_iter = get_strategy_iterator(max_ban_length, max_try_length);
    let best_initial_strategies = progressive_brute_force_initial_run(strategys_iter, engine, 10);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, engine, 10 * 5);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(strategy, median)| println!("{}: {}", strategy, median));
}

fn progressive_brute_force_initial_run<T: GameEngine>(
    strategy_iter: Iter,
    engine: &T,
    runs: usize,
) -> StrategyDataStore<Strategy> {
    println!("Initial @ {} runs", runs);
    let mut count = 0;
    let total_count = strategy_iter.len();
    let mut best_strategies = Vec::new();
    for mut strategy in strategy_iter {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut results = Vec::new();
        run_strategy(&mut strategy, engine, &mut results, runs);
        best_strategies = compare_strategy_to_best((strategy, results), best_strategies);
    }
    best_strategies
}

pub fn progressive_brute_force_no_save_fixed_fallback(
    max_ban_length: usize,
    max_try_length: usize,
) {
    println!("Creating engine...");
    let engine = GameEngineStores::new();
    println!("Getting strategys iterator");
    let strategys_iter = get_strategy_iterator_fixed_fallback(max_ban_length, max_try_length);
    let best_initial_strategies =
        progressive_brute_force_fixed_fallback_initial_run(strategys_iter, &engine, 5);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, &engine, 5 * 10);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(strategy, median)| println!("{}: {}", strategy, median));
}

fn progressive_brute_force_fixed_fallback_initial_run<T: GameEngine>(
    strategy_iter: IterFixedFallback,
    engine: &T,
    runs: usize,
) -> StrategyDataStore<Strategy> {
    println!("Initial @ {} runs", runs);
    let mut count = 0;
    let total_count = strategy_iter.len();
    let mut best_strategies = Vec::new();
    for mut strategy in strategy_iter {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut results = Vec::new();
        run_strategy(&mut strategy, engine, &mut results, runs);
        best_strategies = compare_strategy_to_best((strategy, results), best_strategies);
    }
    best_strategies
}
fn progressive_brute_force_no_save_aux<T: GameEngine>(
    current_best: StrategyDataStore<Strategy>,
    engine: &T,
    runs: usize,
) -> StrategyDataStore<Strategy> {
    if runs >= 100000 {
        return current_best;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = current_best.len();
    let best = current_best
        .into_iter()
        .map(|(mut strategy, mut results)| {
            count += 1;
            if count % get_count_mod(total_count) == 0 {
                println!("{}/{}", count, total_count);
            }
            run_strategy(&mut strategy, engine, &mut results, runs);
            (strategy, results)
        })
        .collect::<StrategyDataStore<Strategy>>();
    progressive_brute_force_no_save_aux(compare_strategies(best), engine, runs * 5)
}

pub fn progressive_brute_force(max_ban_length: usize, max_try_length: usize, foldername: &str) {
    println!("engine...");
    let engine = GameEngineStores::new();
    let path = Path::new(foldername);
    let dir_builder = DirBuilder::new();
    dir_builder.create(path).expect("Failed to create folder");
    println!("generating strategys");
    let strategys = generate_strategys(max_ban_length, max_try_length);
    println!("creating data");
    let data = strategys
        .into_iter()
        .map(|strategy| (strategy, Vec::new()))
        .collect::<StrategyDataStore<Strategy>>();
    println!("calling aux function");
    let best_strategies = progressive_brute_force_aux(data, &engine, 10, path);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(strategy, median)| println!("{}: {}", strategy, median));
}

fn progressive_brute_force_aux<T: GameEngine>(
    data: StrategyDataStore<Strategy>,
    engine: &T,
    runs: usize,
    foldername: &Path,
) -> StrategyDataStore<Strategy> {
    if runs > 1000 {
        return data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = data.len();
    let data = data
        .into_iter()
        .map(|(mut strategy, mut results)| {
            count += 1;
            println!("{}/{}", count, total_count);
            run_strategy(&mut strategy, engine, &mut results, runs);
            (strategy, results)
        })
        .collect::<StrategyDataStore<Strategy>>();
    save_results(&data, foldername, runs);
    progressive_brute_force_aux(compare_strategies(data), engine, runs * 10, foldername)
}
