use crate::ai::AI;
use crate::engine::Basic;
use crate::engine::GameEngine;
use crate::engine::Move;
use crate::engine::Optimised;

pub struct OptimisedExpectimax(Optimised);

impl AI for OptimisedExpectimax {
    type Engine = Optimised;

    fn new() -> Self {
        OptimisedExpectimax(Optimised::new())
    }

    fn restart(&mut self) {
        self.0 = Optimised::new();
    }

    fn get_engine(&mut self) -> &Self::Engine {
        &self.0
    }

    fn get_mut_engine(&mut self) -> &mut Self::Engine {
        &mut self.0
    }

    fn get_next_move(&mut self) -> Option<Move> {
        expectimax(self.get_engine(), Node::Max, 3).move_dir
    }
}

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

    fn get_next_move(&mut self) -> Option<Move> {
        expectimax(self.get_engine(), Node::Max, 3).move_dir
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

fn expectimax(engine: &impl GameEngine, node: Node, move_depth: u64) -> ExpectimaxResult {
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
        let score = expectimax(&engine_copy, Node::Chance, move_depth).score;
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
            idx_avg_score += prob * expectimax(&engine_copy, Node::Max, move_depth - 1).score;
        }
        acc + idx_avg_score
    });

    ExpectimaxResult {
        score: average_score,
        move_dir: None,
    }
}
