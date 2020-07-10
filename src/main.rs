use msc_2048_ai::ai::snake::evaluate_strategies;
use msc_2048_ai::ai::snake::generate_strategies::generate_rule_variations;
use msc_2048_ai::ai::snake::generate_strategies::Variations;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    let generators: &[Variations] = &[
        generate_rule_variations(TryMoveIfProducesLeftMerge::new, &[Move::Up, Move::Down]),
        generate_rule_variations(BanMoveIfLeftColumnLocked::new, &[Move::Up]),
        generate_rule_variations(TryMoveIfMergePossible::new, &[Move::Left]),
    ];

    let fallback_rules: Rules = vec![
        ForceMoveIfPossible::new(Move::Left),
        ForceMoveIfPossible::new(Move::Down),
        ForceMoveIfPossible::new(Move::Up),
        ForceMoveIfPossible::new(Move::Right),
    ];
    run_strategies::run_strategies_save_results(
        generators,
        |strategy| Snake::new(strategy, fallback_rules.clone()),
        1000,
        "./data.txt",
    );
    evaluate_strategies::find_best_strategies("./data.txt");
}
