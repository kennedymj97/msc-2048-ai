use super::mann_whitney::mann_whitney_u_test_01;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub fn find_best_strategies(filename: &str) {
    let path = Path::new(filename);
    let strategies = parse_data(path);
    let best_strategies = compare_strategies(strategies);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    println!("{:#?}", best_strategies_info);
}

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

type StrategyData<T> = (T, Vec<u64>);
pub type StrategyDataStore<T> = Vec<StrategyData<T>>;

pub fn compare_strategies<T: Clone>(data: StrategyDataStore<T>) -> StrategyDataStore<T> {
    let mut top_strategies = Vec::new();
    for strategy in data {
        top_strategies = compare_strategy_to_best(strategy, top_strategies);
        //if top_strategies.len() == 0 {
        //    top_strategies.push(strategy.clone());
        //    continue;
        //}
        //match mann_whitney_u_test_01(strategy.1.clone(), top_strategies[0].1.clone()) {
        //    Ordering::Equal => top_strategies.push(strategy.clone()),
        //    Ordering::Greater => {
        //        let mut new_top_strategies = vec![strategy.clone()];
        //        top_strategies.iter().for_each(|top_strategy| {
        //            match mann_whitney_u_test_01(top_strategy.1.clone(), strategy.1.clone()) {
        //                Ordering::Equal => new_top_strategies.push(top_strategy.clone()),
        //                Ordering::Greater => panic!("this should be impossible"),
        //                Ordering::Less => (),
        //            }
        //        });
        //        top_strategies = new_top_strategies;
        //    }
        //    Ordering::Less => (),
        //}
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
    match mann_whitney_u_test_01(new_strategy.1.clone(), best_strategies[0].1.clone()) {
        Ordering::Equal => best_strategies.push(new_strategy.clone()),
        Ordering::Greater => {
            let mut new_best_strategies = vec![new_strategy.clone()];
            best_strategies.iter().for_each(|strategy| {
                match mann_whitney_u_test_01(strategy.1.clone(), new_strategy.1.clone()) {
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

/// Takes a file containing strategy data in the format:
/// [STRATEGY RULES]->score1;score2;score3...
/// This function will parse the strategy data and return a vector of tuples, the first item in the
/// tuple is the strategy and the second is a vector of the scores
fn parse_data(path: &Path) -> Vec<(String, Vec<u64>)> {
    let text = fs::read_to_string(path).expect("Failed to read file");
    text.lines().fold(Vec::new(), |mut data, line| {
        let mut line_data = Vec::new();
        line.split("->").for_each(|sec| line_data.push(sec));

        let mut strategy = String::new();
        let mut scores = Vec::new();
        line_data
            .iter()
            .enumerate()
            .for_each(|(idx, &val)| match idx {
                0 => strategy = val.to_owned(),
                1 => val.split(";").for_each(|score| {
                    scores.push(score.parse::<u64>().expect("failed to parse score"))
                }),
                _ => unreachable!(),
            });
        data.push((strategy, scores));
        data
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_median() {
        assert_eq!(median(&vec![5, 1, 1, 3, 4]), 3);
        assert_eq!(median(&vec![5, 1, 2, 4, 1, 2, 3, 5, 8, 7, 6, 5]), 5);
    }
}
