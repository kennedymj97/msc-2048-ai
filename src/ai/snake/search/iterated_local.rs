use super::{
    local::local_search, print_best_strategy_info, strategy_duel, Runs, SnakeData,
    StrategyDuelResult,
};
use crate::ai::snake::{ban_rules::BanMove, mann_whitney::Confidence, try_rules::TryMove, Rule};
use crate::engine::GameEngine;
use rand::{seq::IteratorRandom, thread_rng};

pub fn iterated_local_search(engine: &GameEngine, snake_data: SnakeData) -> SnakeData {
    println!("\n\nStarting ILS...");
    let mut current_best = local_search(engine, snake_data);
    for count in 0..10 {
        println!("Mutating {}/10", count + 1);
        let mutated = permutate(&current_best);
        let mut mutated_best = local_search(engine, mutated);
        match strategy_duel(
            engine,
            &mut current_best,
            &mut mutated_best,
            Runs {
                current: 5,
                max: 10000,
            },
            Confidence::P01,
        ) {
            StrategyDuelResult::Champion(results) => {
                println!("The mutation did not escape local maxima");
                current_best = results;
            }
            StrategyDuelResult::Challenger(results) => {
                println!("New local maxima found");
                current_best = results;
            }
        }
    }
    print_best_strategy_info(engine, &mut current_best);
    current_best
}

fn permutate(current_best: &SnakeData) -> SnakeData {
    let mut mutated = current_best.clone();
    let mut rng = thread_rng();
    // select random rule
    let random_rule = current_best
        .clone()
        .strategy
        .try_rules
        .into_iter()
        .choose(&mut rng)
        .expect("could not pick random rule");

    loop {
        let new_rule = TryMove::generate_all_variations()
            .into_iter()
            .filter(|&rule| {
                rule != random_rule || !current_best.clone().strategy.try_rules.contains(&rule)
            })
            .choose(&mut rng)
            .expect("failed to select new rule");
        match mutated.strategy.swap_try_rule(random_rule, new_rule) {
            Some(valid_snake) => {
                mutated.strategy = valid_snake;
                return mutated;
            }
            None => (),
        }
    }
}
