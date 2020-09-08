use super::{
    find_best_fallback_set, print_best_strategy_info, strategy_duel, Runs, StrategyData,
    StrategyDuelResult,
};
use crate::ai::strategy::ban_rules::BanMove;
use crate::ai::strategy::generate_strategies::number_of_possible_strategies;
use crate::ai::strategy::mann_whitney::Confidence;
use crate::ai::strategy::try_rules::TryMove;
use crate::ai::strategy::Strategy;
use crate::engine::GameEngine;
use crate::engine::Move;

pub enum Greedy {
    PrioritiseTry,
    PrioritiseBan,
    PrioritiseBest,
}

pub fn greedy_prioritise_best<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
) -> StrategyData {
    greedy(
        engine,
        max_ban_length,
        max_try_length,
        Greedy::PrioritiseBest,
    )
}

pub fn greedy_prioritise_try<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
) -> StrategyData {
    greedy(
        engine,
        max_ban_length,
        max_try_length,
        Greedy::PrioritiseTry,
    )
}

pub fn greedy_prioritise_ban<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
) -> StrategyData {
    greedy(
        engine,
        max_ban_length,
        max_try_length,
        Greedy::PrioritiseBan,
    )
}

fn greedy<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
    greedy_type: Greedy,
) -> StrategyData {
    println!("Starting greedy search");
    let confidence = Confidence::P01;
    let max_runs = 20000;
    let mut best_strategy_data = match greedy_type {
        Greedy::PrioritiseTry => {
            _greedy_prioritise_try(engine, max_ban_length, max_try_length, confidence, max_runs)
        }
        Greedy::PrioritiseBan => {
            _greedy_prioritise_ban(engine, max_ban_length, max_try_length, confidence, max_runs)
        }
        Greedy::PrioritiseBest => {
            _greedy_prioritise_best(engine, max_ban_length, max_try_length, confidence, max_runs)
        }
    };
    print_best_strategy_info(engine, &mut best_strategy_data);
    best_strategy_data
}

fn _greedy_prioritise_best<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
    confidence: Confidence,
    max_runs: usize,
) -> StrategyData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    println!(
        "Total number of possible strategies: {}",
        number_of_possible_strategies(1, ban_variants.len(), 4, try_variants.len())
    );
    loop {
        println!("\n\nNew best strategy: {}", best_strategy_data.strategy);
        let mut best_front_data = best_strategy_data.clone();
        let mut best_back_data = best_strategy_data.clone();
        let mut best_ban_data = best_strategy_data.clone();
        let mut front_variant_idx = None;
        let mut back_variant_idx = None;
        let mut ban_variant_idx = None;
        let max_try_length_reached = best_strategy_data.strategy.try_rules.len() >= max_try_length;
        let max_ban_length_reached = best_strategy_data.strategy.ban_rules.len() >= max_ban_length;
        if max_try_length_reached && max_ban_length_reached {
            return best_strategy_data;
        }
        if !max_try_length_reached {
            // try all remaining try rules at the front
            println!("Finding best front strategy...");
            let front_results = find_best_try_front_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );
            best_front_data = front_results.0;
            front_variant_idx = front_results.1;
            // try all remaining try rules at the back
            println!("Finding best back strategy...");
            let back_results = find_best_try_back_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );
            best_back_data = back_results.0;
            back_variant_idx = back_results.1;
            if max_ban_length_reached {
                let (best_try_data, best_try_idx) = match strategy_duel(
                    engine,
                    &mut best_front_data,
                    &mut best_back_data,
                    Runs {
                        current: 10,
                        max: max_runs,
                    },
                    confidence,
                ) {
                    StrategyDuelResult::Champion(results) => (results, front_variant_idx),
                    StrategyDuelResult::Challenger(results) => (results, back_variant_idx),
                };
                best_strategy_data = best_try_data;
                if let Some(idx) = best_try_idx {
                    try_variants.remove(idx);
                }
                continue;
            }
        }
        if !max_ban_length_reached {
            // try all remaining ban rules
            println!("Finding best ban strategy...");
            let ban_results = find_best_ban_rule(
                engine,
                &best_strategy_data,
                &ban_variants,
                confidence,
                max_runs,
            );
            best_ban_data = ban_results.0;
            ban_variant_idx = ban_results.1;
            if max_try_length_reached {
                best_strategy_data = best_ban_data;
                if let Some(idx) = ban_variant_idx {
                    ban_variants.remove(idx);
                }
                continue;
            }
        }

        println!("Comparing best rules...");
        match strategy_duel(
            engine,
            &mut best_front_data,
            &mut best_back_data,
            Runs {
                current: 10,
                max: max_runs,
            },
            confidence,
        ) {
            StrategyDuelResult::Champion(_) => {
                match strategy_duel(
                    engine,
                    &mut best_front_data,
                    &mut best_ban_data,
                    Runs {
                        current: 10,
                        max: max_runs,
                    },
                    confidence,
                ) {
                    StrategyDuelResult::Champion(_) => {
                        best_strategy_data = best_front_data;
                        if let Some(idx) = front_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuelResult::Challenger(_) => {
                        best_strategy_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
            StrategyDuelResult::Challenger(_) => {
                match strategy_duel(
                    engine,
                    &mut best_back_data,
                    &mut best_ban_data,
                    Runs {
                        current: 10,
                        max: max_runs,
                    },
                    confidence,
                ) {
                    StrategyDuelResult::Champion(_) => {
                        best_strategy_data = best_back_data;
                        if let Some(idx) = back_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuelResult::Challenger(_) => {
                        best_strategy_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
        }
    }
}

fn _greedy_prioritise_try<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
    confidence: Confidence,
    max_runs: usize,
) -> StrategyData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        println!("\n\nNew best strategy: {}", best_strategy_data.strategy);
        if best_strategy_data.strategy.try_rules.len() < max_try_length {
            // try all remaining try rules at the front
            println!("Finding best try strategy...");
            let (mut best_front_data, front_variant_idx) = find_best_try_front_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );
            // try all remaining try rules at the back
            let (mut best_back_data, back_variant_idx) = find_best_try_back_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );

            let (best_try_data, best_try_idx) = match strategy_duel(
                engine,
                &mut best_front_data,
                &mut best_back_data,
                Runs {
                    current: 10,
                    max: max_runs,
                },
                confidence,
            ) {
                StrategyDuelResult::Champion(results) => (results, front_variant_idx),
                StrategyDuelResult::Challenger(results) => (results, back_variant_idx),
            };

            if &best_try_data.strategy != &best_strategy_data.strategy {
                println!("New try rule added.");
                best_strategy_data = best_try_data;
                if let Some(idx) = best_try_idx {
                    try_variants.remove(idx);
                }
                continue;
            }
        }

        if best_strategy_data.strategy.ban_rules.len() < max_ban_length {
            // try all remaining ban rules
            println!("Finding best ban strategy...");
            let (best_ban_data, best_ban_idx) = find_best_ban_rule(
                engine,
                &best_strategy_data,
                &ban_variants,
                confidence,
                max_runs,
            );

            if &best_ban_data.strategy != &best_strategy_data.strategy {
                println!("New ban rule added.");
                best_strategy_data = best_ban_data;
                if let Some(idx) = best_ban_idx {
                    ban_variants.remove(idx);
                }
                continue;
            }
        }

        return best_strategy_data;
    }
}

fn _greedy_prioritise_ban<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
    confidence: Confidence,
    max_runs: usize,
) -> StrategyData {
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
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        println!("\n\nNew best strategy: {}", best_strategy_data.strategy);
        if best_strategy_data.strategy.ban_rules.len() < max_ban_length {
            // try all remaining ban rules
            println!("Finding best ban strategy...");
            let (best_ban_data, best_ban_idx) = find_best_ban_rule(
                engine,
                &best_strategy_data,
                &ban_variants,
                confidence,
                max_runs,
            );

            if &best_ban_data.strategy != &best_strategy_data.strategy {
                println!("New ban rule added.");
                best_strategy_data = best_ban_data;
                if let Some(idx) = best_ban_idx {
                    ban_variants.remove(idx);
                }
                continue;
            }
        }

        if best_strategy_data.strategy.try_rules.len() < max_try_length {
            // try all remaining try rules at the front
            println!("Finding best try strategy...");
            let (mut best_front_data, front_variant_idx) = find_best_try_front_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );
            // try all remaining try rules at the back
            let (mut best_back_data, back_variant_idx) = find_best_try_back_rule(
                engine,
                &best_strategy_data,
                &try_variants,
                confidence,
                max_runs,
            );

            let (best_try_data, best_try_idx) = match strategy_duel(
                engine,
                &mut best_front_data,
                &mut best_back_data,
                Runs {
                    current: 10,
                    max: max_runs,
                },
                confidence,
            ) {
                StrategyDuelResult::Champion(results) => (results, front_variant_idx),
                StrategyDuelResult::Challenger(results) => (results, back_variant_idx),
            };

            if &best_try_data.strategy != &best_strategy_data.strategy {
                println!("New try rule added.");
                best_strategy_data = best_try_data;
                if let Some(idx) = best_try_idx {
                    try_variants.remove(idx);
                }
                continue;
            }
        }

        return best_strategy_data;
    }
}

// Will compare the current best try rule with all possible options for fallback set and will
// return the best
fn find_best_try_front_rule<T: GameEngine>(
    engine: &T,
    strategy_data: &StrategyData,
    try_variants: &Vec<TryMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (StrategyData, Option<usize>) {
    let mut best_strategy_data = strategy_data.clone();
    let mut rule_added_idx = None;
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = strategy_data.strategy.try_rules.clone();
        new_try_rules.push(try_rule);
        let mut challenger;
        match Strategy::new(
            &strategy_data.strategy.ban_rules,
            &new_try_rules,
            &strategy_data.strategy.fallback_moves,
        ) {
            Some(valid_strategy) => {
                challenger = StrategyData {
                    strategy: valid_strategy,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger);
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
                if rule_added_idx.is_none() {
                    best_strategy_data = challenger;
                    rule_added_idx = Some(idx);
                } else {
                    best_strategy_data = results;
                }
            }
            StrategyDuelResult::Challenger(results) => {
                best_strategy_data = results;
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_strategy_data, rule_added_idx)
}

fn find_best_try_back_rule<T: GameEngine>(
    engine: &T,
    strategy_data: &StrategyData,
    try_variants: &Vec<TryMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (StrategyData, Option<usize>) {
    let mut best_strategy_data = strategy_data.clone();
    let mut rule_added_idx = None;
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = strategy_data.strategy.try_rules.clone();
        new_try_rules.insert(0, try_rule);
        let mut challenger;
        match Strategy::new(
            &strategy_data.strategy.ban_rules,
            &new_try_rules,
            &strategy_data.strategy.fallback_moves,
        ) {
            Some(valid_strategy) => {
                challenger = StrategyData {
                    strategy: valid_strategy,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger);
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
                if rule_added_idx.is_none() {
                    best_strategy_data = challenger;
                    rule_added_idx = Some(idx);
                } else {
                    best_strategy_data = results;
                }
            }
            StrategyDuelResult::Challenger(results) => {
                best_strategy_data = results;
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_strategy_data, rule_added_idx)
}

fn find_best_ban_rule<T: GameEngine>(
    engine: &T,
    strategy_data: &StrategyData,
    ban_variants: &Vec<BanMove>,
    confidence: Confidence,
    max_runs: usize,
) -> (StrategyData, Option<usize>) {
    let mut best_strategy_data = strategy_data.clone();
    let mut rule_added_idx = None;
    for (idx, &ban_rule) in ban_variants.iter().enumerate() {
        let mut new_ban_rules = strategy_data.strategy.ban_rules.clone();
        new_ban_rules.push(ban_rule);
        let mut challenger;
        match Strategy::new(
            &new_ban_rules,
            &strategy_data.strategy.try_rules,
            &strategy_data.strategy.fallback_moves,
        ) {
            Some(valid_strategy) => {
                challenger = StrategyData {
                    strategy: valid_strategy,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        challenger = find_best_fallback_set(engine, challenger);
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
                if rule_added_idx.is_none() {
                    best_strategy_data = challenger;
                    rule_added_idx = Some(idx);
                } else {
                    best_strategy_data = results;
                }
            }
            StrategyDuelResult::Challenger(results) => {
                best_strategy_data = results;
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_strategy_data, rule_added_idx)
}
