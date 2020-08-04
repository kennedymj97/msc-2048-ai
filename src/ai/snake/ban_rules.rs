use super::attributes;
use super::attributes::Column;
use super::attributes::Row;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub type BanRules = Vec<BanMove>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BanMove {
    IfColumnNotLocked(Move, Column),
    IfRowNotLocked(Move, Row),
    IfBreaksMonotonicityOfColumn(Move, Column),
    IfBreaksMonotonicityOfRow(Move, Row),
    Seperates2LargestTiles(Move),
    Always(Move),
}

impl BanMove {
    pub fn execute(&self, engine: &GameEngine, board: Board) -> Option<Move> {
        match self {
            BanMove::Always(direction) => Some(*direction),
            BanMove::IfColumnNotLocked(direction, column) => {
                ban_move_if_column_not_locked(board, *direction, *column)
            }
            BanMove::IfRowNotLocked(direction, row) => {
                ban_move_if_row_not_locked(board, *direction, *row)
            }
            BanMove::IfBreaksMonotonicityOfColumn(direction, column) => {
                ban_move_if_breaks_monotonicity_of_column(engine, board, *direction, *column)
            }
            BanMove::IfBreaksMonotonicityOfRow(direction, row) => {
                ban_move_if_breaks_monotonicity_of_row(engine, board, *direction, *row)
            }
            BanMove::Seperates2LargestTiles(direction) => {
                ban_move_if_seperates_2_largest_tiles(engine, board, *direction)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        variations.append(&mut ban_move_if_column_not_locked_variations());
        variations.append(&mut ban_move_if_row_not_locked_variations());
        variations.append(&mut ban_move_if_breaks_monotonicity_of_column_variations());
        variations.append(&mut ban_move_if_breaks_monotonicity_of_row_variations());
        variations.append(&mut ban_move_if_seperates_2_largest_tiles_variations());
        variations.append(&mut always_ban_variations());
        variations
    }

    pub fn get_move(&self) -> Move {
        match self {
            BanMove::Always(direction) => *direction,
            BanMove::IfColumnNotLocked(direction, _) => *direction,
            BanMove::IfRowNotLocked(direction, _) => *direction,
            BanMove::IfBreaksMonotonicityOfColumn(direction, _) => *direction,
            BanMove::IfBreaksMonotonicityOfRow(direction, _) => *direction,
            BanMove::Seperates2LargestTiles(direction) => *direction,
        }
    }
}

impl fmt::Display for BanMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BanMove::IfColumnNotLocked(direction, column) => {
                write!(f, "ban move {} if {} column not locked", direction, column)
            }
            BanMove::IfRowNotLocked(direction, row) => {
                write!(f, "ban move {} if {} row not locked", direction, row)
            }
            BanMove::IfBreaksMonotonicityOfColumn(direction, column) => write!(
                f,
                "ban move {} if breaks monotonicity of {} column",
                direction, column,
            ),
            BanMove::IfBreaksMonotonicityOfRow(direction, row) => write!(
                f,
                "ban move {} if breaks monotonicity of {} row",
                direction, row,
            ),
            BanMove::Always(direction) => write!(f, "always ban move {}", direction),
            BanMove::Seperates2LargestTiles(direction) => {
                write!(f, "ban move {} if seperates 2 largest tiles", direction)
            }
        }
    }
}

fn ban_move_if_column_not_locked(board: Board, direction: Move, column: Column) -> Option<Move> {
    if attributes::is_column_locked(board, column) {
        return None;
    }
    Some(direction)
}

fn ban_move_if_row_not_locked(board: Board, direction: Move, row: Row) -> Option<Move> {
    if attributes::is_row_locked(board, row) {
        return None;
    }
    Some(direction)
}

fn ban_move_if_breaks_monotonicity_of_column(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    column: Column,
) -> Option<Move> {
    let is_monotonic = attributes::is_column_monotonic(board, column);
    let new_board = engine.shift(board, direction);
    let is_new_monotonic = attributes::is_column_monotonic(new_board, column);
    if is_monotonic && !is_new_monotonic {
        return Some(direction);
    }
    None
}

fn ban_move_if_breaks_monotonicity_of_row(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    row: Row,
) -> Option<Move> {
    let is_monotonic = attributes::is_row_monotonic(board, row);
    let new_board = engine.shift(board, direction);
    let is_new_monotonic = attributes::is_row_monotonic(new_board, row);
    if is_monotonic && !is_new_monotonic {
        return Some(direction);
    }
    None
}

fn ban_move_if_seperates_2_largest_tiles(
    engine: &GameEngine,
    board: Board,
    direction: Move,
) -> Option<Move> {
    let are_2_largest_tiles_adjacent = attributes::are_2_largest_tiles_adjacent(board);
    let new_board = engine.shift(board, direction);
    let are_2_largest_tiles_adjacent_new = attributes::are_2_largest_tiles_adjacent(new_board);
    if are_2_largest_tiles_adjacent && !are_2_largest_tiles_adjacent_new {
        return Some(direction);
    }
    None
}

fn ban_move_if_column_not_locked_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        variations.push(BanMove::IfColumnNotLocked(Move::Up, column));
        variations.push(BanMove::IfColumnNotLocked(Move::Down, column));
    }
    variations
}

fn ban_move_if_row_not_locked_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        variations.push(BanMove::IfRowNotLocked(Move::Left, row));
        variations.push(BanMove::IfRowNotLocked(Move::Right, row));
    }
    variations
}

fn ban_move_if_breaks_monotonicity_of_column_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        variations.push(BanMove::IfBreaksMonotonicityOfColumn(Move::Left, column));
        variations.push(BanMove::IfBreaksMonotonicityOfColumn(Move::Right, column));
    }
    variations
}

fn ban_move_if_breaks_monotonicity_of_row_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        variations.push(BanMove::IfBreaksMonotonicityOfRow(Move::Up, row));
        variations.push(BanMove::IfBreaksMonotonicityOfRow(Move::Down, row));
    }
    variations
}

fn always_ban_variations() -> Vec<BanMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(BanMove::Always(direction));
        variations
    })
}

fn ban_move_if_seperates_2_largest_tiles_variations() -> Vec<BanMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(BanMove::Seperates2LargestTiles(direction));
        variations
    })
}
