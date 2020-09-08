use super::attributes;
use super::attributes::Column;
use super::attributes::Corner;
use super::attributes::Row;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub type BanRules = Vec<BanMove>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BanMove {
    Always(Move),
    IfColumnNotLocked(Move, Column),
    IfRowNotLocked(Move, Row),
    IfBreaksMonotonicityOfColumn(Move, Column),
    IfBreaksMonotonicityOfRow(Move, Row),
    Seperates2LargestTiles(Move),
    UnlocksColumn(Move, Column),
    UnlocksRow(Move, Row),
    RemovesPotentialMerge(Move),
    MovesLargestTileOutOfCorner(Move, Corner),
    FillsColumn(Move, Column),
    FillsRow(Move, Row),
}

impl BanMove {
    pub fn execute<T: GameEngine>(&self, engine: &T, board: Board) -> Option<Move> {
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
            BanMove::UnlocksColumn(direction, column) => {
                ban_move_if_unlocks_column(engine, board, *direction, *column)
            }
            BanMove::UnlocksRow(direction, row) => {
                ban_move_if_unlocks_row(engine, board, *direction, *row)
            }
            BanMove::RemovesPotentialMerge(direction) => {
                ban_move_if_removes_potential_merge(engine, board, *direction)
            }
            BanMove::MovesLargestTileOutOfCorner(direction, corner) => {
                ban_move_if_moves_largest_tile_out_of_corner(engine, board, *direction, *corner)
            }
            BanMove::FillsColumn(direction, column) => {
                ban_move_if_fills_column(engine, board, *direction, *column)
            }
            BanMove::FillsRow(direction, row) => {
                ban_move_if_fills_row(engine, board, *direction, *row)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        variations.append(&mut always_ban_variations());
        variations.append(&mut ban_move_if_column_not_locked_variations());
        variations.append(&mut ban_move_if_row_not_locked_variations());
        variations.append(&mut ban_move_if_breaks_monotonicity_of_column_variations());
        variations.append(&mut ban_move_if_breaks_monotonicity_of_row_variations());
        variations.append(&mut ban_move_if_seperates_2_largest_tiles_variations());
        variations.append(&mut ban_move_if_unlocks_column_variations());
        variations.append(&mut ban_move_if_unlocks_row_variations());
        variations.append(&mut ban_move_if_removes_potential_merge_variations());
        variations.append(&mut ban_move_if_moves_largest_tile_out_of_corner_variations());
        variations.append(&mut ban_move_if_fills_column_variations());
        variations.append(&mut ban_move_if_fills_row_variations());
        //variations.append(&mut ban_move_if_column_not_locked_variations_subset());
        //variations.append(&mut ban_move_if_breaks_monotonicity_of_column_variations_subset());
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
            BanMove::UnlocksColumn(direction, _) => *direction,
            BanMove::UnlocksRow(direction, _) => *direction,
            BanMove::RemovesPotentialMerge(direction) => *direction,
            BanMove::MovesLargestTileOutOfCorner(direction, _) => *direction,
            BanMove::FillsColumn(direction, _) => *direction,
            BanMove::FillsRow(direction, _) => *direction,
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
            BanMove::UnlocksColumn(direction, column) => {
                write!(f, "ban move {} if unlocks {} column", direction, column)
            }
            BanMove::UnlocksRow(direction, row) => {
                write!(f, "ban move {} if unlocks {} row", direction, row)
            }
            BanMove::RemovesPotentialMerge(direction) => {
                write!(f, "ban move {} if removes potential merge", direction)
            }
            BanMove::MovesLargestTileOutOfCorner(direction, corner) => write!(
                f,
                "ban move {} if moves largest tile out of {} corner",
                direction, corner
            ),
            BanMove::FillsColumn(direction, column) => {
                write!(f, "ban move {} if fills {} column", direction, column)
            }
            BanMove::FillsRow(direction, row) => {
                write!(f, "ban move {} if fills {} row", direction, row)
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

fn ban_move_if_breaks_monotonicity_of_column<T: GameEngine>(
    engine: &T,
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

fn ban_move_if_breaks_monotonicity_of_row<T: GameEngine>(
    engine: &T,
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

fn ban_move_if_seperates_2_largest_tiles<T: GameEngine>(
    engine: &T,
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

fn ban_move_if_unlocks_column<T: GameEngine>(
    engine: &T,
    board: Board,
    direction: Move,
    column: Column,
) -> Option<Move> {
    let is_locked = attributes::is_column_locked(board, column);
    let new_board = engine.shift(board, direction);
    let is_new_locked = attributes::is_column_locked(new_board, column);
    if is_locked && !is_new_locked {
        return Some(direction);
    }
    None
}

fn ban_move_if_unlocks_row<T: GameEngine>(
    engine: &T,
    board: Board,
    direction: Move,
    row: Row,
) -> Option<Move> {
    let is_locked = attributes::is_row_locked(board, row);
    let new_board = engine.shift(board, direction);
    let is_new_locked = attributes::is_row_locked(new_board, row);
    if is_locked && !is_new_locked {
        return Some(direction);
    }
    None
}

fn ban_move_if_removes_potential_merge<T: GameEngine>(
    engine: &T,
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
    if is_merge_possible && !is_new_merge_possible {
        return Some(direction);
    }
    None
}

fn ban_move_if_moves_largest_tile_out_of_corner<T: GameEngine>(
    engine: &T,
    board: Board,
    direction: Move,
    corner: Corner,
) -> Option<Move> {
    let is_largest_tile_in_corner = attributes::is_largest_tile_in_corner(board, corner);
    let new_board = engine.shift(board, direction);
    let is_new_largest_tile_in_corner = attributes::is_largest_tile_in_corner(new_board, corner);
    if is_largest_tile_in_corner && !is_new_largest_tile_in_corner {
        return Some(direction);
    }
    None
}

fn ban_move_if_fills_column<T: GameEngine>(
    engine: &T,
    board: Board,
    direction: Move,
    column: Column,
) -> Option<Move> {
    let is_empty = attributes::is_column_empty(board, column);
    let new_board = engine.shift(board, direction);
    let is_new_empty = attributes::is_column_empty(new_board, column);
    if is_empty && !is_new_empty {
        return Some(direction);
    }
    None
}

fn ban_move_if_fills_row<T: GameEngine>(
    engine: &T,
    board: Board,
    direction: Move,
    row: Row,
) -> Option<Move> {
    let is_empty = attributes::is_row_empty(board, row);
    let new_board = engine.shift(board, direction);
    let is_new_empty = attributes::is_row_empty(new_board, row);
    if is_empty && !is_new_empty {
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

fn ban_move_if_column_not_locked_variations_subset() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for direction in Move::iterator() {
        variations.push(BanMove::IfColumnNotLocked(direction, Column::Left));
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

fn ban_move_if_breaks_monotonicity_of_column_variations_subset() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for direction in Move::iterator() {
        variations.push(BanMove::IfBreaksMonotonicityOfColumn(
            direction,
            Column::Left,
        ));
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

fn ban_move_if_unlocks_column_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        variations.push(BanMove::UnlocksColumn(Move::Left, column));
        variations.push(BanMove::UnlocksColumn(Move::Right, column));
    }
    variations
}

fn ban_move_if_unlocks_row_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        variations.push(BanMove::UnlocksRow(Move::Left, row));
        variations.push(BanMove::UnlocksRow(Move::Right, row));
    }
    variations
}

fn ban_move_if_removes_potential_merge_variations() -> Vec<BanMove> {
    Move::iterator().fold(Vec::new(), |mut variations, direction| {
        variations.push(BanMove::RemovesPotentialMerge(direction));
        variations
    })
}

fn ban_move_if_moves_largest_tile_out_of_corner_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for direction in Move::iterator() {
        match direction {
            Move::Left => {
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::TopRight,
                ));
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::BottomRight,
                ));
            }
            Move::Right => {
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::TopLeft,
                ));
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::BottomLeft,
                ));
            }
            Move::Up => {
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::BottomLeft,
                ));
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::BottomRight,
                ));
            }
            Move::Down => {
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::TopLeft,
                ));
                variations.push(BanMove::MovesLargestTileOutOfCorner(
                    direction,
                    Corner::TopRight,
                ));
            }
        }
    }
    variations
}

fn ban_move_if_fills_column_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for column in Column::iterator() {
        if column != Column::Left {
            variations.push(BanMove::FillsColumn(Move::Right, column));
        }
        if column != Column::Right {
            variations.push(BanMove::FillsColumn(Move::Left, column));
        }
    }
    variations
}

fn ban_move_if_fills_row_variations() -> Vec<BanMove> {
    let mut variations = Vec::new();
    for row in Row::iterator() {
        if row != Row::Top {
            variations.push(BanMove::FillsRow(Move::Down, row));
        }
        if row != Row::Bottom {
            variations.push(BanMove::FillsRow(Move::Up, row));
        }
    }
    variations
}
