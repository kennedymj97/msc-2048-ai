use super::attributes;
use super::attributes::Column;
use super::attributes::Corner;
use super::attributes::Row;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub type TryRules = Vec<TryMove>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TryMove {
    Always(Move),
    ProducesMerge(Move),
    IfMergePossible(Move),
    IfMovesLargestTileToCorner(Move, Corner),
    Makes2LargestTilesAdjacent(Move),
    CreatesMonotonicColumn(Move, Column),
    CreatesMonotonicRow(Move, Row),
    LocksColumn(Move, Column),
    LocksRow(Move, Row),
}

impl TryMove {
    pub fn execute(&self, engine: &GameEngine, board: Board) -> Option<Move> {
        match self {
            TryMove::Always(direction) => always_try_move(engine, board, *direction),
            TryMove::ProducesMerge(direction) => {
                try_move_if_produces_potential_merge(engine, board, *direction)
            }
            TryMove::IfMergePossible(direction) => try_move_if_merge_possible(board, *direction),
            TryMove::IfMovesLargestTileToCorner(direction, corner) => {
                try_move_if_moves_largest_tile_to_corner(engine, board, *direction, *corner)
            }
            TryMove::Makes2LargestTilesAdjacent(direction) => {
                try_move_if_makes_2_largest_tiles_adjacent(engine, board, *direction)
            }
            TryMove::CreatesMonotonicColumn(direction, column) => {
                try_move_if_creates_monotonic_column(engine, board, *direction, *column)
            }
            TryMove::CreatesMonotonicRow(direction, row) => {
                try_move_if_creates_monotonic_row(engine, board, *direction, *row)
            }
            TryMove::LocksColumn(direction, column) => {
                try_move_if_locks_column(engine, board, *direction, *column)
            }
            TryMove::LocksRow(direction, row) => {
                try_move_if_locks_row(engine, board, *direction, *row)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        variations.append(&mut always_try_variations());
        variations.append(&mut try_move_if_produces_potential_merge_variations());
        variations.append(&mut try_move_if_merge_possible_variations());
        variations.append(&mut try_move_if_moves_largest_tile_to_corner_variations());
        variations.append(&mut try_move_if_makes_2_largest_tiles_adjacent_variations());
        variations.append(&mut try_move_if_creates_monotonic_column_variations());
        variations.append(&mut try_move_if_creates_monotonic_row_variations());
        variations.append(&mut try_move_if_locks_column_variations());
        variations.append(&mut try_move_if_locks_row_variations());
        variations
    }

    pub fn get_move(&self) -> Move {
        match self {
            TryMove::Always(direction) => *direction,
            TryMove::ProducesMerge(direction) => *direction,
            TryMove::IfMergePossible(direction) => *direction,
            TryMove::IfMovesLargestTileToCorner(direction, _) => *direction,
            TryMove::Makes2LargestTilesAdjacent(direction) => *direction,
            TryMove::CreatesMonotonicColumn(direction, _) => *direction,
            TryMove::CreatesMonotonicRow(direction, _) => *direction,
            TryMove::LocksColumn(direction, _) => *direction,
            TryMove::LocksRow(direction, _) => *direction,
        }
    }
}

impl fmt::Display for TryMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TryMove::ProducesMerge(direction) => {
                write!(f, "try move {} if produces merge", direction)
            }
            TryMove::IfMergePossible(direction) => {
                write!(f, "try move {} if merge possible", direction)
            }
            TryMove::IfMovesLargestTileToCorner(direction, corner) => write!(
                f,
                "try move {} if moves largest tile to {} corner",
                direction, corner
            ),
            TryMove::Always(direction) => write!(f, "always try move {}", direction),
            TryMove::Makes2LargestTilesAdjacent(direction) => write!(
                f,
                "try move {} if makes 2 largest tiles adjacent",
                direction
            ),
            TryMove::CreatesMonotonicColumn(direction, column) => write!(
                f,
                "try move {} if creates monotonic {} column",
                direction, column
            ),
            TryMove::CreatesMonotonicRow(direction, row) => {
                write!(f, "try move {} if creates monotonic {} row", direction, row)
            }
            TryMove::LocksColumn(direction, column) => {
                write!(f, "try move {} if locks {} column", direction, column)
            }
            TryMove::LocksRow(direction, row) => {
                write!(f, "try move {} if locks {} row", direction, row)
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

fn try_move_if_produces_potential_merge(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    let is_merge_possible = match direction {
        Move::Up | Move::Down => attributes::is_merge_possible(board, Move::Left),
        Move::Left | Move::Right => attributes::is_merge_possible(board, Move::Up),
    };
    let new_board = engine.shift(board, direction);
    let is_new_merge_possible = match direction {
        Move::Up | Move::Down => attributes::is_merge_possible(new_board, Move::Left),
        Move::Left | Move::Right => attributes::is_merge_possible(new_board, Move::Up),
    };
    if !is_merge_possible && is_new_merge_possible {
        return Some(direction);
    }
    None
}

fn try_move_if_merge_possible(board: Board, direction: Move) -> Option<Move> {
    if attributes::is_merge_possible(board, direction) {
        return Some(direction);
    }
    None
}

fn try_move_if_moves_largest_tile_to_corner(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    corner: Corner,
) -> Option<Move> {
    let largest_tile_in_corner = attributes::is_largest_tile_in_corner(board, corner);
    let new_board = engine.shift(board, direction);
    let largest_tile_in_corner_new = attributes::is_largest_tile_in_corner(new_board, corner);
    if !largest_tile_in_corner && largest_tile_in_corner_new {
        return Some(direction);
    }
    None
}

fn always_try_move(engine: &GameEngine, board: Board, direction: Move) -> Option<Move> {
    if attributes::is_move_possible(engine, board, direction) {
        return Some(direction);
    }
    None
}

fn try_move_if_makes_2_largest_tiles_adjacent(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    let are_2_largest_tiles_adjacent = attributes::are_2_largest_tiles_adjacent(board);
    let new_board = engine.shift(board, direction);
    let are_2_largest_tiles_adjacent_new = attributes::are_2_largest_tiles_adjacent(new_board);
    if !are_2_largest_tiles_adjacent && are_2_largest_tiles_adjacent_new {
        return Some(direction);
    }
    None
}

fn try_move_if_creates_monotonic_column(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    column: Column,
) -> Option<Move> {
    let is_monotonic = attributes::is_column_monotonic(board, column);
    let new_board = engine.shift(board, direction);
    let is_new_monotonic = attributes::is_column_monotonic(new_board, column);
    if !is_monotonic && is_new_monotonic {
        return Some(direction);
    }
    None
}

fn try_move_if_creates_monotonic_row(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    row: Row,
) -> Option<Move> {
    let is_monotonic = attributes::is_row_monotonic(board, row);
    let new_board = engine.shift(board, direction);
    let is_new_monotonic = attributes::is_row_monotonic(new_board, row);
    if !is_monotonic && is_new_monotonic {
        return Some(direction);
    }
    None
}

fn try_move_if_locks_column(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    column: Column,
) -> Option<Move> {
    let is_locked = attributes::is_column_locked(board, column);
    let new_board = engine.shift(board, direction);
    let is_new_locked = attributes::is_column_locked(new_board, column);
    if !is_locked && is_new_locked {
        return Some(direction);
    }
    None
}

fn try_move_if_locks_row(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    row: Row,
) -> Option<Move> {
    let is_locked = attributes::is_row_locked(board, row);
    let new_board = engine.shift(board, direction);
    let is_new_locked = attributes::is_row_locked(new_board, row);
    if !is_locked && is_new_locked {
        return Some(direction);
    }
    None
}

fn try_move_if_produces_potential_merge_variations() -> Vec<TryMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(TryMove::ProducesMerge(direction));
        variations
    })
}

fn try_move_if_merge_possible_variations() -> Vec<TryMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(TryMove::IfMergePossible(direction));
        variations
    })
}

fn try_move_if_moves_largest_tile_to_corner_variations() -> Vec<TryMove> {
    let mut variations = Vec::new();
    for direction in Move::iterator() {
        match direction {
            Move::Left => {
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::TopLeft,
                ));
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::BottomLeft,
                ));
            }
            Move::Right => {
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::TopRight,
                ));
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::BottomRight,
                ));
            }
            Move::Up => {
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::TopLeft,
                ));
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::TopRight,
                ));
            }
            Move::Down => {
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::BottomLeft,
                ));
                variations.push(TryMove::IfMovesLargestTileToCorner(
                    direction,
                    Corner::BottomRight,
                ));
            }
        }
    }
    variations
}

fn always_try_variations() -> Vec<TryMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(TryMove::Always(direction));
        variations
    })
}

fn try_move_if_makes_2_largest_tiles_adjacent_variations() -> Vec<TryMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(TryMove::Makes2LargestTilesAdjacent(direction));
        variations
    })
}

fn try_move_if_creates_monotonic_column_variations() -> Vec<TryMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        variations.push(TryMove::CreatesMonotonicColumn(Move::Left, column));
        variations.push(TryMove::CreatesMonotonicColumn(Move::Right, column));
    }
    variations
}

fn try_move_if_creates_monotonic_row_variations() -> Vec<TryMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        variations.push(TryMove::CreatesMonotonicRow(Move::Up, row));
        variations.push(TryMove::CreatesMonotonicRow(Move::Down, row));
    }
    variations
}

fn try_move_if_locks_column_variations() -> Vec<TryMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        variations.push(TryMove::LocksColumn(Move::Left, column));
        variations.push(TryMove::LocksColumn(Move::Right, column));
    }
    variations
}

fn try_move_if_locks_row_variations() -> Vec<TryMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        variations.push(TryMove::LocksRow(Move::Left, row));
        variations.push(TryMove::LocksRow(Move::Right, row));
    }
    variations
}
