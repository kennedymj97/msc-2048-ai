use msc_2048_ai::ai::snake::attributes::*;
use msc_2048_ai::ai::snake::ban_rules::*;
use msc_2048_ai::ai::snake::generate_strategies::factorial;
use msc_2048_ai::ai::snake::mann_whitney::Confidence;
use msc_2048_ai::ai::snake::run_strategies::brute_force;
use msc_2048_ai::ai::snake::run_strategies::greedy;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force_no_save;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force_no_save_fixed_fallback;
use msc_2048_ai::ai::snake::run_strategies::run_strategy_save_results;
use msc_2048_ai::ai::snake::run_strategies::Greedy;
use msc_2048_ai::ai::snake::try_rules::*;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::engine::Move;

fn main() {
    //brute_force(2, 4, 100, "./data/brute_force_2try_4ban_100runs.csv");
    //progressive_brute_force_no_save_fixed_fallback(2, 5);
    //progressive_brute_force_no_save(2, 3);
    let num = 68;
    println!("factorial({}): {}", num, factorial(num));
    greedy(Greedy::PrioritiseBest, Confidence::P01, 1000);
    //Strategy: Ban Rules: ban move Up if left column locked->ban move Right if left column locked->ban move Left if breaks monotonicity of left column       Try Rules: try move Left if moves largest tile to corner->try move Down if moves largest tile to corner->try move Left if merge possible->try move Up if produces left merge->try move Down if produces left merge->try move Up if merge possible        Fall-back: Left->Up->Down->Right
    //let ban_rules = vec![
    //    BanMove::IfColumnNotLocked(Move::Up, Column::Left),
    //    BanMove::IfColumnNotLocked(Move::Right, Column::Left),
    //    BanMove::IfBreaksMonotonicityOfColumn(Move::Left, Column::Left),
    //];
    //let try_rules = vec![
    //    TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft),
    //    TryMove::IfMovesLargestTileToCorner(Move::Down, Corner::BottomLeft),
    //    TryMove::IfMergePossible(Move::Left),
    //    TryMove::ProducesMerge(Move::Up),
    //    TryMove::ProducesMerge(Move::Down),
    //    TryMove::IfMergePossible(Move::Up),
    //];
    //let fallback_moves = vec![Move::Left, Move::Up, Move::Down, Move::Right];
    //let snake =
    //    Snake::new(&ban_rules, &try_rules, &fallback_moves).expect("Failed to create snake");
    //run_strategy_save_results(snake);
    //run_ai_with_delay(&mut snake, 1000);
}
