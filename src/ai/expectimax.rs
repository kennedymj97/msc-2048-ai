use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;
use std::collections::HashMap;
use std::thread;

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
        //let mut map = HashMap::new();
        let depth = 3.max(GameEngine::count_unique(self.get_board()) + 1) as u64;
        //let depth = depth.min(6);
        //expectimax(self, Node::Max, depth, 1., &mut map).move_dir
        evaluate_multithread(self, depth, 1.).move_dir
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

#[derive(Debug)]
pub struct ExpectimaxResult {
    score: f64,
    move_dir: Option<Move>,
}

type TranspositionTable = HashMap<u64, TranspositionEntry>;

fn evaluate_multithread(ai: Expectimax, move_depth: u64, cum_prob: f32) -> ExpectimaxResult {
    let mut threads = vec![];
    for &direction in &[Move::Up, Move::Down, Move::Left, Move::Right] {
        // spawn computation threads using function and push to vec
        let expectimax_copy = ai;
        threads.push(spawn_move_computation(
            expectimax_copy,
            move_depth,
            cum_prob,
            direction,
        ));
    }

    let mut best_result = ExpectimaxResult {
        score: 0.,
        move_dir: None,
    };
    for thread in threads {
        let result = thread.join().unwrap();
        if result.score > best_result.score {
            best_result = result;
        }
    }
    best_result
}

fn spawn_move_computation(
    ai: Expectimax,
    move_depth: u64,
    cum_prob: f32,
    direction: Move,
) -> thread::JoinHandle<ExpectimaxResult> {
    thread::spawn(move || {
        let board = ai.get_board();
        let mut ai = ai;
        match direction {
            Move::Up => ai = ai.update_board(GameEngine::move_up_or_down(board, Move::Up)),
            Move::Down => ai = ai.update_board(GameEngine::move_up_or_down(board, Move::Down)),
            Move::Left => ai = ai.update_board(GameEngine::move_left_or_right(board, Move::Left)),
            Move::Right => ai = ai.update_board(GameEngine::move_left_or_right(board, Move::Right)),
        }
        if ai.get_board() != board {
            return ExpectimaxResult {
                score: expectimax(ai, Node::Chance, move_depth, cum_prob, &mut HashMap::new())
                    .score,
                move_dir: Some(direction),
            };
        }

        ExpectimaxResult {
            score: 0.,
            move_dir: None,
        }
    })
}

fn expectimax(
    ai: Expectimax,
    node: Node,
    move_depth: u64,
    cum_prob: f32,
    map: &mut TranspositionTable,
) -> ExpectimaxResult {
    match node {
        Node::Max => return evaluate_max(ai, move_depth, cum_prob, map),
        Node::Chance => return evaluate_chance(ai, move_depth, cum_prob, map),
    }
}

fn evaluate_max(
    ai: Expectimax,
    move_depth: u64,
    cum_prob: f32,
    map: &mut TranspositionTable,
) -> ExpectimaxResult {
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
            let score = expectimax(expectimax_copy, Node::Chance, move_depth, cum_prob, map).score;
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

struct TranspositionEntry {
    score: f64,
    move_depth: u64,
}

fn evaluate_chance(
    ai: Expectimax,
    move_depth: u64,
    cum_prob: f32,
    map: &mut TranspositionTable,
) -> ExpectimaxResult {
    if move_depth == 0 || cum_prob < 0.0001 {
        return ExpectimaxResult {
            score: GameEngine::get_score(ai.get_board()) as f64,
            move_dir: None,
        };
    }

    let board = ai.get_board();

    // Check if board has already been seen
    if let Some(entry) = map.get(&board) {
        // need to check depth is greater than or equal to current depth
        // if depth is less then the score will not be accurate enough
        if entry.move_depth >= move_depth {
            return ExpectimaxResult {
                score: entry.score,
                move_dir: None,
            };
        }
    }

    let num_empty_tiles = GameEngine::count_empty(board);
    let mut tiles_searched = 0;
    let mut tmp = board;
    let mut insert_tile = 1;
    let mut score = 0.;
    let cum_prob = cum_prob / num_empty_tiles as f32;

    while tiles_searched < num_empty_tiles {
        if (tmp & 0xf) == 0 {
            let expectimax_copy = ai;
            let expectimax_copy =
                expectimax_copy.update_board(expectimax_copy.get_board() | insert_tile);
            score += expectimax(
                expectimax_copy,
                Node::Max,
                move_depth - 1,
                cum_prob * 0.9,
                map,
            )
            .score
                * 0.9;

            let expectimax_copy = ai;
            let expectimax_copy =
                expectimax_copy.update_board(expectimax_copy.get_board() | (insert_tile << 1));
            score += expectimax(
                expectimax_copy,
                Node::Max,
                move_depth - 1,
                cum_prob * 0.1,
                map,
            )
            .score
                * 0.1;

            tiles_searched += 1;
        }
        tmp >>= 4;
        insert_tile <<= 4;
    }

    score = score / num_empty_tiles as f64;

    map.insert(board, TranspositionEntry { score, move_depth });

    ExpectimaxResult {
        score,
        move_dir: None,
    }
}
