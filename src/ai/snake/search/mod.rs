use super::evaluate_strategies::StrategyDataStore;
use super::generate_strategies::permutations;
use super::mann_whitney::{mann_whitney_u_test, Confidence};
use super::Snake;
use crate::ai::AI;
use crate::engine::{
    get_highest_tile_val, new_board, Board, GameEngine, GameEngineStores, Move, Score,
};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

pub mod brute_force;
pub mod greedy;
pub mod iterated_local;
pub mod local;
pub mod progressive_brute_force;
pub mod random;

pub fn test_chosen_length(filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(path).expect("Failed to create file");
    file.write("iteration,strategy,median_score,average_score,search_time\n".as_bytes())
        .expect("failed to write headers to file");
    let engine = GameEngineStores::new();
    let mut count = 0;
    loop {
        count += 1;
        let start_time = SystemTime::now();
        let result = search(&engine, 1, 4);
        let time_elapsed = match start_time.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(e) => panic!(e),
        };
        file.write_fmt(format_args!(
            "{},{},{},{},{}\n",
            count,
            result.strategy,
            median(&result.results),
            average(&result.results),
            time_elapsed
        ))
        .expect("Failed to save search data");
    }
}

pub fn test_different_lengths(filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(path).expect("Failed to create file");
    file.write(
        "ban_length,try_length,strategy,median_score,average_score,search_time\n".as_bytes(),
    )
    .expect("failed to write headers to file");
    let engine = GameEngineStores::new();
    for ban_length in 0..4 {
        for try_length in 0..6 {
            let start_time = SystemTime::now();
            let result = search(&engine, ban_length, try_length);
            let time_elapsed = match start_time.elapsed() {
                Ok(elapsed) => elapsed.as_millis(),
                Err(e) => panic!(e),
            };
            file.write_fmt(format_args!(
                "{},{},{},{},{},{}\n",
                ban_length,
                try_length,
                result.strategy,
                median(&result.results),
                average(&result.results),
                time_elapsed
            ))
            .expect("Failed to save search data");
        }
    }
}

pub fn search<T: GameEngine>(
    engine: &T,
    max_ban_length: usize,
    max_try_length: usize,
) -> SnakeData {
    let greedy_results = greedy::greedy_prioritise_best(engine, max_ban_length, max_try_length);
    iterated_local::ils_mutate_try_accept_if_better(engine, greedy_results)
}

pub fn test_search_method(
    f: fn(&GameEngineStores, usize, usize) -> SnakeData,
    filename: &str,
    search_repeats: usize,
) {
    let engine = GameEngineStores::new();
    println!("Testing search method...");
    // create csv file to save the results
    let path = Path::new(filename);
    let mut file = File::create(path).expect("Failed to create file");
    // add the column headers to the csv file
    file.write("strategy,median_score,average_score,time\n".as_bytes())
        .expect("failed to write headers to file");
    // repeat the search procecure *search repeats* times
    let mut medians = Vec::new();
    let mut times = Vec::new();
    for search_num in 0..search_repeats {
        println!(
            "\nCarrying out search {}/{}\n",
            search_num + 1,
            search_repeats
        );
        let start_time = SystemTime::now();
        let mut search_result = f(&engine, 1, 4);
        run_strategy(
            &mut search_result.strategy,
            &engine,
            &mut search_result.results,
            50000,
        );
        let time_elapsed = match start_time.elapsed() {
            Ok(elapsed) => elapsed.as_millis(),
            Err(e) => panic!(e),
        };
        times.push(time_elapsed as u64);
        let median = median(&search_result.results);
        let average = average(&search_result.results);
        medians.push(median);
        // save the resulting median and time taken for the search
        file.write_fmt(format_args!(
            "{},{},{},{}\n",
            search_result.strategy, median, average, time_elapsed
        ))
        .expect("Failed to save search data");
    }
    println!(
        "\nAverage median score of the search method: {}\nAverage time taken for the search: {}\n",
        average(&medians),
        average(&times)
    );
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

pub fn run_strategy<T: AI, E: GameEngine>(
    ai: &mut T,
    engine: &E,
    current_results: &mut Vec<Score>,
    runs: usize,
) {
    let mut current_runs = current_results.len();
    while current_runs < runs {
        let mut board = new_board();
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

pub fn run_strategy_save_results(mut ai: Snake, filename: &str) {
    let engine = GameEngineStores::new();
    let mut f = File::create(Path::new(filename)).expect("Failed to create file");
    f.write("score,highest tile\n".as_bytes())
        .expect("Failed to write strategy");
    (0..100000).for_each(|_| {
        let mut board = new_board();
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
        let highest_tile = get_highest_tile_val(board);
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

pub fn print_best_strategy_info<T: GameEngine>(engine: &T, strategy_data: &mut SnakeData) {
    println!("\n\nGetting stats for best strategy_data...");
    run_strategy(
        &mut strategy_data.strategy,
        engine,
        &mut strategy_data.results,
        10000,
    );
    let median = median(&strategy_data.results);
    let average = average(&strategy_data.results);
    println!(
        "Strategy: {}\nMedian: {}\nAverage: {}",
        strategy_data.strategy, median, average
    );
}

pub fn median<T: Ord + Copy>(items: &Vec<T>) -> T {
    let mut items = items.clone();
    items.sort();
    items[items.len() / 2]
}

fn average(items: &Vec<u64>) -> f64 {
    items.iter().fold(0., |acc, &ele| acc + ele as f64) / items.len() as f64
}

fn find_best_fallback_set<T: GameEngine>(engine: &T, snake: SnakeData) -> SnakeData {
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

fn strategy_duel<T: GameEngine>(
    engine: &T,
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

use super::attributes::{is_move_possible, Column, Corner, Row};
use super::ban_rules::BanMove;
use super::try_rules::TryMove;
use crate::engine::GameEngineNoStores;
use std::io::{self, BufRead};

pub fn add_used_rule_to_data(filename: &str) {
    let mut new_fname = filename.split('.').collect::<Vec<&str>>()[0].to_string();
    new_fname.push_str("_adapted.csv");
    let mut f = File::create(Path::new(&new_fname)).expect("Failed to create file");
    if let Ok(lines) = read_lines(Path::new(&filename)) {
        for line in lines {
            if let Ok(content) = line {
                let board = content.split(',').collect::<Vec<&str>>()[2];
                let board = board.replace("\"", "");
                match u64::from_str_radix(&board, 16) {
                    Ok(int_board) => {
                        let mut new_content = content.clone();
                        new_content.push_str(&format!(",{}\n", find_used_rule(int_board)));
                        f.write(new_content.as_bytes())
                            .expect("Failed to write new row");
                    }
                    Err(_) => {
                        let mut new_content = content.clone();
                        new_content.push_str(",action\n");
                        f.write(new_content.as_bytes())
                            .expect("failed to write new header");
                    }
                }
            }
        }
    }
}

fn find_used_rule(board: Board) -> String {
    let engine = GameEngineNoStores;
    let is_up_banned =
        match BanMove::IfColumnNotLocked(Move::Up, Column::Left).execute(&engine, board) {
            Some(_) => true,
            None => false,
        };
    match TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft)
        .execute(&GameEngineNoStores, board)
    {
        Some(_) => {
            return TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft).to_string()
        }
        None => (),
    }
    if !is_up_banned {
        match TryMove::ProducesMerge(Move::Up).execute(&engine, board) {
            Some(_) => return TryMove::ProducesMerge(Move::Up).to_string(),
            None => (),
        }
    }
    match TryMove::ProducesMerge(Move::Down).execute(&engine, board) {
        Some(_) => return TryMove::ProducesMerge(Move::Down).to_string(),
        None => (),
    }
    match TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop).execute(&engine, board) {
        Some(_) => return TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop).to_string(),
        None => (),
    }
    if is_move_possible(&engine, board, Move::Left) {
        return "initial fallback left".to_string();
    }
    if is_move_possible(&engine, board, Move::Down) {
        return "initial fallback down".to_string();
    }
    if is_move_possible(&engine, board, Move::Up) && !is_up_banned {
        return "initial fallback up".to_string();
    }
    if is_move_possible(&engine, board, Move::Right) {
        return "initial fallback right".to_string();
    }
    if is_move_possible(&engine, board, Move::Up) {
        return "up forced in fallback".to_string();
    }
    return "no possible move".to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
