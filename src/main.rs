use msc_2048_ai::ai::expectimax::Expectimax;
use msc_2048_ai::ai::expectimax::ExpectimaxMultithread;
use msc_2048_ai::ai::random::Random;
use msc_2048_ai::ai::run_ai;
use msc_2048_ai::ai::AI;
//use msc_2048_ai::ui;

fn main() {
    //ui::start_game_in_ui();
    ExpectimaxMultithread::new();
    run_ai(Box::new(ExpectimaxMultithread));
}
