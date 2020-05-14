use msc_2048_ai::ai::Expectimax;
use msc_2048_ai::ai::Random;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::ui;

fn main() {
    //let mut engine = GameEngine::new();
    //ui::start_game_in_ui(&mut engine);
    let mut expectimax = Expectimax::new();
    expectimax.evaluate(1);
}
