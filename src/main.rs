use msc_2048_ai::ai;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Initial;
use msc_2048_ai::ui;

fn main() {
    let mut engine = Initial::new();
    //ai::run_ai(&mut engine);
    ui::start_game_in_ui(&mut engine);
}
