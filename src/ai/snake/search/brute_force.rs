use super::get_count_mod;
use super::strategy_duel;
use super::Confidence;
use super::Runs;
use super::Snake;
use super::SnakeData;
use super::StrategyDuelResult;
use crate::ai::snake::generate_strategies::get_snake_iterator;
use crate::engine::{GameEngineStores, Move};

pub fn brute_force(
    engine: &GameEngineStores,
    max_ban_length: usize,
    max_try_length: usize,
) -> SnakeData {
    println!("Generating snakes...");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let mut count = 0;
    let runs = 1000;
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
                current: runs,
                max: runs,
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
