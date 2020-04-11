use crate::ai::AI;
use crate::engine::Basic;
use crate::engine::GameEngine;
use crate::engine::Move;

pub struct BasicExpectimax;

impl AI for BasicExpectimax {
    fn run() -> u64 {
        let mut engine = Basic::new();
        loop {
            let best_move = expectimax(&engine);
            match best_move {
                Move::Left => engine.move_left(),
                Move::Right => engine.move_right(),
                Move::Up => engine.move_up(),
                Move::Down => engine.move_down(),
            }
            if engine.is_game_over() {
                return engine.get_score();
            }
        }
    }
}

fn expectimax(engine: &impl GameEngine) -> Move {
    let mut max_score = 0.;
    let mut best_move = Move::Down;
    // iterate over possible moves
    for move_dir in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
        let mut engine_copy = engine.clone();
        let old_state = engine_copy.get_state();
        match move_dir {
            Move::Up => engine_copy.move_up_or_down(Move::Up),
            Move::Down => engine_copy.move_up_or_down(Move::Down),
            Move::Left => engine_copy.move_left_or_right(Move::Left),
            Move::Right => engine_copy.move_left_or_right(Move::Right),
        }
        let new_state = engine_copy.get_state();
        if old_state == new_state {
            continue;
        }
        // for each move make every possible random insertion and calculate the average score
        let score = calculate_move_score(&engine_copy);
        if score > max_score {
            max_score = score;
            best_move = move_dir;
        }
    }
    // return the move with the highest average score
    best_move
}

fn calculate_move_score(engine: &impl GameEngine) -> f32 {
    // find all empty tiles
    let empty_tile_idxs = engine.get_empty_tile_idxs();
    // sum up the score for every tile allocation multiplied by probability
    empty_tile_idxs.iter().fold(0., |acc, &idx| {
        let mut idx_avg_score = 0.;
        for (val, prob) in vec![(1, 0.9), (2, 0.1)] {
            let mut engine_copy = engine.clone();
            engine_copy.update_state_by_idx(idx, val);
            idx_avg_score += prob * (engine_copy.get_score() as f32);
        }
        // probability = tile probability * (1 / num free tiles)???
        // don't think there is any use in the scaling down by number of empty tiles acc + ((1 / empty_tile_idxs) * idx_avg_score)
        acc + idx_avg_score
    })
}
