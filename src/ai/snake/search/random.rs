use super::{
    average, find_best_fallback_set, get_count_mod, median, run_strategy, strategy_duel, Runs,
    SnakeData, StrategyDuelResult,
};
use crate::ai::snake::ban_rules::BanMove;
use crate::ai::snake::mann_whitney::Confidence;
use crate::ai::snake::try_rules::TryMove;
use crate::ai::snake::Snake;
use crate::engine::GameEngine;
use rand::{seq::IteratorRandom, thread_rng};

// Find the best randomly generated snake
pub fn random_search(ban_length: usize, try_length: usize) -> SnakeData {
    let engine = GameEngine::new();
    let n = 10000;
    let test_runs = 1000;
    // start with random snake as best
    println!("Generating initial random snake...");
    let mut best_random_snake = get_random_snake(&engine, ban_length, try_length);
    // generate random snakes and compare with current best
    let mut count = 0;
    println!("Starting search...");
    loop {
        // if tested n strategies without finding better return the best strategy
        if count >= n {
            println!("\n\nGetting stats for best strategy...");
            run_strategy(
                &mut best_random_snake.strategy,
                &engine,
                &mut best_random_snake.results,
                10000,
            );
            let median = median(&best_random_snake.results);
            let average = average(&best_random_snake.results);
            println!(
                "Strategy: {}\nMedian: {}\nAverage: {}",
                best_random_snake.strategy, median, average
            );

            return best_random_snake;
        }
        let mut random_snake = get_random_snake(&engine, ban_length, try_length);
        // if better then replace best snake
        match strategy_duel(
            &engine,
            &mut best_random_snake,
            &mut random_snake,
            Runs {
                current: 5,
                max: test_runs,
            },
            Confidence::P05,
        ) {
            StrategyDuelResult::Champion(results) => {
                if count as u64 % get_count_mod(n) == 0 && count > 0 {
                    println!("No better strategies found: {}/{}", count, n);
                }
                best_random_snake = results;
                count += 1;
            }
            StrategyDuelResult::Challenger(results) => {
                println!("Better strategy found, restarting search...");
                best_random_snake = results;
                count = 0;
            }
        }
    }
}

// Create a random snake
fn get_random_snake(engine: &GameEngine, ban_length: usize, try_length: usize) -> SnakeData {
    let ban_variants = BanMove::generate_all_variations();
    let try_variants = TryMove::generate_all_variations();

    // Randomly sample from the variants
    let mut rng = thread_rng();
    let random_ban_rules = ban_variants
        .into_iter()
        .choose_multiple(&mut rng, ban_length);
    let random_try_rules = try_variants
        .into_iter()
        .choose_multiple(&mut rng, try_length);

    // find the best possible fallback set for the sets
    find_best_fallback_set(
        engine,
        SnakeData {
            strategy: Snake::new(&random_ban_rules, &random_try_rules, &Vec::new())
                .expect("failed to create initial random snake"),
            results: Vec::new(),
        },
    )
}
