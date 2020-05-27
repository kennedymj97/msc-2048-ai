use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;
use std::collections::HashMap;
use std::thread;

#[derive(Clone, Copy)]
pub struct Expectimax(GameEngine::Board);

static mut HEURISTIC_SCORES: [f64; 0xffff] = [0.; 0xffff];

impl AI for Expectimax {
    fn new() -> Self {
        unsafe { create_heuristic_score_table() };
        Expectimax(GameEngine::new_game())
    }

    fn restart(self) -> Self {
        Expectimax::new()
    }

    fn get_board(self) -> GameEngine::Board {
        self.0
    }

    fn update_board(self, new_board: GameEngine::Board) -> Self {
        Expectimax(new_board)
    }

    fn get_next_move(self) -> Option<Move> {
        let mut map = HashMap::new();
        let depth = 3.max(count_unique(self.get_board()) - 2) as u64;
        let depth = depth.min(6);
        expectimax(self, Node::Max, depth, 1., &mut map).move_dir
        //evaluate_multithread(self, depth, 1.).move_dir
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
struct ExpectimaxResult {
    score: f64,
    move_dir: Option<Move>,
}

type TranspositionTable = HashMap<GameEngine::Board, TranspositionEntry>;

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
            Move::Up => ai = ai.update_board(GameEngine::shift(board, Move::Up)),
            Move::Down => ai = ai.update_board(GameEngine::shift(board, Move::Down)),
            Move::Left => ai = ai.update_board(GameEngine::shift(board, Move::Left)),
            Move::Right => ai = ai.update_board(GameEngine::shift(board, Move::Right)),
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
                expectimax_copy = expectimax_copy.update_board(GameEngine::shift(board, Move::Up))
            }
            Move::Down => {
                expectimax_copy = expectimax_copy.update_board(GameEngine::shift(board, Move::Down))
            }
            Move::Left => {
                expectimax_copy = expectimax_copy.update_board(GameEngine::shift(board, Move::Left))
            }
            Move::Right => {
                expectimax_copy =
                    expectimax_copy.update_board(GameEngine::shift(board, Move::Right))
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
            score: get_heurisitic_score(ai.get_board()) as f64,
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

// Credit to Nneonneo
fn count_unique(board: GameEngine::Board) -> i32 {
    let mut bitset = 0;
    let mut board_copy = board;
    while board_copy != 0 {
        bitset |= 1 << (board_copy & 0xf);
        board_copy >>= 4;
    }

    // Don't count empty tiles.
    bitset >>= 1;

    let mut count = 0;
    while bitset != 0 {
        bitset &= bitset - 1;
        count += 1;
    }
    return count;
}

fn get_heurisitic_score(board: GameEngine::Board) -> f64 {
    let transpose_board = GameEngine::transpose(board);
    (0..4).fold(0., |score, line_idx| {
        let row_val = GameEngine::extract_line(board, line_idx);
        let col_val = GameEngine::extract_line(transpose_board, line_idx);
        let row_score = unsafe { HEURISTIC_SCORES.get_unchecked(row_val as usize) };
        let col_score = unsafe { HEURISTIC_SCORES.get_unchecked(col_val as usize) };
        score + row_score + col_score
    })
}

unsafe fn create_heuristic_score_table() {
    for i in 0..0xffff {
        HEURISTIC_SCORES[i] = calc_heuristic_score(i as u64);
    }
}

// The heuristics developed by Nneonneo were used: https://github.com/nneonneo/2048-ai/blob/master/2048.cpp
fn calc_heuristic_score(line: u64) -> f64 {
    const LOST_PENALTY: f64 = 200000.;
    let tiles = GameEngine::line_to_vec(line);
    LOST_PENALTY + calc_empty(&tiles) + calc_merges(&tiles)
        - calc_monotonicity(&tiles)
        - calc_sum(&tiles)
}

fn calc_sum(line: &Vec<u64>) -> f64 {
    const SUM_POWER: f64 = 3.5;
    const SUM_WEIGHT: f64 = 11.;
    line.iter()
        .fold(0., |acc, &tile_val| acc + (tile_val as f64).powf(SUM_POWER))
        * SUM_WEIGHT
}

fn calc_empty(line: &Vec<u64>) -> f64 {
    const EMPTY_WEIGHT: f64 = 270.0;
    line.iter().fold(0., |num_empty_tiles, &tile_val| {
        if tile_val == 0 {
            num_empty_tiles + 1.
        } else {
            num_empty_tiles
        }
    }) * EMPTY_WEIGHT
}

fn calc_merges(line: &Vec<u64>) -> f64 {
    const MERGES_WEIGHT: f64 = 700.0;
    let mut prev = 0;
    let mut counter = 0.;
    let mut merges = 0.;
    for &tile_val in line {
        if prev == tile_val && tile_val != 0 {
            counter += 1.;
        } else if counter > 0. {
            merges += 1. + counter;
            counter = 0.;
        }
        prev = tile_val;
    }
    if counter > 0. {
        merges += 1. + counter;
    }
    merges * MERGES_WEIGHT
}

fn calc_monotonicity(line: &Vec<u64>) -> f64 {
    const MONOTONICITY_POWER: f64 = 4.;
    const MONOTONICITY_WEIGHT: f64 = 47.;

    let mut monotonicity_left = 0.;
    let mut monotonicity_right = 0.;
    for i in 1..4 {
        let tile1 = line[i - 1] as f64;
        let tile2 = line[i] as f64;
        if tile1 > tile2 {
            monotonicity_left += tile1.powf(MONOTONICITY_POWER) - tile2.powf(MONOTONICITY_POWER);
        } else {
            monotonicity_right += tile2.powf(MONOTONICITY_POWER) - tile1.powf(MONOTONICITY_POWER);
        }
    }
    monotonicity_left.min(monotonicity_right) * MONOTONICITY_WEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_unique() {
        let game = 0x1134000000000000;
        assert_eq!(count_unique(game), 3);
        let game = 0x0000010000000010;
        assert_eq!(count_unique(game), 1);
    }
}
