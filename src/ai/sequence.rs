use std::fs::File;
use std::io::prelude::Write;

use crate::ai::AI;
use crate::engine;
use crate::engine::{Board, GameEngine, GameEngineStores, Move, Score};

type MoveSequence = Vec<Move>;

pub struct Sequence {
    move_sequence: MoveSequence,
    sequence_idx: usize,
    starting_board: Board,
}

impl Sequence {
    pub fn new(move_sequence: MoveSequence) -> Self {
        Sequence {
            move_sequence,
            sequence_idx: 0,
            starting_board: 0,
        }
    }
}

impl AI for Sequence {
    fn get_next_move<T: GameEngine>(&mut self, engine: &T, board: Board) -> Option<Move> {
        if engine.is_game_over(board) {
            return None;
        }

        // terminate if full sequence complete with no change to board
        if self.sequence_idx == 0 {
            if self.starting_board == board {
                for &move_dir in [Move::Up, Move::Down, Move::Left, Move::Right].iter() {
                    let count = self
                        .move_sequence
                        .iter()
                        .filter(|&sequence_move| move_dir == *sequence_move)
                        .collect::<Vec<_>>()
                        .len();
                    if count == 0 {
                        return Some(move_dir);
                    }
                }
                // return None;
            }
            self.starting_board = board;
        }

        let next_move = self.move_sequence[self.sequence_idx];
        self.sequence_idx += 1;
        if self.sequence_idx >= self.move_sequence.len() {
            self.sequence_idx = 0;
        }
        Some(next_move)
    }
}

pub fn evaluate_sequences(length: u32, runs: u32) {
    // create file buffer with information about length and runs
    let engine = GameEngineStores::new();
    let mut f = File::create("results.txt").expect("Failed to create file");
    f.write_fmt(format_args!(
        "Sequence length: {}. Number of test runs for each sequence: {}\n",
        length, runs
    ))
    .expect("Failed to write to file");
    evaluate_sequences_aux(&engine, length, runs, vec![], &mut f);
}

fn evaluate_sequences_aux<T: GameEngine>(
    engine: &T,
    length: u32,
    runs: u32,
    sequence: MoveSequence,
    f: &mut File,
) {
    if length == 0 {
        // run sequence for that number of runs
        println!("Testing sequence: {:?}", sequence);
        let average_score = get_average_score(sequence.clone(), engine, runs);
        // add entry to the file
        f.write_fmt(format_args!(
            "Score: {}, Sequence: {:?}\n",
            average_score, sequence
        ))
        .expect("Failed to write sequence information to file");
        return;
    }

    [Move::Up, Move::Down, Move::Left, Move::Right]
        .iter()
        .for_each(|&move_dir| {
            let mut new_sequence = sequence.clone();
            new_sequence.push(move_dir);
            evaluate_sequences_aux(engine, length - 1, runs, new_sequence, f);
        })
}

fn get_average_score<T: GameEngine>(sequence: MoveSequence, engine: &T, runs: u32) -> f32 {
    (0..runs).fold(0., |score, _| {
        score + run_sequence(sequence.clone(), engine) as f32
    }) / runs as f32
}

fn run_sequence<T: GameEngine>(sequence: MoveSequence, engine: &T) -> Score {
    let mut board = engine::new_board();
    let mut sequence_ai = Sequence::new(sequence);
    loop {
        let next_move = sequence_ai.get_next_move(engine, board);
        match next_move {
            Some(move_dir) => board = engine.make_move(board, move_dir),
            None => break,
        }
    }
    engine.get_score(board)
}
