use super::get_count_mod;
use super::run_strategy;
use crate::ai::snake::generate_strategies::get_snake_iterator;
use crate::engine::GameEngineStores;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn brute_force(max_ban_length: usize, max_try_length: usize, runs: usize, filename: &str) {
    println!("Creating engine...");
    let engine = GameEngineStores::new();
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
