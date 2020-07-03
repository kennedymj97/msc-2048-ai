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
    get_strategy_data(Snake::new(strategy, fallback_rules), 1000, "strategy5.txt");
}
