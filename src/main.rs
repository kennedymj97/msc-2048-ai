use msc_2048_ai::ai::BasicExpectimax;
use msc_2048_ai::ai::BasicRandom;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::Basic;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Initial;
use msc_2048_ai::ui;

fn main() {
    //let mut engine = Initial::new();
    //let mut engine = Basic::new();
    //ai::basic::run_ai(&mut engine);
    //BasicRandom::evaluate(20);
    //BasicExpectimax::evaluate(20);
    //ui::start_game_in_ui(&mut engine);
    let mut basic_random = BasicRandom::new();
    basic_random.evaluate(20);
    let mut basic_exp = BasicExpectimax::new();
    basic_exp.debug();
}
