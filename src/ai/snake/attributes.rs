use crate::engine as GameEngine;
use crate::engine::Move;

pub fn is_move_possible(board: GameEngine::Board, direction: Move) -> bool {
    board != GameEngine::shift(board, direction)
}

pub fn is_column_locked(board: GameEngine::Board, col_idx: usize) -> bool {
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

pub fn is_row_locked(board: GameEngine::Board, row_idx: usize) -> bool {
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

pub fn is_merge_possible(board: GameEngine::Board, direction: Move) -> bool {
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
            previous_val_row = val_row;
            previous_val_col = val_col;
        }
    }
    false
}

pub fn does_move_produce_merge_in_direction(
    board: GameEngine::Board,
    direction: Move,
    merge_direction: Move,
) -> bool {
    let new_board = GameEngine::shift(board, direction);
    if board == new_board {
        return false;
    }
    is_merge_possible(new_board, merge_direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_move_possible() {
        GameEngine::create_stores();
        assert_eq!(is_move_possible(0x1111222233334444, Move::Left), true);
        assert_eq!(is_move_possible(0x1234123412341234, Move::Left), false);
        assert_eq!(is_move_possible(0x1111123412341234, Move::Right), true);
        assert_eq!(is_move_possible(0x1234123412341234, Move::Right), false);
        assert_eq!(is_move_possible(0x1111123423452345, Move::Down), true);
        assert_eq!(is_move_possible(0x1111123423452345, Move::Up), true);
        assert_eq!(is_move_possible(0x1234432112344321, Move::Up), false);
        assert_eq!(is_move_possible(0x1234432112344321, Move::Down), false);
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
    }

    #[test]
    fn it_does_move_produce_merge_in_direction() {
        GameEngine::create_stores();
        assert_eq!(
            does_move_produce_merge_in_direction(0x1234234000000000, Move::Right, Move::Up),
            true
        );
    }
}
