use super::average;
use super::median;
use super::run_strategy;
use crate::ai::snake::ban_rules::BanMove;
use crate::ai::snake::generate_strategies::number_of_possible_strategies;
use crate::ai::snake::generate_strategies::permutations;
use crate::ai::snake::mann_whitney::mann_whitney_u_test;
use crate::ai::snake::mann_whitney::Confidence;
use crate::ai::snake::try_rules::TryMove;
use crate::ai::snake::Snake;
use crate::engine::GameEngine;
use crate::engine::Move;
use crate::engine::Score;
use std::cmp::Ordering;

#[derive(Clone)]
struct SnakeData {
    strategy: Snake,
    results: Vec<Score>,
}

pub enum Greedy {
    PrioritiseTry,
    PrioritiseBan,
    PrioritiseBest,
}

struct GreedyRuns {
    current: usize,
    max: usize,
}

pub fn greedy(greedy_type: Greedy, confidence: Confidence, max_runs: usize) {
    println!("Starting greedy search");
    let engine = GameEngine::new();
    let mut best_strategy_data = match greedy_type {
        Greedy::PrioritiseTry => greedy_prioritise_try(&engine, confidence, max_runs),
        Greedy::PrioritiseBan => greedy_prioritise_ban(&engine, confidence, max_runs),
        Greedy::PrioritiseBest => greedy_prioritise_best(&engine, confidence, max_runs),
    };
    println!("\n\nGetting stats for best strategy...");
    run_strategy(
        &mut best_strategy_data.strategy,
        &engine,
        &mut best_strategy_data.results,
        10000,
    );
    let median = median(&best_strategy_data.results);
    let average = average(&best_strategy_data.results);
    println!(
        "Strategy: {}\nMedian: {}\nAverage: {}",
        best_strategy_data.strategy, median, average
    );
}

fn greedy_prioritise_best(
    engine: &GameEngine,
    confidence: Confidence,
    max_runs: usize,
) -> SnakeData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    println!(
        "Total number of possible strategies: {}",
        number_of_possible_strategies(4, ban_variants.len(), 5, try_variants.len())
    );
    loop {
        println!("\n\nNew best strategy: {}", best_snake_data.strategy);
        // try all remaining try rules at the front
        println!("Finding best front strategy...");
        let (mut best_front_data, front_variant_idx) = find_best_try_front_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );
        // try all remaining try rules at the back
        println!("Finding best back strategy...");
        let (mut best_back_data, back_variant_idx) = find_best_try_back_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );
        // try all remaining ban rules
        println!("Finding best ban strategy...");
        let (mut best_ban_data, ban_variant_idx) = find_best_ban_rule(
            &engine,
            &best_snake_data,
            &ban_variants,
            confidence,
            max_runs,
        );
        // set the best rules to be the best of the three
        if &best_front_data.strategy == &best_snake_data.strategy
            && &best_back_data.strategy == &best_snake_data.strategy
            && &best_ban_data.strategy == &best_snake_data.strategy
        {
            return best_snake_data;
        }

        println!("Comparing best rules...");
        match strategy_duel(
            &engine,
            &mut best_front_data,
            &mut best_back_data,
            GreedyRuns {
                current: 10,
                max: max_runs,
            },
            confidence,
        ) {
            StrategyDuelResult::Champion(_) => {
                match strategy_duel(
                    &engine,
                    &mut best_front_data,
                    &mut best_ban_data,
                    GreedyRuns {
                        current: 10,
                        max: max_runs,
                    },
                    confidence,
                ) {
                    StrategyDuelResult::Champion(_) => {
                        best_snake_data = best_front_data;
                        if let Some(idx) = front_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuelResult::Challenger(_) => {
                        best_snake_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
            StrategyDuelResult::Challenger(_) => {
                match strategy_duel(
                    &engine,
                    &mut best_back_data,
                    &mut best_ban_data,
                    GreedyRuns {
                        current: 10,
                        max: max_runs,
                    },
                    confidence,
                ) {
                    StrategyDuelResult::Champion(_) => {
                        best_snake_data = best_back_data;
                        if let Some(idx) = back_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuelResult::Challenger(_) => {
                        best_snake_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
        }
    }
}

fn greedy_prioritise_try(
    engine: &GameEngine,
    confidence: Confidence,
    max_runs: usize,
) -> SnakeData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        println!("\n\nNew best strategy: {}", best_snake_data.strategy);
        // try all remaining try rules at the front
        println!("Finding best try strategy...");
        let (mut best_front_data, front_variant_idx) = find_best_try_front_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );
        // try all remaining try rules at the back
        let (mut best_back_data, back_variant_idx) = find_best_try_back_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );

        let (best_try_data, best_try_idx) = match strategy_duel(
            &engine,
            &mut best_front_data,
            &mut best_back_data,
            GreedyRuns {
                current: 10,
                max: max_runs,
            },
            confidence,
        ) {
            StrategyDuelResult::Champion(results) => (results, front_variant_idx),
            StrategyDuelResult::Challenger(results) => (results, back_variant_idx),
        };

        if &best_try_data.strategy != &best_snake_data.strategy {
            println!("New try rule added.");
            best_snake_data = best_try_data;
            if let Some(idx) = best_try_idx {
                try_variants.remove(idx);
            }
            continue;
        }

        // try all remaining ban rules
        println!("Finding best ban strategy...");
        let (best_ban_data, best_ban_idx) = find_best_ban_rule(
            &engine,
            &best_snake_data,
            &ban_variants,
            confidence,
            max_runs,
        );

        if &best_ban_data.strategy != &best_snake_data.strategy {
            println!("New ban rule added.");
            best_snake_data = best_ban_data;
            if let Some(idx) = best_ban_idx {
                ban_variants.remove(idx);
            }
            continue;
        }

        return best_snake_data;
    }
}

fn greedy_prioritise_ban(
    engine: &GameEngine,
    confidence: Confidence,
    max_runs: usize,
) -> SnakeData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        println!("\n\nNew best strategy: {}", best_snake_data.strategy);
        // try all remaining ban rules
        println!("Finding best ban strategy...");
        let (best_ban_data, best_ban_idx) = find_best_ban_rule(
            &engine,
            &best_snake_data,
            &ban_variants,
            confidence,
            max_runs,
        );

        if &best_ban_data.strategy != &best_snake_data.strategy {
            println!("New ban rule added.");
            best_snake_data = best_ban_data;
            if let Some(idx) = best_ban_idx {
                ban_variants.remove(idx);
            }
            continue;
        }

        // try all remaining try rules at the front
        println!("Finding best try strategy...");
        let (mut best_front_data, front_variant_idx) = find_best_try_front_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );
        // try all remaining try rules at the back
        let (mut best_back_data, back_variant_idx) = find_best_try_back_rule(
            &engine,
            &best_snake_data,
            &try_variants,
            confidence,
            max_runs,
        );

        let (best_try_data, best_try_idx) = match strategy_duel(
            &engine,
            &mut best_front_data,
            &mut best_back_data,
            GreedyRuns {
                current: 10,
                max: max_runs,
            },
            confidence,
        ) {
            StrategyDuelResult::Champion(results) => (results, front_variant_idx),
            StrategyDuelResult::Challenger(results) => (results, back_variant_idx),
        };

        if &best_try_data.strategy != &best_snake_data.strategy {
            println!("New try rule added.");
            best_snake_data = best_try_data;
            if let Some(idx) = best_try_idx {
                try_variants.remove(idx);
            }
            continue;
        }

        return best_snake_data;
    }
}

// Will compare the current best try rule with all possible options for fallback set and will
// return the best
fn find_best_try_front_rule(
    engine: &GameEngine,
    snake_data: &SnakeData,
    try_variants: &Vec<TryMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (SnakeData, Option<usize>) {
    let mut best_snake_data = snake_data.clone();
    let mut rule_added_idx = None;
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = snake_data.strategy.try_rules.clone();
        new_try_rules.push(try_rule);
        let mut challenger;
        match Snake::new(
            &snake_data.strategy.ban_rules,
            &new_try_rules,
            &snake_data.strategy.fallback_moves,
        ) {
            Some(valid_snake) => {
                challenger = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger, confidence, max_runs);
        let duel_results = strategy_duel(
            engine,
            &mut best_snake_data,
            &mut challenger,
            GreedyRuns {
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
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_snake_data, rule_added_idx)
}

fn find_best_try_back_rule(
    engine: &GameEngine,
    snake_data: &SnakeData,
    try_variants: &Vec<TryMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (SnakeData, Option<usize>) {
    let mut best_snake_data = snake_data.clone();
    let mut rule_added_idx = None;
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = snake_data.strategy.try_rules.clone();
        new_try_rules.insert(0, try_rule);
        let mut challenger;
        match Snake::new(
            &snake_data.strategy.ban_rules,
            &new_try_rules,
            &snake_data.strategy.fallback_moves,
        ) {
            Some(valid_snake) => {
                challenger = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger, confidence, max_runs);
        let duel_results = strategy_duel(
            engine,
            &mut best_snake_data,
            &mut challenger,
            GreedyRuns {
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
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_snake_data, rule_added_idx)
}

fn find_best_ban_rule(
    engine: &GameEngine,
    snake_data: &SnakeData,
    ban_variants: &Vec<BanMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (SnakeData, Option<usize>) {
    let mut best_snake_data = snake_data.clone();
    let mut rule_added_idx = None;
    for (idx, &ban_rule) in ban_variants.iter().enumerate() {
        let mut new_ban_rules = snake_data.strategy.ban_rules.clone();
        new_ban_rules.push(ban_rule);
        let mut challenger;
        match Snake::new(
            &new_ban_rules,
            &snake_data.strategy.try_rules,
            &snake_data.strategy.fallback_moves,
        ) {
            Some(valid_snake) => {
                challenger = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger, confidence, max_runs);
        let duel_results = strategy_duel(
            engine,
            &mut best_snake_data,
            &mut challenger,
            GreedyRuns {
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
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_snake_data, rule_added_idx)
}

fn find_best_fallback_set(
    engine: &GameEngine,
    challenger: SnakeData,
    confidence: Confidence,
    max_runs: usize,
) -> SnakeData {
    let mut best_challenger = challenger.to_owned();
    for mut fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up]]) {
        fallback_set.push(Move::Right);
        let mut challenger_alt;
        match Snake::new(
            &challenger.strategy.ban_rules,
            &challenger.strategy.try_rules,
            &fallback_set,
        ) {
            Some(valid_snake) => {
                challenger_alt = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_challenger,
            &mut challenger_alt,
            GreedyRuns {
                current: 10,
                max: max_runs,
            },
            confidence,
        );
        match duel_results {
            StrategyDuelResult::Champion(challenger_more_results) => {
                best_challenger = challenger_more_results;
            }
            StrategyDuelResult::Challenger(challenger_alt_results) => {
                best_challenger = challenger_alt_results;
            }
        }
    }
    best_challenger
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
