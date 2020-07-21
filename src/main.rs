use msc_2048_ai::ai::record_ai_game;
use msc_2048_ai::ai::run_ai;
use msc_2048_ai::ai::run_ai_with_delay;
use msc_2048_ai::ai::snake::evaluate_strategies::find_best_strategies;
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::run_strategies::progressive_brute_force;
use msc_2048_ai::ai::snake::Snake;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::Move;

fn main() {
    progressive_brute_force(1, 3, "apple");
}
