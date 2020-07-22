use super::evaluate_strategies::compare_strategies;
use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::generate_snakes;
use super::Snake;
use crate::ai::AI;
use crate::engine::GameEngine;
use crate::engine::Score;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn progressive_brute_force_no_save(max_ban_length: usize, max_try_length: usize) {
    println!("Creating engine...");
    let engine = GameEngine::new();
    println!("Generating snakes...");
    let snakes = generate_snakes(max_ban_length, max_try_length);
    println!("Creating datastructure...");
    let data = snakes
        .iter()
        .map(|snake| (snake.clone(), Vec::new()))
        .collect::<StrategyDataStore<Box<Snake>>>();
    let best_strategies = progressive_brute_force_no_save_aux(data, &engine, 10);
    let best_strategies_info = best_strategies
        .iter()
        .map(|(strategy_info, scores)| (strategy_info, median(scores)))
        .collect::<Vec<_>>();
    best_strategies_info
        .iter()
        .for_each(|(snake, median)| println!("{}: {}", snake, median));
}

fn progressive_brute_force_no_save_aux(
    data: StrategyDataStore<Box<Snake>>,
    engine: &GameEngine,
    runs: usize,
) -> StrategyDataStore<Box<Snake>> {
    if runs > 1000 {
        return data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = data.len();
    let data = data
        .iter()
        .map(|(snake, results)| {
            count += 1;
            if count % 1000 == 0 {
                println!("{}/{}", count, total_count);
            }
            (
                snake.clone(),
                run_strategy(snake.clone(), engine, results.clone(), runs),
            )
        })
        .collect::<StrategyDataStore<Box<Snake>>>();
    progressive_brute_force_no_save_aux(compare_strategies(data), engine, runs * 10)
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
        .iter()
        .map(|snake| (snake.clone(), Vec::new()))
        .collect::<StrategyDataStore<Box<Snake>>>();
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

fn progressive_brute_force_aux(
    data: StrategyDataStore<Box<Snake>>,
    engine: &GameEngine,
    runs: usize,
    foldername: &Path,
) -> StrategyDataStore<Box<Snake>> {
    if runs > 1000 {
        return data;
    }
    println!("@ {} runs...", runs);
    let mut count = 0;
    let total_count = data.len();
    let data = data
        .iter()
        .map(|(snake, results)| {
            count += 1;
            println!("{}/{}", count, total_count);
            (
                snake.clone(),
                run_strategy(snake.clone(), engine, results.clone(), runs),
            )
        })
        .collect::<StrategyDataStore<Box<Snake>>>();
    save_results(&data, foldername, runs);
    progressive_brute_force_aux(compare_strategies(data), engine, runs * 10, foldername)
}

fn save_results(data: &StrategyDataStore<Box<Snake>>, foldername: &Path, runs: usize) {
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
    snakes.iter().for_each(|snake| {
        count += 1;
        println!("{}/{}", count, total_count);
        f.write_fmt(format_args!("{},", snake))
            .expect("Failed to write strategy information to file");
        let results = run_strategy(snake.clone(), &engine, Vec::new(), runs);
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

fn run_strategy(
    mut ai: Box<dyn AI>,
    engine: &GameEngine,
    mut current_results: Vec<u64>,
    runs: usize,
) -> Vec<Score> {
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
    current_results
}

//use super::Snake;
//
//pub fn run_strategy_save_results(strategy: &Strategy) {
//    let engine = GameEngine::new();
//    let fallback = vec![
//        ForceMoveIfPossible::new(Move::Left),
//        ForceMoveIfPossible::new(Move::Down),
//        ForceMoveIfPossible::new(Move::Up),
//        ForceMoveIfPossible::new(Move::Right),
//    ];
//    let mut ai = Snake::new(strategy, &fallback);
//    let mut f = File::create(Path::new(&format!("{}.csv", strategy_to_str(strategy))))
//        .expect("Failed to create file");
//    f.write("score,highest tile\n".as_bytes())
//        .expect("Failed to write strategy");
//    (0..10000).for_each(|_| {
//        let mut board = GameEngine::new_board();
//        loop {
//            let best_move = ai.get_next_move(&engine, board);
//            match best_move {
//                Some(direction) => {
//                    board = engine.make_move(board, direction);
//                }
//                None => break,
//            }
//        }
//        let score = engine.get_score(board);
//        let highest_tile = GameEngine::get_highest_tile_val(board);
//        f.write_fmt(format_args!("{},{}\n", score, highest_tile))
//            .expect("failed to write data to file");
//    });
//}
