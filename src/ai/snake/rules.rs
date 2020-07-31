use super::attributes;
use super::attributes::Column;
use super::attributes::Corner;
use super::attributes::Row;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub type BanRules = Vec<BanMove>;
pub type TryRules = Vec<TryMove>;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TryMove {
    ProducesMerge(Move),
    IfMergePossible(Move),
    IfMovesLargestTileToCorner(Move, Corner),
    Makes2LargestTilesAdjacent(Move),
    Always(Move),
}

impl TryMove {
    pub fn execute(&self, engine: &GameEngine, board: Board) -> Option<Move> {
        match self {
            TryMove::Always(direction) => always_try_move(engine, board, *direction),
            TryMove::ProducesMerge(direction) => {
                try_move_if_produces_merge(engine, board, *direction)
            }
            TryMove::IfMergePossible(direction) => try_move_if_merge_possible(board, *direction),
            TryMove::IfMovesLargestTileToCorner(direction, corner) => {
                try_move_if_moves_largest_tile_to_corner(engine, board, *direction, *corner)
            }
            TryMove::Makes2LargestTilesAdjacent(direction) => {
                try_move_if_makes_2_largest_tiles_adjacent(engine, board, *direction)
            }
        }
    }

    pub fn generate_all_variations() -> Vec<Self> {
        let mut variations = Vec::new();
        variations.append(&mut always_try_variations());
        variations.append(&mut try_move_if_produces_merge_variations());
        variations.append(&mut try_move_if_merge_possible_variations());
        variations.append(&mut try_move_if_moves_largest_tile_to_corner_variations());
        variations.append(&mut try_move_if_makes_2_largest_tiles_adjacent_variations());
        variations
    }

    pub fn get_move(&self) -> Move {
        match self {
            TryMove::Always(direction) => *direction,
            TryMove::ProducesMerge(direction) => *direction,
            TryMove::IfMergePossible(direction) => *direction,
            TryMove::IfMovesLargestTileToCorner(direction, _) => *direction,
            TryMove::Makes2LargestTilesAdjacent(direction) => *direction,
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
        }
    }
}

pub fn force_move_if_possible(engine: &GameEngine, board: Board, direction: Move) -> Option<Move> {
    if attributes::is_move_possible(engine, board, direction) {
        return Some(direction);
    }
    None
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

fn try_move_if_produces_merge(engine: &GameEngine, board: Board, direction: Move) -> Option<Move> {
    match direction {
        Move::Left | Move::Right => {
            if attributes::does_move_produce_merge_in_direction(engine, board, direction, Move::Up)
            {
                return Some(direction);
            }
        }
        Move::Up | Move::Down => {
            if attributes::does_move_produce_merge_in_direction(
                engine,
                board,
                direction,
                Move::Left,
            ) {
                return Some(direction);
            }
        }
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

fn try_move_if_produces_merge_variations() -> Vec<TryMove> {
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
