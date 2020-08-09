#![allow(unused_imports)]
use msc_2048_ai::ai::snake::attributes::*;
use msc_2048_ai::ai::snake::ban_rules::*;
use msc_2048_ai::ai::snake::mann_whitney::Confidence;
use msc_2048_ai::ai::snake::search::brute_force;
use msc_2048_ai::ai::snake::search::greedy;
use msc_2048_ai::ai::snake::search::greedy::greedy;
use msc_2048_ai::ai::snake::search::greedy::Greedy;
use msc_2048_ai::ai::snake::search::progressive_brute_force::progressive_brute_force_no_save;
use msc_2048_ai::ai::snake::search::progressive_brute_force::progressive_brute_force_no_save_fixed_fallback;
use msc_2048_ai::ai::snake::search::run_strategy_save_results;
use msc_2048_ai::ai::snake::try_rules::*;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    //brute_force(2, 4, 100, "./data/brute_force_2try_4ban_100runs.csv");
    //progressive_brute_force_no_save_fixed_fallback(2, 5);
    //progressive_brute_force_no_save(2, 3);
    greedy(Greedy::PrioritiseTry, Confidence::P05, 1000);
    //let ban_rules = vec![
    //    BanMove::IfColumnNotLocked(Move::Up, Column::Left),
    //    BanMove::MovesLargestTileOutOfCorner(Move::Down, Corner::TopRight),
    //    BanMove::MovesLargestTileOutOfCorner(Move::Up, Corner::BottomLeft),
    //];
    //let try_rules = vec![
    //    TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft),
    //    TryMove::CreatesMonotonicColumn(Move::Left, Column::Left),
    //    TryMove::ProducesMerge(Move::Up),
    //    TryMove::ProducesMerge(Move::Down),
    //    TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop),
    //];
    //let fallback_moves = vec![Move::Left, Move::Up, Move::Down, Move::Right];
    //let snake =
    //    Snake::new(&ban_rules, &try_rules, &fallback_moves).expect("Failed to create snake");
    //run_strategy_save_results(snake);
    //run_ai_with_delay(&mut snake, 1000);
}
