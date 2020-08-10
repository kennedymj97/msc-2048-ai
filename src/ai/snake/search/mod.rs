use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::permutations;
use super::mann_whitney::{mann_whitney_u_test, Confidence};
use super::Snake;
use crate::ai::AI;
use crate::engine::{GameEngine, Move, Score};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod brute_force;
pub mod greedy;
pub mod local;
pub mod progressive_brute_force;
pub mod random;

pub fn search(max_ban_length: usize, max_try_length: usize) {
    let engine = GameEngine::new();
    //let greedy_results = greedy::greedy(
    //    &engine,
    //    max_ban_length,
    //    max_try_length,
    //    greedy::Greedy::PrioritiseTry,
    //);
    let random_results = random::random_search(max_ban_length, max_try_length);
    local::local_search(&engine, random_results);
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

fn run_strategy_save_results(mut ai: Snake) {
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

fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

fn average(items: &Vec<u64>) -> f64 {
    items.iter().fold(0., |acc, &ele| acc + ele as f64) / items.len() as f64
}

fn find_best_fallback_set(engine: &GameEngine, snake: SnakeData) -> SnakeData {
    let mut best_snake = snake.to_owned();
    for mut fallback_set in permutations(vec![vec![Move::Left, Move::Down, Move::Up]]) {
        fallback_set.push(Move::Right);
        let mut snake_alt;
        match Snake::new(
            &snake.strategy.ban_rules,
            &snake.strategy.try_rules,
            &fallback_set,
        ) {
            Some(valid_snake) => {
                snake_alt = SnakeData {
                    strategy: valid_snake,
                    results: Vec::new(),
                }
            }
            None => continue,
        }
        let duel_results = strategy_duel(
            engine,
            &mut best_snake,
            &mut snake_alt,
            Runs {
                current: 10,
                max: 100,
            },
            Confidence::P05,
        );
        match duel_results {
            StrategyDuelResult::Champion(snake_more_results) => {
                best_snake = snake_more_results;
            }
            StrategyDuelResult::Challenger(snake_alt_results) => {
                best_snake = snake_alt_results;
            }
        }
    }
    best_snake
}

#[derive(Clone)]
pub struct SnakeData {
    strategy: Snake,
    results: Vec<Score>,
}

struct Runs {
    current: usize,
    max: usize,
}

enum StrategyDuelResult {
    Champion(SnakeData),
    Challenger(SnakeData),
}

fn strategy_duel(
    engine: &GameEngine,
    champion: &mut SnakeData,
    challenger: &mut SnakeData,
    runs: Runs,
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
            Runs {
                current: runs.current * 2,
                max: runs.max,
            },
            confidence,
        ),
        Ordering::Greater => StrategyDuelResult::Champion(champion.to_owned()),
    }
}
