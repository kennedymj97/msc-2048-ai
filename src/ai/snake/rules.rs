use super::attributes;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub type BanRules = Vec<BanMove>;
pub type TryRules = Vec<TryMove>;

#[derive(Clone, Copy, Debug)]
pub enum BanMove {
    IfLeftColumnLocked(Move),
}

impl BanMove {
    pub fn execute(&self, engine: &GameEngine, board: Board) -> Option<Move> {
        match self {
            BanMove::IfLeftColumnLocked(direction) => {
                ban_move_if_left_column_locked(engine, board, *direction)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        for &direction in &[Move::Left, Move::Right, Move::Up, Move::Down] {
            variations.push(BanMove::IfLeftColumnLocked(direction));
        }
        variations
    }
}

impl fmt::Display for BanMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BanMove::IfLeftColumnLocked(direction) => {
                write!(f, "ban move {} if left column locked", direction)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TryMove {
    ProducesLeftMerge(Move),
    IfMergePossible(Move),
}

impl TryMove {
    pub fn execute(&self, engine: &GameEngine, board: Board) -> Option<Move> {
        match self {
            TryMove::ProducesLeftMerge(direction) => {
                try_move_if_produces_left_merge(engine, board, *direction)
            }
            TryMove::IfMergePossible(direction) => {
                try_move_if_merge_possible(engine, board, *direction)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        for &direction in &[Move::Left, Move::Right, Move::Up, Move::Down] {
            variations.push(TryMove::ProducesLeftMerge(direction));
            variations.push(TryMove::IfMergePossible(direction));
        }
        variations
    }
}

impl fmt::Display for TryMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TryMove::ProducesLeftMerge(direction) => {
                write!(f, "try move {} if produces left merge", direction)
            }
            TryMove::IfMergePossible(direction) => {
                write!(f, "try move {} if merge possible", direction)
            }
        }
    }
}

pub fn force_move_if_possible(engine: &GameEngine, board: Board, direction: Move) -> Option<Move> {
    if attributes::is_move_possible(engine, board, direction) {
        return Some(direction);
    }
    None
}

pub fn ban_move_if_left_column_locked(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    if attributes::is_column_locked(board, 0) {
        return None;
    }
    Some(direction)
}

pub fn try_move_if_produces_left_merge(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    if attributes::does_move_produce_merge_in_direction(engine, board, direction, Move::Left) {
        return Some(direction);
    }
    None
}

pub fn try_move_if_merge_possible(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    if attributes::is_merge_possible(board, direction) {
        return Some(direction);
    }
    None
}
