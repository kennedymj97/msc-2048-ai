use crate::ai::AI;
use crate::engine::GameEngine;
use crate::engine::Move;

#[derive(Clone)]
pub struct Expectimax(GameEngine);

impl AI for Expectimax {
    fn new() -> Self {
        Expectimax(GameEngine::new())
    }

    fn restart(&mut self) {
        self.0 = GameEngine::new();
    }

    fn get_engine(&mut self) -> &GameEngine {
        &self.0
    }

    fn get_mut_engine(&mut self) -> &mut GameEngine {
        &mut self.0
    }

    fn get_next_move(&mut self) -> Option<Move> {
        self.expectimax(Node::Max, 3).move_dir
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

pub struct ExpectimaxResult {
    score: f64,
    move_dir: Option<Move>,
}

impl Expectimax {
    fn expectimax(&mut self, node: Node, move_depth: u64) -> ExpectimaxResult {
        if move_depth == 0 {
            return self.evaluate_terminal();
        } else {
            match node {
                Node::Max => return self.evaluate_max(move_depth),
                Node::Chance => return self.evaluate_chance(move_depth),
            }
        }
    }

    fn evaluate_terminal(&mut self) -> ExpectimaxResult {
        ExpectimaxResult {
            score: self.get_engine().get_score() as f64,
            move_dir: None,
        }
    }

    fn evaluate_max(&mut self, move_depth: u64) -> ExpectimaxResult {
        let mut best_score = 0.;
        let mut best_move = None;
        for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
            let mut expectimax_copy = self.clone();
            let engine_copy = expectimax_copy.get_mut_engine();
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
            let score = expectimax_copy.expectimax(Node::Chance, move_depth).score;
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

    pub fn evaluate_chance(&mut self, move_depth: u64) -> ExpectimaxResult {
        let engine = self.get_engine();
        let num_empty_tiles = engine.count_empty();
        let mut tiles_searched = 0;
        let mut tmp = engine.get_state();
        let mut insert_tile = 1;
        let mut score = 0.;

        while tiles_searched < num_empty_tiles {
            if (tmp & 0xf) == 0 {
                let mut expectimax_copy = self.clone();
                let engine_copy = expectimax_copy.get_mut_engine();
                engine_copy.update_state(engine_copy.get_state() | insert_tile);
                score += expectimax_copy.expectimax(Node::Max, move_depth - 1).score * 0.9;

                let mut expectimax_copy = self.clone();
                let engine_copy = expectimax_copy.get_mut_engine();
                engine_copy.update_state(engine_copy.get_state() | (insert_tile << 1));
                score += expectimax_copy.expectimax(Node::Max, move_depth - 1).score * 0.1;

                tiles_searched += 1;
            }
            tmp >>= 4;
            insert_tile <<= 4;
        }

        ExpectimaxResult {
            score: score / num_empty_tiles as f64,
            move_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evaluate_chance() {
        let mut expectimax = Expectimax::new();
        let engine = expectimax.get_mut_engine();
        engine.update_state(0x1100000000000000);
        assert_eq!(engine.count_empty(), 14);
        assert_eq!(expectimax.evaluate_chance(1).score, 12.4);
    }
}
