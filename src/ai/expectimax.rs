use crate::ai::AI;
use crate::engine::Basic;
use crate::engine::GameEngine;
use crate::engine::Move;

pub struct BasicExpectimax(Basic);

impl AI for BasicExpectimax {
    type Engine = Basic;

    fn new() -> Self {
        BasicExpectimax(Basic::new())
    }

    fn restart(&mut self) {
        self.0 = Basic::new();
    }

    fn get_engine(&mut self) -> &Self::Engine {
        &self.0
    }

    fn get_mut_engine(&mut self) -> &mut Self::Engine {
        &mut self.0
    }

    fn get_next_move(&mut self) -> Move {
        expectimax2(self.get_engine(), Node::Max, 3)
            .move_dir
            .unwrap()
    }
}

// Three cases:
//  - max nodes (moves)
//  - chance nodes (after moves)
//  - terminal node (depth is 0)
//
// Use an enum to represent the three states.
// Have a seperate function for each of the three cases
//  - the max function (and expectimax will need to return which move is chosen along with the
//  value

enum Node {
    Max,
    Chance,
}

struct ExpectimaxResult {
    score: f64,
    move_dir: Option<Move>,
}

fn expectimax2(engine: &impl GameEngine, node: Node, move_depth: u64) -> ExpectimaxResult {
    if move_depth == 0 {
        return evaluate_terminal(engine);
    } else {
        match node {
            Node::Max => return evaluate_max(engine, move_depth),
            Node::Chance => return evaluate_chance(engine, move_depth),
        }
    }
}

fn evaluate_terminal(engine: &impl GameEngine) -> ExpectimaxResult {
    ExpectimaxResult {
        score: engine.get_score() as f64,
        move_dir: None,
    }
}

fn evaluate_max(engine: &impl GameEngine, move_depth: u64) -> ExpectimaxResult {
    let mut best_score = 0.;
    let mut best_move = None;
    for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
        let mut engine_copy = engine.clone();
        let old_state = engine_copy.get_state();
        match direction {
            Move::Up => engine_copy.move_up_or_down(Move::Up),
            Move::Down => engine_copy.move_up_or_down(Move::Down),
            Move::Left => engine_copy.move_left_or_right(Move::Left),
            Move::Right => engine_copy.move_left_or_right(Move::Right),
        }
        let new_state = engine_copy.get_state();
        if old_state == new_state {
            continue;
        }
        let score = expectimax2(&engine_copy, Node::Chance, move_depth).score;
        if score > best_score {
            best_score = score;
            best_move = Some(direction);
        }
    }
    ExpectimaxResult {
        score: best_score,
        move_dir: best_move,
    }
}

fn evaluate_chance(engine: &impl GameEngine, move_depth: u64) -> ExpectimaxResult {
    let empty_tile_idxs = engine.get_empty_tile_idxs();

    let average_score = empty_tile_idxs.iter().fold(0., |acc, &idx| {
        let mut idx_avg_score = 0.;
        for (val, prob) in vec![(1, 0.9), (2, 0.1)] {
            let mut engine_copy = engine.clone();
            engine_copy.update_state_by_idx(idx, val);
            idx_avg_score += prob * expectimax2(&engine_copy, Node::Max, move_depth - 1).score;
        }
        acc + idx_avg_score
    });

    ExpectimaxResult {
        score: average_score,
        move_dir: None,
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
