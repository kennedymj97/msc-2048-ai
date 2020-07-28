use super::evaluate_strategies::compare_strategies;
use super::evaluate_strategies::compare_strategy_to_best;
use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::generate_snakes;
use super::generate_strategies::get_snake_iterator;
use super::generate_strategies::Iter;
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

pub fn greedy() {
    println!("Starting greedy search...");
    let engine = GameEngine::new();
    let mut snake = Snake::new(
        &Vec::new(),
        &Vec::new(),
        &vec![Move::Left, Move::Down, Move::Up, Move::Right],
    );
    loop {
        match greedy_add_rule(&mut snake, &engine) {
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

fn greedy_add_rule(snake: &mut Snake, engine: &GameEngine) -> Option<Snake> {
    println!("Adding a try rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let try_variants = TryMove::generate_all_variations();
    let mut count = 0;
    for try_rule in try_variants {
        count += 1;
        println!("Trying rule #{}...", count);
        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.insert(0, try_rule);
        let mut challenger = Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves);
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        best_snake = duel_results.0;
        best_snake_results = duel_results.1;
        let mut new_try_rules = snake.try_rules.clone();
        new_try_rules.push(try_rule);
        let mut challenger = Snake::new(&snake.ban_rules, &new_try_rules, &snake.fallback_moves);
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        best_snake = duel_results.0;
        best_snake_results = duel_results.1;
    }
    if snake.clone() != best_snake {
        return Some(best_snake);
    }

    println!("Adding a ban rule...");
    let mut best_snake = snake.clone();
    let mut best_snake_results = Vec::new();
    let ban_variants = BanMove::generate_all_variations();
    for ban_rule in ban_variants {
        let mut new_ban_rules = snake.ban_rules.clone();
        new_ban_rules.push(ban_rule);
        let mut challenger = Snake::new(&new_ban_rules, &snake.try_rules, &snake.fallback_moves);
        let mut challenger_results = Vec::new();
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut challenger,
            &mut best_snake_results,
            &mut challenger_results,
            10,
        );
        best_snake = duel_results.0;
        best_snake_results = duel_results.1;
    }
    if snake.clone() != best_snake {
        return Some(best_snake);
    }

    None
}

fn strategy_duel(
    engine: &GameEngine,
    champion: &mut Snake,
    challenger: &mut Snake,
    champion_results: &mut Vec<Score>,
    challenger_results: &mut Vec<Score>,
    runs: usize,
) -> (Snake, Vec<Score>) {
    println!("Champion: {}\nChallenger: {}", champion, challenger);
    if runs > 40_000 {
        println!("No winner!");
        return (champion.clone(), champion_results.to_owned());
    }
    println!("Dueling at {} runs...", runs);
    run_strategy(champion, engine, champion_results, runs);
    run_strategy(challenger, engine, challenger_results, runs);
    match mann_whitney_u_test_01(&champion_results, &challenger_results) {
        Ordering::Less => return (challenger.to_owned(), challenger_results.to_owned()),
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
        Ordering::Greater => return (champion.to_owned(), champion_results.to_owned()),
    }
}

pub fn progressive_brute_force_no_save(max_ban_length: usize, max_try_length: usize) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Getting snakes iterator");
    let snakes_iter = get_snake_iterator(max_ban_length, max_try_length);
    let best_initial_strategies = progressive_brute_force_initial_run(snakes_iter, &engine, 5);
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
    progressive_brute_force_no_save_aux(compare_strategies(best), engine, runs * 10)
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
    let engine = GameEngine::new();
    let snakes = generate_snakes(max_ban_length, max_try_length);
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    let mut count = 0;
    let total_count = snakes.len();
    snakes.into_iter().for_each(|mut snake| {
        count += 1;
        println!("{}/{}", count, total_count);
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
    while current_runs <= runs {
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
