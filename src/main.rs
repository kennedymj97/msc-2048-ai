use msc_2048_ai::ai::expectimax::ExpectimaxMultithread;
use msc_2048_ai::ai::run_ai;
use msc_2048_ai::ai::snake::evaluate_strategies;
use msc_2048_ai::ai::snake::generate_strategies::generate_rule_variations;
use msc_2048_ai::ai::snake::generate_strategies::Variations;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    let generators: &[Variations] = &[
        generate_rule_variations(
            TryMoveIfProducesLeftMerge::new,
            &[Move::Up, Move::Down, Move::Left, Move::Right],
        ),
        generate_rule_variations(
            BanMoveIfLeftColumnLocked::new,
            &[Move::Up, Move::Down, Move::Left, Move::Right],
        ),
        generate_rule_variations(
            TryMoveIfMergePossible::new,
            &[Move::Up, Move::Down, Move::Left, Move::Right],
        ),
    ];

    let fallback_rules: Strategy = vec![
        ForceMoveIfPossible::new(Move::Left),
        ForceMoveIfPossible::new(Move::Down),
        ForceMoveIfPossible::new(Move::Up),
        ForceMoveIfPossible::new(Move::Right),
    ];
    progressive_brute_force(
        generators,
        |strategy| Snake::new(strategy, fallback_rules.clone()),
        4,
        "banana",
    );
}
