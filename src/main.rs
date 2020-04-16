use msc_2048_ai::ai::BasicExpectimax;
use msc_2048_ai::ai::BasicRandom;
use msc_2048_ai::ai::OptimisedExpectimax;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::Basic;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Optimised;
use msc_2048_ai::ui;

fn main() {
    //BasicRandom::evaluate(20);
    //BasicExpectimax::evaluate(20);
    //let mut engine = Optimised::new();
    //ui::start_game_in_ui(&mut engine);
    //let mut basic_exp = BasicExpectimax::new();
    //basic_exp.evaluate(1);
    let mut optimised_exp = OptimisedExpectimax::new();
    optimised_exp.evaluate(1);
}
