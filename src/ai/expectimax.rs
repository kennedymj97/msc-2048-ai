use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;
use once_cell::unsync::Lazy;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Expectimax(GameEngine::Board);

impl AI for Expectimax {
    fn new() -> Self {
        Expectimax(GameEngine::new_game())
    }

    fn restart(self) -> Self {
        Expectimax(GameEngine::start_new_game())
    }

    fn get_board(self) -> GameEngine::Board {
        self.0
    }

    fn update_board(self, new_board: GameEngine::Board) -> Self {
        Expectimax(new_board)
    }

    fn get_next_move(self) -> Option<Move> {
        unsafe { TRANSPOSITION = Lazy::new(|| HashMap::new()) }
        expectimax(self, Node::Max, 6).move_dir
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

fn expectimax(ai: Expectimax, node: Node, move_depth: u64) -> ExpectimaxResult {
    if move_depth == 0 {
        return evaluate_terminal(ai);
    } else {
        match node {
            Node::Max => return evaluate_max(ai, move_depth),
            Node::Chance => return evaluate_chance(ai, move_depth),
        }
    }
}

fn evaluate_terminal(ai: Expectimax) -> ExpectimaxResult {
    ExpectimaxResult {
        score: GameEngine::get_score(ai.get_board()) as f64,
        move_dir: None,
    }
}

fn evaluate_max(ai: Expectimax, move_depth: u64) -> ExpectimaxResult {
    let mut best_score = 0.;
    let mut best_move = None;
    for &direction in &[Move::Up, Move::Down, Move::Left, Move::Right] {
        let board = ai.get_board();
        let mut expectimax_copy = ai;
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
                expectimax_copy =
                    expectimax_copy.update_board(GameEngine::move_left_or_right(board, Move::Left))
            }
            Move::Right => {
                expectimax_copy =
                    expectimax_copy.update_board(GameEngine::move_left_or_right(board, Move::Right))
            }
        }
        if expectimax_copy.get_board() != board {
            let score = expectimax(expectimax_copy, Node::Chance, move_depth).score;
            if score > best_score {
                best_score = score;
                best_move = Some(direction);
            }
        }
    }
    ExpectimaxResult {
        score: best_score,
        move_dir: best_move,
    }
}

static mut TRANSPOSITION: Lazy<HashMap<u64, TranspositionEntry>> = Lazy::new(|| HashMap::new());

struct TranspositionEntry {
    score: f64,
    move_depth: u64,
}

fn evaluate_chance(ai: Expectimax, move_depth: u64) -> ExpectimaxResult {
    let board = ai.get_board();

    // Check if board has already been seen
    unsafe {
        if let Some(entry) = TRANSPOSITION.get(&board) {
            // need to check depth is greater than or equal to current depth
            // if depth is less then the score will not be accurate enough
            if entry.move_depth >= move_depth {
                return ExpectimaxResult {
                    score: entry.score,
                    move_dir: None,
                };
            }
        }
    }

    let num_empty_tiles = GameEngine::count_empty(board);
    let mut tiles_searched = 0;
    let mut tmp = board;
    let mut insert_tile = 1;
    let mut score = 0.;

    while tiles_searched < num_empty_tiles {
        if (tmp & 0xf) == 0 {
            let expectimax_copy = ai;
            let expectimax_copy =
                expectimax_copy.update_board(expectimax_copy.get_board() | insert_tile);
            score += expectimax(expectimax_copy, Node::Max, move_depth - 1).score * 0.9;

            let expectimax_copy = ai;
            let expectimax_copy =
                expectimax_copy.update_board(expectimax_copy.get_board() | (insert_tile << 1));
            score += expectimax(expectimax_copy, Node::Max, move_depth - 1).score * 0.1;

            tiles_searched += 1;
        }
        tmp >>= 4;
        insert_tile <<= 4;
    }

    score = score / num_empty_tiles as f64;

    // add the result to the TRANSPOSITION table before returning
    unsafe {
        TRANSPOSITION.insert(board, TranspositionEntry { score, move_depth });
    }

    ExpectimaxResult {
        score,
        move_dir: None,
    }
}
