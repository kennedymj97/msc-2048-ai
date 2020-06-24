use msc_2048_ai::ai::expectimax::Expectimax;
use msc_2048_ai::ai::expectimax::ExpectimaxMultithread;
use msc_2048_ai::ai::random::Random;
use msc_2048_ai::ai::record_ai_game;
use msc_2048_ai::ai::run_ai;
use msc_2048_ai::ai::run_ai_with_delay;
use msc_2048_ai::ai::sequence::evaluate_sequences;
use msc_2048_ai::ai::sequence::Sequence;
use msc_2048_ai::engine::Move;
//use msc_2048_ai::ui;

fn main() {
    //ui::start_game_in_ui();
    Expectimax::new();
    record_ai_game(Box::new(Expectimax), "test");
    //let move_sequence = vec![
    //    Move::Right,
    //    Move::Right,
    //    Move::Right,
    //    Move::Up,
    //    Move::Right,
    //    Move::Down,
    //    Move::Right,
    //    Move::Up,
    //];
    //let sequence = Sequence::new(move_sequence);
    //run_ai_with_delay(Box::new(sequence), 0);
    //evaluate_sequences(8, 10);
}
