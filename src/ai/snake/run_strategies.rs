use super::evaluate_strategies::compare_strategies;
use super::evaluate_strategies::compare_strategy_to_best;
use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::generate_snakes;
use super::generate_strategies::get_snake_iterator;
use super::generate_strategies::get_snake_iterator_fixed_fallback;
use super::generate_strategies::permutations;
use super::generate_strategies::Iter;
use super::generate_strategies::IterFixedFallback;
use super::mann_whitney::mann_whitney_u_test_01;
use super::rules::BanMove;
use super::rules::TryMove;
use super::Snake;
use crate::ai::AI;
use crate::engine::GameEngine;
use crate::engine::Move;
use crate::engine::Score;
use std::cmp::Ordering;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone)]
struct SnakeData {
    strategy: Snake,
    results: Vec<Score>,
}

pub fn greedyy() {
    println!("Starting greedy search");
    let engine = GameEngine::new();
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
        let old_best_snake_data = best_snake_data.clone();
        // try all remaining try rules at the front
        println!("Finding best front strategy...");
        let (mut best_front_data, front_variant_idx) =
            find_best_try_front_rule(&engine, &best_snake_data, &try_variants);
        // try all remaining try rules at the back
        println!("Finding best back strategy...");
        let (mut best_back_data, back_variant_idx) =
            find_best_try_back_rule(&engine, &best_snake_data, &try_variants);
        // try all remaining ban rules
        println!("Finding best ban strategy...");
        let (mut best_ban_data, ban_variant_idx) =
            find_best_ban_rule(&engine, &best_snake_data, &ban_variants);
        // set the best rules to be the best of the three
        println!("Comparing best rules...");
        match strategy_duell(&engine, &mut best_front_data, &mut best_back_data, 10) {
            StrategyDuellResult::Champion(_) => {
                match strategy_duell(&engine, &mut best_front_data, &mut best_ban_data, 10) {
                    StrategyDuellResult::Champion(_) => {
                        best_snake_data = best_front_data;
                        if let Some(idx) = front_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuellResult::Challenger(_) => {
                        best_snake_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
            StrategyDuellResult::Challenger(_) => {
                match strategy_duell(&engine, &mut best_back_data, &mut best_ban_data, 10) {
                    StrategyDuellResult::Champion(_) => {
                        best_snake_data = best_back_data;
                        if let Some(idx) = back_variant_idx {
                            try_variants.remove(idx);
                        }
                    }
                    StrategyDuellResult::Challenger(_) => {
                        best_snake_data = best_ban_data;
                        if let Some(idx) = ban_variant_idx {
                            ban_variants.remove(idx);
                        }
                    }
                }
            }
        }
        // if there is no change in any of the sets then break
        if best_snake_data.strategy == old_best_snake_data.strategy {
            break;
        }
    }
    println!("\n\nGetting stats for best strategy...");
    run_strategy(
        &mut best_snake_data.strategy,
        &engine,
        &mut best_snake_data.results,
        100000,
    );
    let median = median(&best_snake_data.results);
    let average = average(&best_snake_data.results);
    println!(
        "Strategy: {}\nMedian: {}\nAverage: {}",
        best_snake_data.strategy, median, average
    );
}

// Will compare the current best try rule with all possible options for fallback set and will
// return the best
fn find_best_try_front_rule(
    engine: &GameEngine,
    snake_data: &SnakeData,
    try_variants: &Vec<TryMove>,
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
        challenger = find_best_fallback_set(engine, challenger);
        let duel_results = strategy_duell(engine, &mut best_snake_data, &mut challenger, 10);
        match duel_results {
            StrategyDuellResult::Champion(results) => {
                best_snake_data = results;
            }
            StrategyDuellResult::Challenger(results) => {
                best_snake_data = results;
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_snake_data, rule_added_idx)
}

fn find_best_fallback_set(engine: &GameEngine, challenger: SnakeData) -> SnakeData {
    let mut best_challenger = challenger.to_owned();
    for fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up, Move::Right]]) {
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
        let duel_results = strategy_duell(engine, &mut best_challenger, &mut challenger_alt, 10);
        match duel_results {
            StrategyDuellResult::Champion(challenger_more_results) => {
                best_challenger = challenger_more_results;
            }
            StrategyDuellResult::Challenger(challenger_alt_results) => {
                best_challenger = challenger_alt_results;
            }
        }
    }
    best_challenger
}

fn find_best_try_back_rule(
    engine: &GameEngine,
    snake_data: &SnakeData,
    try_variants: &Vec<TryMove>,
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
        challenger = find_best_fallback_set(engine, challenger);
        let duel_results = strategy_duell(engine, &mut best_snake_data, &mut challenger, 10);
        match duel_results {
            StrategyDuellResult::Champion(results) => {
                best_snake_data = results;
            }
            StrategyDuellResult::Challenger(results) => {
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
        challenger = find_best_fallback_set(engine, challenger);
        let duel_results = strategy_duell(engine, &mut best_snake_data, &mut challenger, 10);
        match duel_results {
            StrategyDuellResult::Champion(results) => {
                best_snake_data = results;
            }
            StrategyDuellResult::Challenger(results) => {
                best_snake_data = results;
                rule_added_idx = Some(idx);
            }
        }
    }
    (best_snake_data, rule_added_idx)
}

enum StrategyDuellResult {
    Champion(SnakeData),
    Challenger(SnakeData),
}

fn strategy_duell(
    engine: &GameEngine,
    champion: &mut SnakeData,
    challenger: &mut SnakeData,
    runs: usize,
) -> StrategyDuellResult {
    if runs > 50_000 {
        return StrategyDuellResult::Champion(champion.to_owned());
    }
    run_strategy(&mut champion.strategy, engine, &mut champion.results, runs);
    run_strategy(
        &mut challenger.strategy,
        engine,
        &mut challenger.results,
        runs,
    );
    match mann_whitney_u_test_01(&champion.results, &challenger.results) {
        Ordering::Less => StrategyDuellResult::Challenger(challenger.to_owned()),
        Ordering::Equal => strategy_duell(engine, champion, challenger, runs * 2),
        Ordering::Greater => StrategyDuellResult::Champion(champion.to_owned()),
    }
}

pub fn greedy_non_fixed() {
    println!("Starting greedy search...");
    let engine = GameEngine::new();
    let mut snake = Snake::new(
        &Vec::new(),
        &Vec::new(),
        &vec![Move::Left, Move::Down, Move::Up, Move::Right],
    )
    .expect("Should be a valid snake");
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        match greedy_non_fixed_add_rule(&mut snake, &engine, &mut try_variants, &mut ban_variants) {
            Some(new_snake) => snake = new_snake,
            None => break,
        }
    }
    println!("\n\nGetting stats for best strategy...");
    let mut results = Vec::new();
    run_strategy(&mut snake, &engine, &mut results, 100000);
    let median = median(&results);
    let average = average(&results);
    println!(
        "Strategy: {}\nMedian: {}\nAverage: {}",
        snake, median, average
    );
}

fn greedy_non_fixed_add_rule(
    snake: &mut Snake,
    engine: &GameEngine,
    try_variants: &mut Vec<TryMove>,
    ban_variants: &mut Vec<BanMove>,
) -> Option<Snake> {
    println!("Trying to add a try rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let mut rule_added_idx = 0;
    let mut best_try_rule = TryMove::ProducesLeftMerge(Move::Left);
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.insert(0, try_rule);
        let mut challenger;
        match Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        let mut challenger_results = Vec::new();
        for fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up, Move::Right]])
        {
            let mut challenger_alt;
            match Snake::new(&challenger.ban_rules, &challenger.try_rules, &fallback_set) {
                Some(valid_snake) => {
                    challenger_alt = valid_snake;
                }
                None => continue,
            }
            let mut challenger_alt_results = Vec::new();
            let duel_results = strategy_duel(
                engine,
                &mut challenger,
                &mut challenger_alt,
                &mut challenger_results,
                &mut challenger_alt_results,
                10,
            );
            match duel_results {
                StrategyDuelResult::Champion(results) => {
                    challenger_results = results;
                }
                StrategyDuelResult::Challenger(results) => {
                    challenger = challenger_alt;
                    challenger_results = results;
                }
            }
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_try_rule = try_rule;
            }
        }

        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.push(try_rule);
        let mut challenger;
        match Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        let mut challenger_results = Vec::new();
        for fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up, Move::Right]])
        {
            let mut challenger_alt;
            match Snake::new(&challenger.ban_rules, &challenger.try_rules, &fallback_set) {
                Some(valid_snake) => {
                    challenger_alt = valid_snake;
                }
                None => continue,
            }
            let mut challenger_alt_results = Vec::new();
            let duel_results = strategy_duel(
                engine,
                &mut challenger,
                &mut challenger_alt,
                &mut challenger_results,
                &mut challenger_alt_results,
                10,
            );
            match duel_results {
                StrategyDuelResult::Champion(results) => {
                    challenger_results = results;
                }
                StrategyDuelResult::Challenger(results) => {
                    challenger = challenger_alt;
                    challenger_results = results;
                }
            }
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_try_rule = try_rule;
            }
        }
    }
    if snake.clone() != best_snake {
        println!("{} added.", best_try_rule);
        try_variants.remove(rule_added_idx);
        return Some(best_snake);
    }

    println!("Trying to add a ban rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let mut rule_added_idx = 0;
    let mut best_ban_rule = BanMove::IfBreaksMonotonicity(Move::Right);
    for (idx, &ban_rule) in ban_variants.iter().enumerate() {
        let mut new_ban_rules = snake.ban_rules.clone();
        new_ban_rules.push(ban_rule);
        let mut challenger;
        match Snake::new(&new_ban_rules, &snake.try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        // select the challenger by choosing the best of the fallback permutations
        let mut challenger_results = Vec::new();
        for fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up, Move::Right]])
        {
            let mut challenger_alt;
            match Snake::new(&challenger.ban_rules, &challenger.try_rules, &fallback_set) {
                Some(valid_snake) => {
                    challenger_alt = valid_snake;
                }
                None => continue,
            }
            let mut challenger_alt_results = Vec::new();
            let duel_results = strategy_duel(
                engine,
                &mut challenger,
                &mut challenger_alt,
                &mut challenger_results,
                &mut challenger_alt_results,
                10,
            );
            match duel_results {
                StrategyDuelResult::Champion(results) => {
                    challenger_results = results;
                }
                StrategyDuelResult::Challenger(results) => {
                    challenger = challenger_alt;
                    challenger_results = results;
                }
            }
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_ban_rule = ban_rule;
            }
        }
    }
    if snake.clone() != best_snake {
        println!("{} added.", best_ban_rule);
        ban_variants.remove(rule_added_idx);
        return Some(best_snake);
    }

    None
}

pub fn greedy() {
    println!("Starting greedy search...");
    let engine = GameEngine::new();
    let mut snake = Snake::new(
        &Vec::new(),
        &Vec::new(),
        &vec![Move::Left, Move::Down, Move::Up, Move::Right],
    )
    .expect("Should be a valid snake");
    let mut try_variants = TryMove::generate_all_variations();
    let mut ban_variants = BanMove::generate_all_variations();
    loop {
        match greedy_add_rule(&mut snake, &engine, &mut try_variants, &mut ban_variants) {
            Some(new_snake) => snake = new_snake,
            None => break,
        }
    }
    println!("\n\nGetting stats for best strategy...");
    let mut results = Vec::new();
    run_strategy(&mut snake, &engine, &mut results, 100000);
    let median = median(&results);
    let average = average(&results);
    println!(
        "Strategy: {}\nMedian: {}\nAverage: {}",
        snake, median, average
    );
}

fn greedy_add_rule(
    snake: &mut Snake,
    engine: &GameEngine,
    try_variants: &mut Vec<TryMove>,
    ban_variants: &mut Vec<BanMove>,
) -> Option<Snake> {
    println!("Trying to add a try rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let mut rule_added_idx = 0;
    let mut best_try_rule = TryMove::ProducesLeftMerge(Move::Left);
    for (idx, &try_rule) in try_variants.iter().enumerate() {
        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.insert(0, try_rule);
        let mut challenger;
        match Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_try_rule = try_rule;
            }
        }
        // Dont bother trying to add rule on the end if the move is the same as the first fallback
        // move
        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.push(try_rule);
        let mut challenger;
        match Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_try_rule = try_rule;
            }
        }
    }
    if snake.clone() != best_snake {
        println!("{} added.", best_try_rule);
        try_variants.remove(rule_added_idx);
        return Some(best_snake);
    }

    println!("Trying to add a ban rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let mut rule_added_idx = 0;
    let mut best_ban_rule = BanMove::IfBreaksMonotonicity(Move::Right);
    for (idx, &ban_rule) in ban_variants.iter().enumerate() {
        let mut new_ban_rules = snake.ban_rules.clone();
        new_ban_rules.push(ban_rule);
        let mut challenger;
        match Snake::new(&new_ban_rules, &snake.try_rules, &snake.fallback_moves) {
            Some(valid_snake) => {
                challenger = valid_snake;
            }
            None => continue,
        }
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        match duel_results {
            StrategyDuelResult::Champion(results) => {
                best_snake_results = results;
            }
            StrategyDuelResult::Challenger(results) => {
                best_snake = challenger;
                best_snake_results = results;
                rule_added_idx = idx;
                best_ban_rule = ban_rule;
            }
        }
    }
    if snake.clone() != best_snake {
        println!("{} added.", best_ban_rule);
        ban_variants.remove(rule_added_idx);
        return Some(best_snake);
    }

    None
}

enum StrategyDuelResult {
    Champion(Vec<Score>),
    Challenger(Vec<Score>),
}

fn strategy_duel(
    engine: &GameEngine,
    champion: &mut Snake,
    challenger: &mut Snake,
    champion_results: &mut Vec<Score>,
    challenger_results: &mut Vec<Score>,
    runs: usize,
) -> StrategyDuelResult {
    if runs > 50_000 {
        return StrategyDuelResult::Champion(champion_results.to_owned());
    }
    run_strategy(champion, engine, champion_results, runs);
    run_strategy(challenger, engine, challenger_results, runs);
    match mann_whitney_u_test_01(&champion_results, &challenger_results) {
        Ordering::Less => return StrategyDuelResult::Challenger(challenger_results.to_owned()),
        Ordering::Equal => {
            return strategy_duel(
                engine,
                champion,
                challenger,
                champion_results,
                challenger_results,
                runs * 2,
            )
        }
        Ordering::Greater => return StrategyDuelResult::Champion(champion_results.to_owned()),
    }
}

pub fn progressive_brute_force_no_save(max_ban_length: usize, max_try_length: usize) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Getting snakes iterator");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let best_initial_strategies = progressive_brute_force_initial_run(snakes_iter, &engine, 10);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, &engine, 10 * 5);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn progressive_brute_force_initial_run(
    snake_iter: Iter,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Snake> {
    println!("Initial @ {} runs", runs);
    let mut count = 0;
    let total_count = snake_iter.len();
    let mut best_strategies = Vec::new();
    for mut snake in snake_iter {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut results = Vec::new();
        run_strategy(&mut snake, engine, &mut results, runs);
        best_strategies = compare_strategy_to_best((snake, results), best_strategies);
    }
    best_strategies
}

pub fn progressive_brute_force_no_save_fixed_fallback(
    max_ban_length: usize,
    max_try_length: usize,
) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Getting snakes iterator");
    let snakes_iter = get_snake_iterator_fixed_fallback(max_ban_length, max_try_length);
    let best_initial_strategies =
        progressive_brute_force_fixed_fallback_initial_run(snakes_iter, &engine, 5);
    let best_strategies =
        progressive_brute_force_no_save_aux(best_initial_strategies, &engine, 5 * 10);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn progressive_brute_force_fixed_fallback_initial_run(
    snake_iter: IterFixedFallback,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Snake> {
    println!("Initial @ {} runs", runs);
    let mut count = 0;
    let total_count = snake_iter.len();
    let mut best_strategies = Vec::new();
    for mut snake in snake_iter {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        let mut results = Vec::new();
        run_strategy(&mut snake, engine, &mut results, runs);
        best_strategies = compare_strategy_to_best((snake, results), best_strategies);
    }
    best_strategies
}
fn progressive_brute_force_no_save_aux(
    current_best: StrategyDataStore<Snake>,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Snake> {
    if runs >= 100000 {
        return current_best;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = current_best.len();
    let best = current_best
        .into_iter()
        .map(|(mut snake, mut results)| {
            count += 1;
            if count % get_count_mod(total_count) == 0 {
                println!("{}/{}", count, total_count);
            }
            run_strategy(&mut snake, engine, &mut results, runs);
            (snake, results)
        })
        .collect::<StrategyDataStore<Snake>>();
    progressive_brute_force_no_save_aux(compare_strategies(best), engine, runs * 5)
}

pub fn progressive_brute_force(max_ban_length: usize, max_try_length: usize, foldername: &str) {
    println!("engine...");
    let engine = GameEngine::new();
    let path = Path::new(foldername);
    let dir_builder = DirBuilder::new();
    dir_builder.create(path).expect("Failed to create folder");
    println!("generating snakes");
    let snakes = generate_snakes(max_ban_length, max_try_length);
    println!("creating data");
    let data = snakes
        .into_iter()
        .map(|snake| (snake, Vec::new()))
        .collect::<StrategyDataStore<Snake>>();
    println!("calling aux function");
    let best_strategies = progressive_brute_force_aux(data, &engine, 10, path);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

fn average(items: &Vec<u64>) -> f64 {
    items.iter().fold(0., |acc, &ele| acc + ele as f64) / items.len() as f64
}

fn progressive_brute_force_aux(
    data: StrategyDataStore<Snake>,
    engine: &GameEngine,
    runs: usize,
    foldername: &Path,
) -> StrategyDataStore<Snake> {
    if runs > 1000 {
        return data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = data.len();
    let data = data
        .into_iter()
        .map(|(mut snake, mut results)| {
            count += 1;
            println!("{}/{}", count, total_count);
            run_strategy(&mut snake, engine, &mut results, runs);
            (snake, results)
        })
        .collect::<StrategyDataStore<Snake>>();
    save_results(&data, foldername, runs);
    progressive_brute_force_aux(compare_strategies(data), engine, runs * 10, foldername)
}

fn save_results(data: &StrategyDataStore<Snake>, foldername: &Path, runs: usize) {
    println!("Saving data @ {} runs...", runs);
    let path = foldername.join(format!("{}_runs.csv", runs));
    let mut f = File::create(path).expect("Failed to create file");
    data.iter().for_each(|(snake, results)| {
        f.write_fmt(format_args!("{},", snake))
            .expect("Failed to write strategy information to file");
        let mut results_iter = results.iter().peekable();
        while let Some(score) = results_iter.next() {
            f.write_fmt(format_args!("{}", score))
                .expect("Failed to write score");
            if results_iter.peek().is_some() {
                f.write(",".as_bytes()).expect("Failed to write comma");
            }
        }
        f.write("\n".as_bytes()).expect("Failed to write new line");
    });
}

pub fn brute_force(max_ban_length: usize, max_try_length: usize, runs: usize, filename: &str) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Generating snakes...");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    let mut count = 0;
    let total_count = snakes_iter.len();
    println!("Getting data...");
    f.write("Strategy,".as_bytes())
        .expect("Failed to write header");
    (1..runs).for_each(|run_idx| {
        f.write_fmt(format_args!("Run {},", run_idx))
            .expect("Failed to write header")
    });
    f.write_fmt(format_args!("Run {}", runs))
        .expect("Failed to write header");
    snakes_iter.for_each(|mut snake| {
        count += 1;
        if count % get_count_mod(total_count) == 0 {
            println!("{}/{}", count, total_count);
        }
        f.write_fmt(format_args!("{},", snake))
            .expect("Failed to write strategy information to file");
        let mut results = Vec::new();
        run_strategy(&mut snake, &engine, &mut results, runs);
        let mut results_iter = results.iter().peekable();
        while let Some(score) = results_iter.next() {
            f.write_fmt(format_args!("{}", score))
                .expect("failed to write final score");
            if results_iter.peek().is_some() {
                f.write(",".as_bytes()).expect("Failed to write comma");
            }
        }
        f.write("\n".as_bytes()).expect("Failed to write new line");
    });
}

fn run_strategy<T: AI>(
    ai: &mut T,
    engine: &GameEngine,
    current_results: &mut Vec<Score>,
    runs: usize,
) {
    let mut current_runs = current_results.len();
    while current_runs < runs {
        let mut board = GameEngine::new_board();
        loop {
            let best_move = ai.get_next_move(engine, board);
            match best_move {
                Some(direction) => {
                    board = engine.make_move(board, direction);
                }
                None => break,
            }
        }
        current_results.push(engine.get_score(board));
        current_runs += 1;
    }
}

pub fn run_strategy_save_results(mut ai: Snake) {
    let engine = GameEngine::new();
    let mut f = File::create(Path::new("strategy.csv")).expect("Failed to create file");
    f.write("score,highest tile\n".as_bytes())
        .expect("Failed to write strategy");
    (0..10000).for_each(|_| {
        let mut board = GameEngine::new_board();
        loop {
            let best_move = ai.get_next_move(&engine, board);
            match best_move {
                Some(direction) => {
                    board = engine.make_move(board, direction);
                }
                None => break,
            }
        }
        let score = engine.get_score(board);
        let highest_tile = GameEngine::get_highest_tile_val(board);
        f.write_fmt(format_args!("{},{}\n", score, highest_tile))
            .expect("failed to write data to file");
    });
}

fn get_count_mod(len: usize) -> u64 {
    if len < 100 {
        return 1;
    }

    if len < 1000 {
        return 10;
    }

    if len < 10_000 {
        return 100;
    }

    if len < 100_000 {
        return 1000;
    }

    if len < 1_000_000 {
        return 10000;
    }

    100000
}
