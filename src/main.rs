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
use msc_2048_ai::ai::snake::rules::*;
use msc_2048_ai::ai::snake::Rule;
use msc_2048_ai::ai::snake::Snake;

fn main() {
    //ui::start_game_in_ui();
    //Expectimax::new();
    //record_ai_game(Box::new(Expectimax), "test");
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
    let snake = Snake::new(vec![
        Rule::new(
            try_move_if_merge_possible,
            Move::Left,
            "is_merge_possible".to_string(),
        ),
        Rule::new(
            try_move_if_produces_left_merge,
            Move::Down,
            "try_move_if_produces_left_merge".to_string(),
        ),
        Rule::new(
            ban_move_if_left_column_locked,
            Move::Up,
            "ban_move_if_left_column_locked".to_string(),
        ),
        Rule::new(
            try_move_if_produces_left_merge,
            Move::Up,
            "try_move_if_produces_left_merge".to_string(),
        ),
        Rule::new(
            force_move_if_possible,
            Move::Left,
            "force_move_if_possible".to_string(),
        ),
        Rule::new(
            force_move_if_possible,
            Move::Down,
            "force_move_if_possible".to_string(),
        ),
        Rule::new(
            force_move_if_possible,
            Move::Up,
            "force_move_if_possible".to_string(),
        ),
        Rule::new(
            force_move_if_possible,
            Move::Right,
            "force_move_if_possible".to_string(),
        ),
    ]);
    //println!("{}", snake);
    run_ai(Box::new(snake));
    //run_ai_with_delay(Box::new(snake), 1000)
}

//is_merge_left_possible,
//|board| does_move_produce_left_merge(board, Move::Down),
//is_left_column_locked,
//|board| does_move_produce_left_merge(board, Move::Up),
//|board| is_move_possible(board, Move::Left),
//|board| is_move_possible(board, Move::Down),
//|board| is_move_possible(board, Move::Up),
//|board| is_move_possible(board, Move::Right),
