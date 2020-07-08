use crate::mann_whitney::mann_whitney_u_test;
use std::cmp::Ordering;
use std::fs;

pub fn find_best_strategies(filename: &str) {
    let strategies = parse_data(filename);
    let best_strategies = compare_strategies(strategies);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    println!("{:#?}", best_strategies_info);
    println!("{}", best_strategies_info.len());
}

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

type Strategy = (String, Vec<u64>);
type Strategies = Vec<Strategy>;

fn compare_strategies(data: Strategies) -> Strategies {
    let mut top_strategies = vec![(String::from("dummy strategy"), vec![0; 20])];
    data.iter().for_each(|strategy| {
        match mann_whitney_u_test(strategy.1.clone(), top_strategies[0].1.clone()) {
            Ordering::Equal => top_strategies.push(strategy.clone()),
            Ordering::Greater => {
                let mut new_top_strategies = vec![strategy.clone()];
                top_strategies.iter().for_each(|top_strategy| {
                    match mann_whitney_u_test(top_strategy.1.clone(), strategy.1.clone()) {
                        Ordering::Equal => new_top_strategies.push(top_strategy.clone()),
                        Ordering::Greater => panic!("this should be impossible"),
                        Ordering::Less => (),
                    }
                });
                top_strategies = new_top_strategies;
            }
            Ordering::Less => (),
        }
    });
    top_strategies
}

/// Takes a file containing strategy data in the format:
/// [STRATEGY RULES]->score1;score2;score3...
/// This function will parse the strategy data and return a vector of tuples, the first item in the
/// tuple is the strategy and the second is a vector of the scores
fn parse_data(filename: &str) -> Vec<(String, Vec<u64>)> {
    let text = fs::read_to_string(filename).expect("Failed to read file");
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
