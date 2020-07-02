use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::Rules;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::ai::*;
use msc_2048_ai::engine::Move;

fn main() {
    let strategy: Rules = vec![
        TryMoveIfMergePossible::new(Move::Left),
        TryMoveIfProducesLeftMerge::new(Move::Down),
        BanMoveIfLeftColumnLocked::new(Move::Up),
        TryMoveIfProducesLeftMerge::new(Move::Up),
    ];
    let fallback_rules: Rules = vec![
        ForceMoveIfPossible::new(Move::Left),
        ForceMoveIfPossible::new(Move::Down),
        ForceMoveIfPossible::new(Move::Up),
        ForceMoveIfPossible::new(Move::Right),
    ];
    //run_ai(Snake::new(strategy, fallback_rules));
    evaluate_strategies(
        &strategy[..],
        |strategyy| Snake::new(strategyy, fallback_rules.clone()),
        50,
        "small.txt",
    );
}
