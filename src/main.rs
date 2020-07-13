use msc_2048_ai::ai::snake::generate_strategies::generate_rule_variations;
use msc_2048_ai::ai::snake::generate_strategies::Variations;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    //let generators: &[Variations] = &[
    //    generate_rule_variations(
    //        TryMoveIfProducesLeftMerge::new,
    //        &[Move::Up, Move::Down, Move::Left, Move::Right],
    //    ),
    //    generate_rule_variations(
    //        BanMoveIfLeftColumnLocked::new,
    //        &[Move::Up, Move::Down, Move::Left, Move::Right],
    //    ),
    //    generate_rule_variations(
    //        TryMoveIfMergePossible::new,
    //        &[Move::Up, Move::Down, Move::Left, Move::Right],
    //    ),
    //];

    //let fallback_rules: Strategy = vec![
    //    ForceMoveIfPossible::new(Move::Left),
    //    ForceMoveIfPossible::new(Move::Down),
    //    ForceMoveIfPossible::new(Move::Up),
    //    ForceMoveIfPossible::new(Move::Right),
    //];
    //progressive_brute_force(
    //    generators,
    //    |&strategy| Snake::new(strategy, &fallback_rules),
    //    4,
    //    "banana",
    //);
    let strategy: Strategy = vec![
        TryMoveIfMergePossible::new(Move::Left),
        TryMoveIfProducesLeftMerge::new(Move::Down),
        BanMoveIfLeftColumnLocked::new(Move::Up),
        TryMoveIfProducesLeftMerge::new(Move::Up),
    ];
    run_strategy_save_results(&strategy);
}
