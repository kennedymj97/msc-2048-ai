#![allow(unused_imports)]
use msc_2048_ai::ai::run_ai_with_delay;
use msc_2048_ai::ai::strategy::search::search;
use msc_2048_ai::ai::strategy::{
    attributes::Column, attributes::Corner, attributes::Row, ban_rules::BanMove,
    try_rules::TryMove, Strategy,
};
use msc_2048_ai::engine::{GameEngineNoStores, GameEngineStores, Move};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "--play" => {
                    let ban_rules = vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)];
                    let try_rules = vec![
                        TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft),
                        TryMove::ProducesMerge(Move::Up),
                        TryMove::ProducesMerge(Move::Down),
                        TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop),
                    ];
                    let fallback = vec![Move::Left, Move::Up, Move::Down, Move::Right];
                    let mut strategy = Strategy::new(&ban_rules, &try_rules, &fallback).unwrap();
                    run_ai_with_delay(&mut strategy, 500);
                }
                _ => {
                    let engine = GameEngineStores::new();
                    search(&engine, 1, 4);
                }
            }
        }
        3 => {
            let cmd = &args[1];
            let move_rate = &args[2];
            match &cmd[..] {
                "--play" => match &move_rate[..].parse::<u64>() {
                    Ok(move_rate_int) => {
                        let ban_rules = vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)];
                        let try_rules = vec![
                            TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft),
                            TryMove::ProducesMerge(Move::Up),
                            TryMove::ProducesMerge(Move::Down),
                            TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop),
                        ];
                        let fallback = vec![Move::Left, Move::Up, Move::Down, Move::Right];
                        let mut strategy =
                            Strategy::new(&ban_rules, &try_rules, &fallback).unwrap();
                        run_ai_with_delay(&mut strategy, *move_rate_int);
                    }
                    Err(err) => {
                        eprintln!("Invalid time: {}", err);
                    }
                },
                _ => {
                    let engine = GameEngineStores::new();
                    search(&engine, 1, 4);
                }
            }
        }
        _ => {
            let engine = GameEngineStores::new();
            search(&engine, 1, 4);
        }
    }
}
