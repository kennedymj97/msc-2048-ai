use super::{print_best_strategy_info, strategy_duel, Runs, SnakeData, StrategyDuelResult};
use crate::ai::snake::{
    ban_rules::BanMove, ban_rules::BanRules, mann_whitney::Confidence, try_rules::TryMove,
    try_rules::TryRules, Rule,
};
use crate::engine::GameEngine;

pub fn local_search(engine: &GameEngine, snake_data: SnakeData) -> SnakeData {
    println!("Starting local search...");
    let mut best_snake_data = snake_data.clone();
    let max_runs = 10000;
    let confidence = Confidence::P01;
    // loop through all of the rules
    for &rule in snake_data.strategy.get_rules().iter() {
        println!("Trying to find alternatives to: {}", rule);
        // try changing the current rule with all possible alternatives
        match rule {
            Rule::Ban(ban_rule) => {
                // find all possible alternatives, ban variants with current ban rules removed
                let alterative_rules = BanMove::generate_all_variations()
                    .into_iter()
                    .filter(|rule| !best_snake_data.strategy.ban_rules.contains(&rule))
                    .collect::<BanRules>();

                // loop through all possible alternatives, if a better strategy is found update
                // best snake data
                for &alternative_rule in alterative_rules.iter() {
                    match best_snake_data
                        .strategy
                        .swap_ban_rule(ban_rule, alternative_rule)
                    {
                        Some(valid_snake) => {
                            // compare this snake with the current best
                            match strategy_duel(
                                engine,
                                &mut best_snake_data,
                                &mut SnakeData {
                                    strategy: valid_snake,
                                    results: Vec::new(),
                                },
                                Runs {
                                    current: 5,
                                    max: max_runs,
                                },
                                confidence,
                            ) {
                                StrategyDuelResult::Champion(results) => best_snake_data = results,
                                StrategyDuelResult::Challenger(results) => {
                                    best_snake_data = results;
                                    println!("Rule changed.",);
                                }
                            }
                        }
                        None => continue,
                    };
                }
            }

            Rule::Try(try_rule) => {
                let alterative_rules = TryMove::generate_all_variations()
                    .into_iter()
                    .filter(|rule| !best_snake_data.strategy.try_rules.contains(&rule))
                    .collect::<TryRules>();

                for &alternative_rule in alterative_rules.iter() {
                    match best_snake_data
                        .strategy
                        .swap_try_rule(try_rule, alternative_rule)
                    {
                        Some(valid_snake) => {
                            // compare this snake with the current best
                            match strategy_duel(
                                engine,
                                &mut best_snake_data,
                                &mut SnakeData {
                                    strategy: valid_snake,
                                    results: Vec::new(),
                                },
                                Runs {
                                    current: 5,
                                    max: max_runs,
                                },
                                confidence,
                            ) {
                                StrategyDuelResult::Champion(results) => best_snake_data = results,
                                StrategyDuelResult::Challenger(results) => {
                                    best_snake_data = results;
                                    println!("Rule changed.",);
                                }
                            }
                        }
                        None => continue,
                    };
                }
            }
        }
    }
    // if a rule has been changed recursively call the local search on the new best
    if best_snake_data.strategy != snake_data.strategy {
        return local_search(engine, best_snake_data);
    }

    print_best_strategy_info(engine, &mut best_snake_data);
    best_snake_data
}
