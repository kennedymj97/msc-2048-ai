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
        TryMoveIfMergePossible::new(Move::Left),
        TryMoveIfProducesLeftMerge::new(Move::Down),
        BanMoveIfLeftColumnLocked::new(Move::Up),
        TryMoveIfProducesLeftMerge::new(Move::Up),
        ForceMoveIfPossible::new(Move::Left),
        ForceMoveIfPossible::new(Move::Down),
        ForceMoveIfPossible::new(Move::Up),
        ForceMoveIfPossible::new(Move::Right),
    ]);
    println!("{}", snake);
    run_ai(Box::new(snake));
    // //run_ai_with_delay(Box::new(snake), 1000)
}

//is_merge_left_possible,
//|board| does_move_produce_left_merge(board, Move::Down),
//is_left_column_locked,
//|board| does_move_produce_left_merge(board, Move::Up),
//|board| is_move_possible(board, Move::Left),
//|board| is_move_possible(board, Move::Down),
//|board| is_move_possible(board, Move::Up),
//|board| is_move_possible(board, Move::Right),
