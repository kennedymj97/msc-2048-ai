use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;

#[derive(Clone, Copy)]
pub struct Expectimax(GameEngine::Board);

impl AI for Expectimax {
    fn new() -> Self {
        Expectimax(GameEngine::new_game())
    }

    fn restart(self) -> Self {
        Expectimax(GameEngine::restart_game())
    }

    fn get_board(self) -> GameEngine::Board {
        self.0
    }

    fn update_board(self, new_board: GameEngine::Board) -> Self {
        Expectimax(new_board)
    }

    fn get_next_move(self) -> Option<Move> {
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
    fn expectimax(self, node: Node, move_depth: u64) -> ExpectimaxResult {
        if move_depth == 0 {
            return self.evaluate_terminal();
        } else {
            match node {
                Node::Max => return self.evaluate_max(move_depth),
                Node::Chance => return self.evaluate_chance(move_depth),
            }
        }
    }

    fn evaluate_terminal(self) -> ExpectimaxResult {
        ExpectimaxResult {
            score: GameEngine::get_score(self.get_board()) as f64,
            move_dir: None,
        }
    }

    fn evaluate_max(self, move_depth: u64) -> ExpectimaxResult {
        let mut best_score = 0.;
        let mut best_move = None;
        for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
            let board = self.get_board();
            let mut expectimax_copy = self;
            match direction {
                Move::Up => {
                    expectimax_copy =
                        expectimax_copy.update_board(GameEngine::move_up_or_down(board, Move::Up))
                }
                Move::Down => {
                    expectimax_copy =
                        expectimax_copy.update_board(GameEngine::move_up_or_down(board, Move::Down))
                }
                Move::Left => {
                    expectimax_copy = expectimax_copy
                        .update_board(GameEngine::move_left_or_right(board, Move::Left))
                }
                Move::Right => {
                    expectimax_copy = expectimax_copy
                        .update_board(GameEngine::move_left_or_right(board, Move::Right))
                }
            }
            if expectimax_copy.get_board() == board {
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

    fn evaluate_chance(self, move_depth: u64) -> ExpectimaxResult {
        let board = self.get_board();
        let num_empty_tiles = GameEngine::count_empty(board);
        let mut tiles_searched = 0;
        let mut tmp = board;
        let mut insert_tile = 1;
        let mut score = 0.;

        while tiles_searched < num_empty_tiles {
            if (tmp & 0xf) == 0 {
                let expectimax_copy = self;
                let expectimax_copy =
                    expectimax_copy.update_board(expectimax_copy.get_board() | insert_tile);
                score += expectimax_copy.expectimax(Node::Max, move_depth - 1).score * 0.9;

                let expectimax_copy = self;
                let expectimax_copy =
                    expectimax_copy.update_board(expectimax_copy.get_board() | (insert_tile << 1));
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
        let expectimax = Expectimax::new();
        let expectimax = expectimax.update_board(0x1100000000000000);
        assert_eq!(Expectimax::evaluate_chance(expectimax, 1).score, 12.4);
    }
}
