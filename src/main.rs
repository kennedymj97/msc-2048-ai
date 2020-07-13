use msc_2048_ai::ai::record_ai_game;
use msc_2048_ai::ai::snake::generate_strategies::generate_rule_variations;
use msc_2048_ai::ai::snake::generate_strategies::Variations;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::Move;

fn main() {
    let strategy: Strategy = vec![
        TryMoveIfMergePossible::new(Move::Left),
        TryMoveIfProducesLeftMerge::new(Move::Down),
        BanMoveIfLeftColumnLocked::new(Move::Up),
        TryMoveIfProducesLeftMerge::new(Move::Up),
    ];
    //let fallback_rules: Strategy = vec![
    //    ForceMoveIfPossible::new(Move::Left),
    //    ForceMoveIfPossible::new(Move::Down),
    //    ForceMoveIfPossible::new(Move::Up),
    //    ForceMoveIfPossible::new(Move::Right),
    //];
    //let snake = Snake::new(&strategy, &fallback_rules);
    //record_ai_game(snake, "bad2.txt");
    run_strategy_save_results(&strategy);
}
