use crate::engine::GameEngine;
use crate::engine::Move;

pub fn run_ai(engine: &mut impl GameEngine) {
    loop {
        let best_move = expectimax(engine);
        println!("{:?}", best_move);
        match best_move {
            Move::Left => engine.move_left(),
            Move::Right => engine.move_right(),
            Move::Up => engine.move_up(),
            Move::Down => engine.move_down(),
        }
        println!("{}", engine);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if engine.is_game_over() {
            break;
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
            idx_avg_score += prob * (evaluate(&engine_copy) as f32);
        }
        // probability = tile probability * (1 / num free tiles)???
        // don't think there is any use in the scaling down by number of empty tiles acc + ((1 / empty_tile_idxs) * idx_avg_score)
        acc + idx_avg_score
    })
}

fn evaluate(engine: &impl GameEngine) -> u64 {
    // very simply just sum the value of the tiles
    engine
        .to_vec()
        .iter()
        .map(|&x| match x {
            Some(y) => y,
            None => 0,
        })
        .sum()
}
