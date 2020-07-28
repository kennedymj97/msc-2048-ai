use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::greedy;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    //greedy();
    let ban_rules = vec![
        BanMove::IfLeftColumnLocked(Move::Up),
        BanMove::IfBreaksMonotonicity(Move::Left),
    ];
    let try_rules = vec![
        TryMove::IfMovesLargestTileToCorner(Move::Down),
        TryMove::IfMergePossible(Move::Left),
        TryMove::ProducesLeftMerge(Move::Up),
        TryMove::ProducesLeftMerge(Move::Down),
        TryMove::IfMergePossible(Move::Up),
    ];
    let fallback_moves = vec![Move::Left, Move::Down, Move::Up, Move::Right];
    let snake = Snake::new(&ban_rules, &try_rules, &fallback_moves);
    run_strategy_save_results(snake);
}
