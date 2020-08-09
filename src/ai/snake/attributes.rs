use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Column {
    Left,
    MiddleLeft,
    MiddleRight,
    Right,
}

impl Column {
    fn get_idx(&self) -> usize {
        match self {
            Column::Left => 0,
            Column::MiddleLeft => 1,
            Column::MiddleRight => 2,
            Column::Right => 3,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Column> {
        [
            Column::Left,
            Column::MiddleLeft,
            Column::MiddleRight,
            Column::Right,
        ]
        .iter()
        .copied()
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Column::Left => write!(f, "left"),
            Column::MiddleLeft => write!(f, "middle left"),
            Column::MiddleRight => write!(f, "middle right"),
            Column::Right => write!(f, "right"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Row {
    Top,
    MiddleTop,
    MiddleBottom,
    Bottom,
}

impl Row {
    fn get_idx(&self) -> usize {
        match self {
            Row::Top => 0,
            Row::MiddleTop => 1,
            Row::MiddleBottom => 2,
            Row::Bottom => 3,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Row> {
        [Row::Top, Row::MiddleTop, Row::MiddleBottom, Row::Bottom]
            .iter()
            .copied()
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Row::Top => write!(f, "top"),
            Row::MiddleTop => write!(f, "middle top"),
            Row::MiddleBottom => write!(f, "middle bottom"),
            Row::Bottom => write!(f, "bottom"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Corner {
    BottomLeft,
    TopLeft,
    BottomRight,
    TopRight,
}

impl Corner {
    fn get_idx(&self) -> usize {
        match self {
            Corner::BottomLeft => 12,
            Corner::TopLeft => 0,
            Corner::BottomRight => 15,
            Corner::TopRight => 3,
        }
    }
}

impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Corner::BottomLeft => write!(f, "bottom left"),
            Corner::TopLeft => write!(f, "top left"),
            Corner::BottomRight => write!(f, "bottom right"),
            Corner::TopRight => write!(f, "top right"),
        }
    }
}

pub fn is_move_possible(engine: &GameEngine, board: Board, direction: Move) -> bool {
    let new_board = engine.shift(board, direction);
    board != new_board
}

pub fn is_column_locked(board: Board, column: Column) -> bool {
    let mut previous_val = 0;
    let col_idx = column.get_idx();
    for i in 0..4 {
        let idx = (i * 4) + col_idx;
        let val = GameEngine::get_tile(board, idx);
        if val == 0 || val == previous_val {
            return false;
        }
        previous_val = val
    }
    true
}

pub fn is_row_locked(board: Board, row: Row) -> bool {
    let mut previous_val = 0;
    let row_idx = row.get_idx();
    for i in 0..4 {
        let idx = (row_idx * 4) + i;
        let val = GameEngine::get_tile(board, idx);
        if val == 0 || val == previous_val {
            return false;
        }
        previous_val = val
    }
    true
}

pub fn is_merge_possible(board: Board, direction: Move) -> bool {
    for i in 0..4 {
        let mut previous_val_row = 0;
        let mut previous_val_col = 0;
        for j in 0..4 {
            let idx_row = (i * 4) + j;
            let idx_col = (j * 4) + i;
            let val_row = GameEngine::get_tile(board, idx_row);
            let val_col = GameEngine::get_tile(board, idx_col);
            match direction {
                Move::Left | Move::Right => {
                    if val_row == previous_val_row && val_row != 0 {
                        return true;
                    }
                }
                Move::Up | Move::Down => {
                    if val_col == previous_val_col && val_col != 0 {
                        return true;
                    }
                }
            }
            if val_row != 0 {
                previous_val_row = val_row;
            }
            if val_col != 0 {
                previous_val_col = val_col;
            }
        }
    }
    false
}

pub fn is_largest_tile_in_corner(board: Board, corner: Corner) -> bool {
    let mut largest_tile_idx = 0;
    let mut largest_tile_val = 0;
    let corner_idx = corner.get_idx();
    for idx in 0..16 {
        let tile_val = GameEngine::get_tile(board, idx);
        if tile_val > largest_tile_val {
            largest_tile_val = tile_val;
            largest_tile_idx = idx;
        }
        if tile_val == largest_tile_val && idx == corner_idx {
            largest_tile_val = tile_val;
            largest_tile_idx = idx;
        }
    }
    largest_tile_idx == corner_idx
}

pub fn is_column_monotonic(board: Board, column: Column) -> bool {
    let col_idx = column.get_idx();
    let tile1 = GameEngine::get_tile(board, 12 + col_idx);
    let tile2 = GameEngine::get_tile(board, 8 + col_idx);
    let tile3 = GameEngine::get_tile(board, 4 + col_idx);
    let tile4 = GameEngine::get_tile(board, 0 + col_idx);
    if tile1 == 0 && tile2 == 0 && tile3 == 0 && tile4 == 0 {
        return false;
    }
    if tile1 <= tile2 && tile2 <= tile3 && tile3 <= tile4 {
        return true;
    }
    if tile1 >= tile2 && tile2 >= tile3 && tile3 >= tile4 {
        return true;
    }
    false
}

pub fn is_row_monotonic(board: Board, row: Row) -> bool {
    let row_starting_idx = 4 * row.get_idx();
    let tile1 = GameEngine::get_tile(board, row_starting_idx);
    let tile2 = GameEngine::get_tile(board, row_starting_idx + 1);
    let tile3 = GameEngine::get_tile(board, row_starting_idx + 2);
    let tile4 = GameEngine::get_tile(board, row_starting_idx + 3);
    if tile1 == 0 && tile2 == 0 && tile3 == 0 && tile4 == 0 {
        return false;
    }
    if tile1 <= tile2 && tile2 <= tile3 && tile3 <= tile4 {
        return true;
    }
    if tile1 >= tile2 && tile2 >= tile3 && tile3 >= tile4 {
        return true;
    }
    false
}

pub fn are_2_largest_tiles_adjacent(board: Board) -> bool {
    // need a vec of all the largest tile idxs
    let mut largest_tile_idxs = Vec::new();
    let mut largest_tile_val = 0;
    for idx in 0..16 {
        let tile_val = GameEngine::get_tile(board, idx);
        if tile_val > largest_tile_val {
            largest_tile_val = tile_val;
            largest_tile_idxs = vec![idx];
        } else if tile_val == largest_tile_val {
            largest_tile_idxs.push(idx);
        }
    }
    let mut second_largest_tile_idxs = Vec::new();
    let mut second_largest_tile_val = 0;
    for idx in 0..16 {
        // skip the idx if it is one of the largest tiles
        if largest_tile_idxs.contains(&idx) {
            continue;
        }
        let tile_val = GameEngine::get_tile(board, idx);
        if tile_val > second_largest_tile_val {
            second_largest_tile_val = tile_val;
            second_largest_tile_idxs = vec![idx];
        } else if tile_val == second_largest_tile_val {
            second_largest_tile_idxs.push(idx);
        }
    }
    for largest_tile_idx in &largest_tile_idxs {
        for second_largest_tile_idx in &second_largest_tile_idxs {
            // if they are adjacent return true
            let is_adjacent = match largest_tile_idx {
                0 => match second_largest_tile_idx {
                    1 | 4 => true,
                    _ => false,
                },
                1 => match second_largest_tile_idx {
                    0 | 2 | 5 => true,
                    _ => false,
                },
                2 => match second_largest_tile_idx {
                    1 | 3 | 6 => true,
                    _ => false,
                },
                3 => match second_largest_tile_idx {
                    2 | 7 => true,
                    _ => false,
                },
                4 => match second_largest_tile_idx {
                    0 | 5 | 8 => true,
                    _ => false,
                },
                5 => match second_largest_tile_idx {
                    1 | 4 | 6 | 9 => true,
                    _ => false,
                },
                6 => match second_largest_tile_idx {
                    2 | 5 | 7 | 10 => true,
                    _ => false,
                },
                7 => match second_largest_tile_idx {
                    3 | 6 | 11 => true,
                    _ => false,
                },
                8 => match second_largest_tile_idx {
                    4 | 9 | 12 => true,
                    _ => false,
                },
                9 => match second_largest_tile_idx {
                    5 | 8 | 10 | 13 => true,
                    _ => false,
                },
                10 => match second_largest_tile_idx {
                    6 | 9 | 11 | 14 => true,
                    _ => false,
                },
                11 => match second_largest_tile_idx {
                    7 | 10 | 15 => true,
                    _ => false,
                },
                12 => match second_largest_tile_idx {
                    8 | 13 => true,
                    _ => false,
                },
                13 => match second_largest_tile_idx {
                    9 | 12 | 14 => true,
                    _ => false,
                },
                14 => match second_largest_tile_idx {
                    10 | 13 | 15 => true,
                    _ => false,
                },
                15 => match second_largest_tile_idx {
                    11 | 14 => true,
                    _ => false,
                },
                _ => false,
            };
            if is_adjacent {
                return true;
            }
        }
    }
    false
}

pub fn is_column_empty(board: Board, column: Column) -> bool {
    let col_idx = column.get_idx();
    let tile1 = GameEngine::get_tile(board, 12 + col_idx);
    let tile2 = GameEngine::get_tile(board, 8 + col_idx);
    let tile3 = GameEngine::get_tile(board, 4 + col_idx);
    let tile4 = GameEngine::get_tile(board, 0 + col_idx);
    if tile1 == 0 && tile2 == 0 && tile3 == 0 && tile4 == 0 {
        return true;
    }
    false
}

pub fn is_row_empty(board: Board, row: Row) -> bool {
    let row_starting_idx = 4 * row.get_idx();
    let tile1 = GameEngine::get_tile(board, row_starting_idx);
    let tile2 = GameEngine::get_tile(board, row_starting_idx + 1);
    let tile3 = GameEngine::get_tile(board, row_starting_idx + 2);
    let tile4 = GameEngine::get_tile(board, row_starting_idx + 3);
    if tile1 == 0 && tile2 == 0 && tile3 == 0 && tile4 == 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_2_largest_adjacent() {
        assert_eq!(are_2_largest_tiles_adjacent(0x2222111100000000), true);
        assert_eq!(are_2_largest_tiles_adjacent(0x0000000010002000), true);
        assert_eq!(are_2_largest_tiles_adjacent(0x3200111111111111), true);
        assert_eq!(are_2_largest_tiles_adjacent(0x3000111120001111), false);
        assert_eq!(are_2_largest_tiles_adjacent(0x3333000022220000), false);
    }

    #[test]
    fn it_is_left_column_monotonic() {
        assert_eq!(is_column_monotonic(0x1234123412341234, Column::Left), true);
        assert_eq!(is_column_monotonic(0x1000200030004000, Column::Left), true);
        assert_eq!(
            is_column_monotonic(0x0100020003000400, Column::MiddleLeft),
            true
        );
        assert_eq!(
            is_column_monotonic(0x0010002000300040, Column::MiddleRight),
            true
        );
        assert_eq!(is_column_monotonic(0x0001000200030004, Column::Right), true);
        assert_eq!(is_column_monotonic(0x4000300050006000, Column::Left), false);
        assert_eq!(is_column_monotonic(0x1000000000001000, Column::Left), false);
        assert_eq!(
            is_column_monotonic(0x0100000000000100, Column::MiddleLeft),
            false
        );
        assert_eq!(
            is_column_monotonic(0x0010000000000010, Column::MiddleRight),
            false
        );
        assert_eq!(
            is_column_monotonic(0x0001000000000001, Column::Right),
            false
        );
    }

    #[test]
    fn it_is_largest_tile_in_corner() {
        assert_eq!(
            is_largest_tile_in_corner(0x0000000000003000, Corner::BottomLeft),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x0000000020002000, Corner::BottomLeft),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x0000000000002222, Corner::BottomLeft),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x5000500012343333, Corner::TopLeft),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x1238281200000000, Corner::TopRight),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x0000000045674567, Corner::BottomRight),
            true
        );
        assert_eq!(
            is_largest_tile_in_corner(0x1234123412341523, Corner::BottomLeft),
            false
        );
        assert_eq!(
            is_largest_tile_in_corner(0x0000000000002222, Corner::TopLeft),
            false
        );
        assert_eq!(
            is_largest_tile_in_corner(0x0000000000002222, Corner::TopRight),
            false
        );
        assert_eq!(
            is_largest_tile_in_corner(0x222200000000000, Corner::BottomRight),
            false
        );
    }

    #[test]
    fn it_is_move_possible() {
        let engine = GameEngine::new();
        assert_eq!(
            is_move_possible(&engine, 0x1111222233334444, Move::Left),
            true
        );
        assert_eq!(
            is_move_possible(&engine, 0x1234123412341234, Move::Left),
            false
        );
        assert_eq!(
            is_move_possible(&engine, 0x1111123412341234, Move::Right),
            true
        );
        assert_eq!(
            is_move_possible(&engine, 0x1234123412341234, Move::Right),
            false
        );
        assert_eq!(
            is_move_possible(&engine, 0x1111123423452345, Move::Down),
            true
        );
        assert_eq!(
            is_move_possible(&engine, 0x1111123423452345, Move::Up),
            true
        );
        assert_eq!(
            is_move_possible(&engine, 0x1234432112344321, Move::Up),
            false
        );
        assert_eq!(
            is_move_possible(&engine, 0x1234432112344321, Move::Down),
            false
        );
    }

    #[test]
    fn it_is_column_locked() {
        assert_eq!(is_column_locked(0x1000100010001000, Column::Left), false);
        assert_eq!(
            is_column_locked(0x0200020003000400, Column::MiddleLeft),
            false
        );
        assert_eq!(
            is_column_locked(0x0050006000800080, Column::MiddleRight),
            false
        );
        assert_eq!(is_column_locked(0x0001000100010002, Column::Right), false);
        assert_eq!(is_column_locked(0x1000000020003000, Column::Left), false);
        assert_eq!(is_column_locked(0x4000300020001000, Column::Left), true);
        assert_eq!(
            is_column_locked(0x0500020003000400, Column::MiddleLeft),
            true
        );
        assert_eq!(
            is_column_locked(0x0050006000900080, Column::MiddleRight),
            true
        );
        assert_eq!(is_column_locked(0x0001000700010002, Column::Right), true);
    }

    #[test]
    fn it_is_row_locked() {
        assert_eq!(is_row_locked(0x1111000000000000, Row::Top), false);
        assert_eq!(is_row_locked(0x0000223400000000, Row::MiddleTop), false);
        assert_eq!(is_row_locked(0x0000000056880000, Row::MiddleBottom), false);
        assert_eq!(is_row_locked(0x0000000000001511, Row::Bottom), false);
        assert_eq!(is_row_locked(0x0234000000000000, Row::Top), false);
        assert_eq!(is_row_locked(0x1234000000000000, Row::Top), true);
        assert_eq!(is_row_locked(0x0000293400000000, Row::MiddleTop), true);
        assert_eq!(is_row_locked(0x0000000056980000, Row::MiddleBottom), true);
        assert_eq!(is_row_locked(0x0000000000001512, Row::Bottom), true);
    }

    #[test]
    fn it_is_merge_possible() {
        assert_eq!(is_merge_possible(0x1111222233334444, Move::Left), true);
        assert_eq!(is_merge_possible(0x1234123412341234, Move::Left), false);
        assert_eq!(is_merge_possible(0x0000003440052424, Move::Left), false);
        assert_eq!(is_merge_possible(0x1111222233334444, Move::Right), true);
        assert_eq!(is_merge_possible(0x1234123412341234, Move::Right), false);
        assert_eq!(is_merge_possible(0x0000003440052424, Move::Right), false);
        assert_eq!(is_merge_possible(0x1234123412341234, Move::Up), true);
        assert_eq!(is_merge_possible(0x1111222233334444, Move::Up), false);
        assert_eq!(is_merge_possible(0x0000205450262035, Move::Up), false);
        assert_eq!(is_merge_possible(0x1234123412341234, Move::Down), true);
        assert_eq!(is_merge_possible(0x1111222233334444, Move::Down), false);
        assert_eq!(is_merge_possible(0x0000205450262035, Move::Down), false);
        assert_eq!(is_merge_possible(0x2100101052008531, Move::Left), true);
    }
}
