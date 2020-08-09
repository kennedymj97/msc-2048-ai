use super::run_strategy;
use crate::ai::snake::ban_rules::BanMove;
use crate::ai::snake::generate_strategies::permutations;
use crate::ai::snake::mann_whitney::{mann_whitney_u_test, Confidence};
use crate::ai::snake::try_rules::TryMove;
use crate::ai::snake::Snake;
use crate::engine::{GameEngine, Move, Score};
use rand::{seq::IteratorRandom, thread_rng};
use std::cmp::Ordering;

// Find the best randomly generated snake
pub fn random_search(ban_length: usize, try_length: usize) {}

#[derive(Clone)]
struct SnakeData {
    strategy: Snake,
    results: Vec<Score>,
}

struct GreedyRuns {
    current: usize,
    max: usize,
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
        Confidence::P01,
        10000,
    )
}

fn find_best_fallback_set(
    engine: &GameEngine,
    snake: SnakeData,
    confidence: Confidence,
    max_runs: usize,
) -> SnakeData {
    let mut best_snake = snake.to_owned();
    for mut fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up]]) {
        fallback_set.push(Move::Right);
        let mut snake_alt;
        match Snake::new(
            &snake.strategy.ban_rules,
            &snake.strategy.try_rules,
            &fallback_set,
        ) {
            Some(valid_snake) => {
                snake_alt = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut snake_alt,
            GreedyRuns {
                current: 10,
                max: max_runs,
            },
            confidence,
        );
        match duel_results {
            StrategyDuelResult::Champion(snake_more_results) => {
                best_snake = snake_more_results;
            }
            StrategyDuelResult::Challenger(snake_alt_results) => {
                best_snake = snake_alt_results;
            }
        }
    }
    best_snake
}

enum StrategyDuelResult {
    Champion(SnakeData),
    Challenger(SnakeData),
}

fn strategy_duel(
    engine: &GameEngine,
    champion: &mut SnakeData,
    challenger: &mut SnakeData,
    runs: GreedyRuns,
    confidence: Confidence,
) -> StrategyDuelResult {
    if runs.current > runs.max {
        return StrategyDuelResult::Champion(champion.to_owned());
    }
    run_strategy(
        &mut champion.strategy,
        engine,
        &mut champion.results,
        runs.current,
    );
    run_strategy(
        &mut challenger.strategy,
        engine,
        &mut challenger.results,
        runs.current,
    );
    match mann_whitney_u_test(&champion.results, &challenger.results, confidence) {
        Ordering::Less => StrategyDuelResult::Challenger(challenger.to_owned()),
        Ordering::Equal => strategy_duel(
            engine,
            champion,
            challenger,
            GreedyRuns {
                current: runs.current * 2,
                max: runs.max,
            },
            confidence,
        ),
        Ordering::Greater => StrategyDuelResult::Champion(champion.to_owned()),
    }
}
