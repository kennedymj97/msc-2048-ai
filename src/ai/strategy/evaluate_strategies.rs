use super::mann_whitney::mann_whitney_u_test;
use super::mann_whitney::Confidence;
use std::cmp::Ordering;

type StrategyData<T> = (T, Vec<u64>);
pub type StrategyDataStore<T> = Vec<StrategyData<T>>;

pub fn compare_strategies<T: Clone>(data: StrategyDataStore<T>) -> StrategyDataStore<T> {
    let mut top_strategies = Vec::new();
    for strategy in data {
        top_strategies = compare_strategy_to_best(strategy, top_strategies);
    }
    top_strategies
}

pub fn compare_strategy_to_best<T: Clone>(
    new_strategy: StrategyData<T>,
    mut best_strategies: StrategyDataStore<T>,
) -> StrategyDataStore<T> {
    if best_strategies.len() == 0 {
        best_strategies.push(new_strategy.clone());
        return best_strategies;
    }
    match mann_whitney_u_test(&new_strategy.1, &best_strategies[0].1, Confidence::P01) {
        Ordering::Equal => best_strategies.push(new_strategy.clone()),
        Ordering::Greater => {
            let mut new_best_strategies = vec![new_strategy.clone()];
            best_strategies.iter().for_each(|strategy| {
                match mann_whitney_u_test(&strategy.1, &new_strategy.1, Confidence::P01) {
                    Ordering::Equal => new_best_strategies.push(strategy.clone()),
                    Ordering::Greater => panic!("this should be impossible"),
                    Ordering::Less => (),
                }
            });
            best_strategies = new_best_strategies;
        }
        Ordering::Less => (),
    }
    best_strategies
}
