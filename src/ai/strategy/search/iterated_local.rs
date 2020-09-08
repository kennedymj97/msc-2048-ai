use super::{
    average, median, print_best_strategy_info, strategy_duel, Runs, StrategyData,
    StrategyDuelResult,
};
use crate::ai::strategy::{ban_rules::BanMove, mann_whitney::Confidence, try_rules::TryMove, Rule};
use crate::engine::GameEngine;
use rand::{seq::IteratorRandom, thread_rng};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

pub fn ils_mutate_try_always_accept<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    iterated_local_search(
        engine,
        strategy_data,
        permutate_try_only,
        ILSVariation::AlwaysAccept,
    )
}

pub fn ils_mutate_any_always_accept<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    iterated_local_search(
        engine,
        strategy_data,
        permutate_any,
        ILSVariation::AlwaysAccept,
    )
}

pub fn ils_mutate_try_accept_if_better<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    iterated_local_search(
        engine,
        strategy_data,
        permutate_try_only,
        ILSVariation::OnlyAcceptIfBetter,
    )
}

pub fn ils_mutate_any_accept_if_better<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    iterated_local_search(
        engine,
        strategy_data,
        permutate_any,
        ILSVariation::OnlyAcceptIfBetter,
    )
}

fn iterated_local_search<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
    mutation_fn: fn(&StrategyData) -> StrategyData,
    variation: ILSVariation,
) -> StrategyData {
    println!("\n\nStarting ILS...");
    let mut global_best = super::local::local_search_ban_restart(engine, strategy_data);
    let mut current_best = global_best.clone();
    let max_count = 20;
    for count in 0..max_count {
        println!("Mutating {}/{}", count + 1, max_count);
        let mutated = mutation_fn(&current_best);
        println!("\nCurrent best:\n{}", current_best.strategy);
        println!("\nMutated:\n{}", mutated.strategy);
        let mut mutated_best = super::local::local_search_ban_restart(engine, mutated);
        match variation {
            ILSVariation::AlwaysAccept => {
                current_best = mutated_best;
                match strategy_duel(
                    engine,
                    &mut global_best,
                    &mut current_best,
                    Runs {
                        current: 5,
                        max: 50000,
                    },
                    Confidence::P01,
                ) {
                    StrategyDuelResult::Champion(results) => {
                        println!("The mutation did not escape local maxima");
                        global_best = results;
                    }
                    StrategyDuelResult::Challenger(results) => {
                        println!("New global maxima found");
                        global_best = results;
                    }
                }
            }
            ILSVariation::OnlyAcceptIfBetter => {
                match strategy_duel(
                    engine,
                    &mut current_best,
                    &mut mutated_best,
                    Runs {
                        current: 5,
                        max: 50000,
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
        }
    }
    if variation == ILSVariation::OnlyAcceptIfBetter {
        global_best = current_best;
    }
    print_best_strategy_info(engine, &mut global_best);
    global_best
}

pub fn ils_mutate_try_always_accept_save<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
    filename: &str,
) {
    iterated_local_search_save(
        engine,
        strategy_data,
        permutate_try_only,
        ILSVariation::AlwaysAccept,
        filename,
    )
}

pub fn ils_mutate_try_accept_if_better_save<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
    filename: &str,
) {
    iterated_local_search_save(
        engine,
        strategy_data,
        permutate_try_only,
        ILSVariation::OnlyAcceptIfBetter,
        filename,
    )
}

fn iterated_local_search_save<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
    mutation_fn: fn(&StrategyData) -> StrategyData,
    variation: ILSVariation,
    filename: &str,
) {
    let path = Path::new(filename);
    let mut file = File::create(path).expect("Failed to create file");
    // add the column headers to the csv file
    file.write("run,strategy,median_score,average_score,time\n".as_bytes())
        .expect("failed to write headers to file");
    println!("\n\nStarting ILS...");
    let mut global_best = super::local::local_search_ban_restart(engine, strategy_data);
    let mut current_best = global_best.clone();
    let mut count = 0;
    let start_time = SystemTime::now();
    loop {
        count += 1;
        println!("Mutating {}", count);
        let mutated = mutation_fn(&current_best);
        println!("\nCurrent best:\n{}", current_best.strategy);
        println!("\nMutated:\n{}", mutated.strategy);
        let mut mutated_best = super::local::local_search_ban_restart(engine, mutated);
        match variation {
            ILSVariation::AlwaysAccept => {
                current_best = mutated_best;
                match strategy_duel(
                    engine,
                    &mut global_best,
                    &mut current_best,
                    Runs {
                        current: 10,
                        max: 50000,
                    },
                    Confidence::P01,
                ) {
                    StrategyDuelResult::Champion(results) => {
                        println!("The mutation did not escape local maxima");
                        global_best = results;
                        let time_elapsed = match start_time.elapsed() {
                            Ok(elapsed) => elapsed.as_millis(),
                            Err(e) => panic!(e),
                        };
                        file.write_fmt(format_args!(
                            "{},{},{},{},{}\n",
                            count,
                            global_best.strategy,
                            median(&global_best.results),
                            average(&global_best.results),
                            time_elapsed
                        ))
                        .expect("Failed to save search data");
                    }
                    StrategyDuelResult::Challenger(results) => {
                        println!("New global maxima found");
                        global_best = results;
                        let time_elapsed = match start_time.elapsed() {
                            Ok(elapsed) => elapsed.as_millis(),
                            Err(e) => panic!(e),
                        };
                        file.write_fmt(format_args!(
                            "{},{},{},{},{}\n",
                            count,
                            global_best.strategy,
                            median(&global_best.results),
                            average(&global_best.results),
                            time_elapsed
                        ))
                        .expect("Failed to save search data");
                    }
                }
            }
            ILSVariation::OnlyAcceptIfBetter => {
                match strategy_duel(
                    engine,
                    &mut current_best,
                    &mut mutated_best,
                    Runs {
                        current: 10,
                        max: 50000,
                    },
                    Confidence::P01,
                ) {
                    StrategyDuelResult::Champion(results) => {
                        println!("The mutation did not escape local maxima");
                        current_best = results;
                        let time_elapsed = match start_time.elapsed() {
                            Ok(elapsed) => elapsed.as_millis(),
                            Err(e) => panic!(e),
                        };
                        file.write_fmt(format_args!(
                            "{},{},{},{},{}\n",
                            count,
                            current_best.strategy,
                            median(&current_best.results),
                            average(&current_best.results),
                            time_elapsed
                        ))
                        .expect("Failed to save search data");
                    }
                    StrategyDuelResult::Challenger(results) => {
                        println!("New local maxima found");
                        current_best = results;
                        let time_elapsed = match start_time.elapsed() {
                            Ok(elapsed) => elapsed.as_millis(),
                            Err(e) => panic!(e),
                        };
                        file.write_fmt(format_args!(
                            "{},{},{},{},{}\n",
                            count,
                            current_best.strategy,
                            median(&current_best.results),
                            average(&current_best.results),
                            time_elapsed
                        ))
                        .expect("Failed to save search data");
                    }
                }
            }
        }
    }
}
#[derive(PartialEq)]
enum ILSVariation {
    AlwaysAccept,
    OnlyAcceptIfBetter,
}

fn permutate_try_only(current_best: &StrategyData) -> StrategyData {
    if current_best.strategy.try_rules.len() == 0 {
        return current_best.to_owned();
    }
    let mut mutated = current_best.clone();
    let mut rng = thread_rng();
    // select random rule
    let rule_to_change = current_best
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
                rule != rule_to_change && !current_best.clone().strategy.try_rules.contains(&rule)
            })
            .choose(&mut rng)
            .expect("failed to select new rule");
        match mutated.strategy.swap_try_rule(rule_to_change, new_rule) {
            Some(valid_strategy) => {
                mutated.strategy = valid_strategy;
                mutated.results = Vec::new();
                return mutated;
            }
            None => (),
        }
    }
}

fn permutate_any(current_best: &StrategyData) -> StrategyData {
    let mut mutated = current_best.clone();
    let mut rng = thread_rng();
    // select random rule
    let rule_to_change = current_best
        .clone()
        .strategy
        .get_rules_ban_first()
        .into_iter()
        .choose(&mut rng)
        .expect("could not pick random rule");

    loop {
        match rule_to_change {
            Rule::Try(try_rule) => {
                let new_rule = TryMove::generate_all_variations()
                    .into_iter()
                    .filter(|&rule| {
                        rule != try_rule && !current_best.clone().strategy.try_rules.contains(&rule)
                    })
                    .choose(&mut rng)
                    .expect("failed to select new rule");
                match mutated.strategy.swap_try_rule(try_rule, new_rule) {
                    Some(valid_strategy) => {
                        mutated.strategy = valid_strategy;
                        mutated.results = Vec::new();
                        return mutated;
                    }
                    None => (),
                }
            }
            Rule::Ban(ban_rule) => {
                let new_rule = BanMove::generate_all_variations()
                    .into_iter()
                    .filter(|&rule| {
                        rule != ban_rule && !current_best.clone().strategy.ban_rules.contains(&rule)
                    })
                    .choose(&mut rng)
                    .expect("failed to select new rule");
                match mutated.strategy.swap_ban_rule(ban_rule, new_rule) {
                    Some(valid_strategy) => {
                        mutated.strategy = valid_strategy;
                        mutated.results = Vec::new();
                        return mutated;
                    }
                    None => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::strategy::{attributes::Column, Strategy};
    use crate::engine::Move;

    #[test]
    fn it_permutate() {
        let strategy = Strategy::new(
            &vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)],
            &vec![
                TryMove::ProducesMerge(Move::Up),
                TryMove::ProducesMerge(Move::Down),
            ],
            &vec![Move::Left, Move::Down, Move::Up, Move::Right],
        )
        .unwrap();
        let mut strategy_data = StrategyData {
            strategy,
            results: Vec::new(),
        };
        for _ in 0..1000 {
            let new_strategy_data = permutate_try_only(&strategy_data);
            println!("Starting try_rules: {:?}", strategy_data.strategy.try_rules);
            println!("New try_rules: {:?}", new_strategy_data.strategy.try_rules);
            let common_rule_count = strategy_data
                .strategy
                .try_rules
                .iter()
                .zip(&new_strategy_data.strategy.try_rules)
                .filter(|&(original_rule, new_rule)| original_rule == new_rule)
                .count();
            assert_eq!(
                common_rule_count,
                strategy_data.strategy.try_rules.len() - 1
            );
            strategy_data = new_strategy_data;
        }
    }
}
