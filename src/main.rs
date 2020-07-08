use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::Rules;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::ai::*;
use msc_2048_ai::engine::Move;
use msc_2048_ai::evaluate_strategies::find_best_strategies;

fn main() {
    //let strategy_rules: Rules = vec![
    //    TryMoveIfMergePossible::new(Move::Left),
    //    TryMoveIfProducesLeftMerge::new(Move::Down),
    //    BanMoveIfLeftColumnLocked::new(Move::Up),
    //    TryMoveIfProducesLeftMerge::new(Move::Up),
    //];
    //let fallback_rules: Rules = vec![
    //    ForceMoveIfPossible::new(Move::Left),
    //    ForceMoveIfPossible::new(Move::Down),
    //    ForceMoveIfPossible::new(Move::Up),
    //    ForceMoveIfPossible::new(Move::Right),
    //];
    //////get_strategy_data(Snake::new(strategy, fallback_rules), 1000, "strategy5.txt");
    //evaluate_strategies(
    //    &strategy_rules,
    //    |strategy| Snake::new(strategy, fallback_rules.clone()),
    //    40,
    //    "data.txt",
    //);
    //
    //run_ai(Snake::new(strategy_rules, fallback_rules));
    find_best_strategies("./data.txt");
}
