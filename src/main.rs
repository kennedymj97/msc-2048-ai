use msc_2048_ai::ai::Expectimax;
use msc_2048_ai::ai::Random;
use msc_2048_ai::ai::AI;
use msc_2048_ai::ai::{debug, evaluate};
use msc_2048_ai::ui;
use once_cell::unsync::Lazy;
use std::collections::HashMap;

fn main() {
    //ui::start_game_in_ui();
    let expectimax = Expectimax::new();
    evaluate(expectimax, 5);
    //debug(expectimax);
    //let random = Random::new();
    //debug(random);
}
