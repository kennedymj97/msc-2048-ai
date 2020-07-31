use msc_2048_ai::ai::run_ai_with_delay;
use msc_2048_ai::ai::snake::mann_whitney::Confidence;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::brute_force;
use msc_2048_ai::ai::snake::run_strategies::greedy;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force_no_save;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force_no_save_fixed_fallback;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::run_strategies::Greedy;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    //brute_force(2, 4, 100, "./data/brute_force_2try_4ban_100runs.csv");
    //progressive_brute_force_no_save_fixed_fallback(2, 5);
    //progressive_brute_force_no_save(2, 4);
    greedy(Greedy::PrioritiseBest, Confidence::P05, 1000);
    //let ban_rules = vec![
    //    BanMove::IfLeftColumnLocked(Move::Up),
    //    BanMove::IfBreaksMonotonicity(Move::Left),
    //];
    //let try_rules = vec![
    //    TryMove::IfMovesLargestTileToCorner(Move::Down),
    //    TryMove::IfMergePossible(Move::Left),
    //    TryMove::ProducesLeftMerge(Move::Down),
    //    TryMove::ProducesLeftMerge(Move::Up),
    //    TryMove::IfMergePossible(Move::Up),
    //];
    //let fallback_moves = vec![Move::Left, Move::Down, Move::Up, Move::Right];
    //let mut snake =
    //    Snake::new(&ban_rules, &try_rules, &fallback_moves).expect("Failed to create snake");
    //run_strategy_save_results(snake);
    //run_ai_with_delay(&mut snake, 1000);
}
