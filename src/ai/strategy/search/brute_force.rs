use super::get_count_mod;
use super::strategy_duel;
use super::Confidence;
use super::Runs;
use super::Strategy;
use super::StrategyData;
use super::StrategyDuelResult;
use crate::ai::strategy::generate_strategies::get_strategy_iterator;
use crate::engine::{GameEngineStores, Move};

pub fn brute_force(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) -> StrategyData {
    println!("Generating strategys...");
    let strategys_iter = get_strategy_iterator(max_ban_length, max_try_length);
    let mut count = 0;
    let runs = 1000;
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
            strategy,
            results: Vec::new(),
        };
        let duel_results = strategy_duel(
            engine,
            &mut best_strategy_data,
            &mut challenger,
            Runs {
                current: runs,
                max: runs,
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
