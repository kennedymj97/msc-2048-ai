use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;

pub fn is_move_possible(engine: &GameEngine, board: Board, direction: Move) -> bool {
    let new_board = engine.shift(board, direction);
    board != new_board
}

pub fn is_column_locked(board: Board, col_idx: usize) -> bool {
    let mut previous_val = 0;
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

pub fn is_row_locked(board: Board, row_idx: usize) -> bool {
    let mut previous_val = 0;
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

pub fn does_move_produce_merge_in_direction(
    engine: &GameEngine,
    board: Board,
    direction: Move,
    merge_direction: Move,
) -> bool {
    let new_board = engine.shift(board, direction);
    if board == new_board {
        return false;
    }
    is_merge_possible(new_board, merge_direction)
}

pub fn is_largest_tile_in_corner(board: Board) -> bool {
    let mut largest_tile_idx = 0;
    let mut largest_tile_val = 0;
    for idx in 0..16 {
        let tile_val = GameEngine::get_tile(board, idx);
        if tile_val > largest_tile_val {
            largest_tile_val = tile_val;
            largest_tile_idx = idx;
        }
        if tile_val == largest_tile_val && idx == 12 {
            largest_tile_val = tile_val;
            largest_tile_idx = idx;
        }
    }
    largest_tile_idx == 12
}

pub fn is_left_column_monotonic(board: Board) -> bool {
    let tile1 = GameEngine::get_tile(board, 12);
    let tile2 = GameEngine::get_tile(board, 8);
    let tile3 = GameEngine::get_tile(board, 4);
    let tile4 = GameEngine::get_tile(board, 0);
    if tile1 == 0 && tile2 == 0 && tile3 == 0 && tile4 == 0 {
        return false;
    }
    if tile1 < tile2 {
        return false;
    }
    if tile2 < tile3 {
        return false;
    }
    if tile3 < tile4 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_left_column_monotonic() {
        assert_eq!(is_left_column_monotonic(0x1234123412341234), true);
        assert_eq!(is_left_column_monotonic(0x1000200030004000), true);
        assert_eq!(is_left_column_monotonic(0x4000300050006000), false);
        assert_eq!(is_left_column_monotonic(0x1000000000001000), false);
    }

    #[test]
    fn it_is_largest_tile_in_corner() {
        assert_eq!(is_largest_tile_in_corner(0x0000000000003000), true);
        assert_eq!(is_largest_tile_in_corner(0x0000000020002000), true);
        assert_eq!(is_largest_tile_in_corner(0x1234123412341523), false);
        assert_eq!(is_largest_tile_in_corner(0x0000000000002222), true);
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
        assert_eq!(is_column_locked(0x1000100010001000, 0), false);
        assert_eq!(is_column_locked(0x0200020003000400, 1), false);
        assert_eq!(is_column_locked(0x0050006000800080, 2), false);
        assert_eq!(is_column_locked(0x0001000100010002, 3), false);
        assert_eq!(is_column_locked(0x1000000020003000, 0), false);
        assert_eq!(is_column_locked(0x4000300020001000, 0), true);
        assert_eq!(is_column_locked(0x0500020003000400, 1), true);
        assert_eq!(is_column_locked(0x0050006000900080, 2), true);
        assert_eq!(is_column_locked(0x0001000700010002, 3), true);
    }

    #[test]
    fn it_is_row_locked() {
        assert_eq!(is_row_locked(0x1111000000000000, 0), false);
        assert_eq!(is_row_locked(0x0000223400000000, 1), false);
        assert_eq!(is_row_locked(0x0000000056880000, 2), false);
        assert_eq!(is_row_locked(0x0000000000001511, 3), false);
        assert_eq!(is_row_locked(0x0234000000000000, 0), false);
        assert_eq!(is_row_locked(0x1234000000000000, 0), true);
        assert_eq!(is_row_locked(0x0000293400000000, 1), true);
        assert_eq!(is_row_locked(0x0000000056980000, 2), true);
        assert_eq!(is_row_locked(0x0000000000001512, 3), true);
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

    #[test]
    fn it_does_move_produce_merge_in_direction() {
        let engine = GameEngine::new();
        assert_eq!(
            does_move_produce_merge_in_direction(
                &engine,
                0x1234234000000000,
                Move::Right,
                Move::Up
            ),
            true
        );
        assert_eq!(
            does_move_produce_merge_in_direction(&engine, 0x5320752186105400, Move::Up, Move::Left,),
            true
        );
    }
}
