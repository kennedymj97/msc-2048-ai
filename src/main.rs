use msc_2048_ai::ai::record_ai_game;
use msc_2048_ai::ai::run_ai;
use msc_2048_ai::ai::run_ai_with_delay;
use msc_2048_ai::ai::snake::evaluate_strategies::find_best_strategies;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force_no_save;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::Move;

fn main() {
    // Ban Rules: [IfBreaksMonotonicity(Left), IfLeftColumnLocked(Up)]
    // Try Rules: [IfMovesLargestTileToCorner(Down), IfMergePossible(Left), ProducesLeftMerge(Up), ProducesLeftMerge(Down)]
    // Fallback: [Left, Down, Up, Right]: 9912
    progressive_brute_force_no_save(2, 4);
    //let ban_rules: BanRules = vec![
    //    BanMove::IfBreaksMonotonicity(Move::Left),
    //    BanMove::IfLeftColumnLocked(Move::Up),
    //];
    //let try_rules: TryRules = vec![
    //    TryMove::IfMovesLargestTileToCorner(Move::Down),
    //    TryMove::IfMergePossible(Move::Left),
    //    TryMove::ProducesLeftMerge(Move::Up),
    //    TryMove::ProducesLeftMerge(Move::Down),
    //];
    //let fallback = vec![Move::Left, Move::Down, Move::Up, Move::Right];
    //let snake = Snake::new(&ban_rules, &try_rules, &fallback);
    //run_strategy_save_results(snake);
}
