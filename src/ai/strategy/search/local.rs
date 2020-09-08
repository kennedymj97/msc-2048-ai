use super::{print_best_strategy_info, strategy_duel, Runs, StrategyData, StrategyDuelResult};
use crate::ai::strategy::{
    ban_rules::BanMove, ban_rules::BanRules, mann_whitney::Confidence, try_rules::TryMove,
    try_rules::TryRules, Rule, Rules, Strategy,
};
use crate::engine::GameEngine;

pub fn local_search_try_restart<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    local_search(
        engine,
        strategy_data,
        Strategy::get_rules_try_first,
        LocalSearchType::RestartOnChange,
    )
}

pub fn local_search_try_no_restart<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    local_search(
        engine,
        strategy_data,
        Strategy::get_rules_try_first,
        LocalSearchType::TryAllBeforeRestart,
    )
}

pub fn local_search_ban_restart<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    local_search(
        engine,
        strategy_data,
        Strategy::get_rules_ban_first,
        LocalSearchType::RestartOnChange,
    )
}

pub fn local_search_ban_no_restart<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
) -> StrategyData {
    local_search(
        engine,
        strategy_data,
        Strategy::get_rules_ban_first,
        LocalSearchType::TryAllBeforeRestart,
    )
}

// need to also allow restart as soon as rule changed vs change all rules before restarting
fn local_search<T: GameEngine>(
    engine: &T,
    strategy_data: StrategyData,
    get_rules: fn(&Strategy) -> Rules,
    search_type: LocalSearchType,
) -> StrategyData {
    println!("Starting local search...");
    let mut best_strategy_data = strategy_data.clone();
    let max_runs = 10000;
    let confidence = Confidence::P01;
    // loop through all of the rules
    for &rule in get_rules(&strategy_data.strategy).iter() {
        println!("Trying to find alternatives to: {}", rule);
        // try changing the current rule with all possible alternatives
        match rule {
            Rule::Ban(ban_rule) => {
                // find all possible alternatives, ban variants with current ban rules removed
                let alterative_rules = BanMove::generate_all_variations()
                    .into_iter()
                    .filter(|rule| !best_strategy_data.strategy.ban_rules.contains(&rule))
                    .collect::<BanRules>();

                // loop through all possible alternatives, if a better strategy is found update
                // best strategy data
                for &alternative_rule in alterative_rules.iter() {
                    match best_strategy_data
                        .strategy
                        .swap_ban_rule(ban_rule, alternative_rule)
                    {
                        Some(valid_strategy) => {
                            // compare this strategy with the current best
                            match strategy_duel(
                                engine,
                                &mut best_strategy_data,
                                &mut StrategyData {
                                    strategy: valid_strategy,
                                    results: Vec::new(),
                                },
                                Runs {
                                    current: 5,
                                    max: max_runs,
                                },
                                confidence,
                            ) {
                                StrategyDuelResult::Champion(results) => {
                                    best_strategy_data = results
                                }
                                StrategyDuelResult::Challenger(results) => {
                                    best_strategy_data = results;
                                    println!("Rule changed.",);
                                    if search_type == LocalSearchType::RestartOnChange {
                                        println!("Restarting search...");
                                        return local_search(
                                            engine,
                                            best_strategy_data,
                                            get_rules,
                                            search_type,
                                        );
                                    }
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
                    .filter(|rule| !best_strategy_data.strategy.try_rules.contains(&rule))
                    .collect::<TryRules>();

                for &alternative_rule in alterative_rules.iter() {
                    match best_strategy_data
                        .strategy
                        .swap_try_rule(try_rule, alternative_rule)
                    {
                        Some(valid_strategy) => {
                            // compare this strategy with the current best
                            match strategy_duel(
                                engine,
                                &mut best_strategy_data,
                                &mut StrategyData {
                                    strategy: valid_strategy,
                                    results: Vec::new(),
                                },
                                Runs {
                                    current: 5,
                                    max: max_runs,
                                },
                                confidence,
                            ) {
                                StrategyDuelResult::Champion(results) => {
                                    best_strategy_data = results
                                }
                                StrategyDuelResult::Challenger(results) => {
                                    best_strategy_data = results;
                                    println!("Rule changed.",);
                                    if search_type == LocalSearchType::RestartOnChange {
                                        println!("Restarting search...");
                                        return local_search(
                                            engine,
                                            best_strategy_data,
                                            get_rules,
                                            search_type,
                                        );
                                    }
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
    if best_strategy_data.strategy != strategy_data.strategy {
        return local_search(engine, best_strategy_data, get_rules, search_type);
    }

    print_best_strategy_info(engine, &mut best_strategy_data);
    best_strategy_data
}

#[derive(PartialEq)]
enum LocalSearchType {
    RestartOnChange,
    TryAllBeforeRestart,
}
