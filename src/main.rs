use msc_2048_ai::ai;
use msc_2048_ai::engine::Basic;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Initial;
use msc_2048_ai::ui;

fn main() {
    //let mut engine = Initial::new();
    let mut engine = Basic::new();
    //ai::basic::run_ai(&mut engine);
    ai::expectimax::run_ai(&mut engine);
    //ui::start_game_in_ui(&mut engine);
}
