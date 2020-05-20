use bitintr::Pext;
use bitintr::Popcnt;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

struct Stores {
    move_left: [u64; 0xffff],
    move_right: [u64; 0xffff],
    move_up: [u64; 0xffff],
    move_down: [u64; 0xffff],
    score: [f64; 0xffff],
}

static mut STORES: Stores = Stores {
    move_left: [0; 0xffff],
    move_right: [0; 0xffff],
    move_up: [0; 0xffff],
    move_down: [0; 0xffff],
    score: [0.; 0xffff],
};

unsafe fn create_stores() {
    let mut val = 0;
    while val < 0xffff {
        STORES.move_left[val] = shift_left_or_right(val as u64, Move::Left);
        STORES.move_right[val] = shift_left_or_right(val as u64, Move::Right);

        STORES.move_up[val] = shift_up_or_down(val as u64, Move::Up);
        STORES.move_down[val] = shift_up_or_down(val as u64, Move::Down);

        STORES.score[val] = calc_score(val as u64);

        val += 1;
    }
}

pub type Board = u64;

pub fn new_game() -> Board {
    unsafe {
        create_stores();
    }
    start_new_game()
}

pub fn start_new_game() -> Board {
    let board = 0;
    let board = generate_random_tile(board);
    generate_random_tile(board)
}

pub fn update_state_by_idx(board: Board, idx: usize, new_value: u64) -> Board {
    let shift_amount = (15 - idx) * 4;
    (board & !(0xf << shift_amount)) | (new_value << shift_amount)
}

pub fn move_left(board: Board) -> Board {
    execute_move(board, Move::Left)
}

pub fn move_right(board: Board) -> Board {
    execute_move(board, Move::Right)
}

pub fn move_up(board: Board) -> Board {
    execute_move(board, Move::Up)
}

pub fn move_down(board: Board) -> Board {
    execute_move(board, Move::Down)
}

fn execute_move(board: Board, dir: Move) -> Board {
    let mut new_board;
    match dir {
        Move::Left | Move::Right => new_board = move_left_or_right(board, dir),
        Move::Up | Move::Down => new_board = move_up_or_down(board, dir),
    }
    if board != new_board {
        new_board = generate_random_tile(new_board)
    }
    new_board
}

pub fn move_left_or_right(board: Board, move_dir: Move) -> Board {
    let mut new_board = 0;
    for row_idx in 0..4 {
        let row_val = extract_row(board, row_idx);
        let new_row_val = match move_dir {
            Move::Left => unsafe { STORES.move_left.get_unchecked(row_val as usize) },
            Move::Right => unsafe { STORES.move_right.get_unchecked(row_val as usize) },
            _ => panic!("Trying to move up or down in move_left_or_right"),
        };
        //match new_row_val {
        //Some(value) => new_board = new_board | (value << (48 - (16 * row_idx))),
        new_board = new_board | (new_row_val << (48 - (16 * row_idx)));
        //   None => panic!(format!("The row: {} was not found in the stores", row_val)),
        //}
    }
    new_board
}

pub fn move_up_or_down(board: Board, move_dir: Move) -> Board {
    let mut new_board = 0;
    let transpose_board = transpose(board);
    for col_idx in 0..4 {
        let col_val = extract_row(transpose_board, col_idx);
        let new_col_val = match move_dir {
            Move::Up => unsafe { STORES.move_up.get_unchecked(col_val as usize) },
            Move::Down => unsafe { STORES.move_down.get_unchecked(col_val as usize) },
            _ => panic!("Trying to move left or right in move up or down"),
        };
        //match new_col_val {
        //    Some(value) => new_board = new_board | (value << (12 - (4 * col_idx))),
        //    None => panic!(format!("The col: {} was not found in the stores", col_val)),
        //}
        new_board = new_board | (new_col_val << (12 - (4 * col_idx)))
    }
    new_board
}

fn generate_random_tile(board: Board) -> Board {
    let zero_tiles = get_empty_tile_idxs(board);
    let num_zero_tiles = zero_tiles.len();
    if num_zero_tiles == 0 {
        return board;
    }
    let mut rng = rand::thread_rng();
    let rand_idx = rng.gen_range(0, zero_tiles.len());
    let rand_val = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
    update_state_by_idx(board, zero_tiles[rand_idx], rand_val)
}

pub fn get_empty_tile_idxs(board: Board) -> Vec<usize> {
    (0..16).fold(Vec::new(), |mut vec, idx| {
        let tile_val = extract_tile(board, idx);
        if tile_val == 0 {
            vec.push(idx)
        };
        vec
    })
}

fn to_vec(board: Board) -> Vec<Option<u64>> {
    (0..16).fold(Vec::new(), |mut vec, idx| {
        let num = extract_tile(board, idx);

        if num == 0 {
            vec.push(None)
        } else {
            vec.push(Some((2 as u64).pow(num as u32)))
        }

        vec
    })
}

pub fn get_score(board: Board) -> f64 {
    let transpose_board = transpose(board);
    (0..4).fold(0., |acc, idx| {
        let row_val = extract_row(board, idx);
        let col_val = extract_row(transpose_board, idx);
        let row_score;
        let col_score;
        unsafe {
            row_score = STORES.score.get_unchecked(row_val as usize);
            col_score = STORES.score.get_unchecked(col_val as usize);
        }
        //match row_score {
        //    Some(row_score_val) => match col_score {
        //        Some(col_score_val) => acc + row_score_val + col_score_val,
        //        None => panic!("Could not find col value in store"),
        //    },
        //    None => panic!("Could not find row value in store"),
        //}
        acc + row_score + col_score
    })
}

fn shift_left_or_right(row: u64, direction: Move) -> u64 {
    let mut tiles = row_to_vec(row);
    match direction {
        Move::Left => tiles = shift_vec_left(tiles),
        Move::Right => tiles = shift_vec_right(tiles),
        _ => panic!("trying to shift up or down in shift left or right"),
    }
    tiles[0] <<= 12;
    tiles[1] <<= 8;
    tiles[2] <<= 4;
    tiles[0] | tiles[1] | tiles[2] | tiles[3]
}

fn shift_up_or_down(col: u64, direction: Move) -> u64 {
    let mut tiles = row_to_vec(col);
    match direction {
        Move::Up => tiles = shift_vec_left(tiles),
        Move::Down => tiles = shift_vec_right(tiles),
        _ => panic!("trying to left or right in shift up or down"),
    }
    tiles[0] <<= 48;
    tiles[1] <<= 32;
    tiles[2] <<= 16;
    tiles[0] | tiles[1] | tiles[2] | tiles[3]
}

fn row_to_vec(row: u64) -> Vec<u64> {
    let tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
        tiles.push(row >> ((3 - tile_idx) * 4) & 0xf);
        tiles
    });
    tiles
}

fn extract_tile(board: Board, idx: usize) -> u64 {
    (board >> ((15 - idx) * 4)) & 0xf
}

fn transpose(x: Board) -> Board {
    let a1 = x & 0xF0F00F0FF0F00F0F;
    let a2 = x & 0x0000F0F00000F0F0;
    let a3 = x & 0x0F0F00000F0F0000;
    let a = a1 | (a2 << 12) | (a3 >> 12);
    let b1 = a & 0xFF00FF0000FF00FF;
    let b2 = a & 0x00FF00FF00000000;
    let b3 = a & 0x00000000FF00FF00;
    return b1 | (b2 >> 24) | (b3 << 24);
}

fn extract_row(board: Board, row_num: u64) -> u64 {
    (board >> ((3 - row_num) * 16)) & 0xffff
}

fn extract_col(board: Board, col_num: u64) -> u64 {
    board.pext(0xf000f000f000f000 >> (col_num * 4))
}

// https://stackoverflow.com/questions/38225571/count-number-of-zero-nibbles-in-an-unsigned-64-bit-integer
pub fn count_empty(board: Board) -> u64 {
    16 - count_non_empty(board)
}

pub fn count_non_empty(board: Board) -> u64 {
    let mut board_copy = board;
    board_copy |= board_copy >> 1;
    board_copy |= board_copy >> 2;
    board_copy &= 0x1111111111111111;
    board_copy.popcnt()
}

pub fn count_unique(board: Board) -> i32 {
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

// The heuristics developed by Nneonneo were used: https://github.com/nneonneo/2048-ai/blob/master/2048.cpp
fn calc_score(row: u64) -> f64 {
    const LOST_PENALTY: f64 = 200000.;
    let tiles = row_to_vec(row);
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

pub fn is_game_over(board: Board) -> bool {
    for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
        let new_board;
        match direction {
            Move::Up => new_board = move_up_or_down(board, Move::Up),
            Move::Down => new_board = move_up_or_down(board, Move::Down),
            Move::Left => new_board = move_left_or_right(board, Move::Left),
            Move::Right => new_board = move_left_or_right(board, Move::Right),
        }
        if new_board != board {
            return false;
        }
    }
    true
}

pub fn to_str(board: Board) -> String {
    let board: Vec<_> = to_vec(board).iter().map(|x| format_val(x)).collect();
    format!(
        "\r
        {}|{}|{}|{}\r
        --------------------------------\r
        {}|{}|{}|{}\r
        --------------------------------\r
        {}|{}|{}|{}\r
        --------------------------------\r
        {}|{}|{}|{}\r
        ",
        board[0],
        board[1],
        board[2],
        board[3],
        board[4],
        board[5],
        board[6],
        board[7],
        board[8],
        board[9],
        board[10],
        board[11],
        board[12],
        board[13],
        board[14],
        board[15]
    )
}

fn format_val(val: &Option<u64>) -> String {
    match val {
        None => return String::from("       "),
        Some(x) => {
            let mut x = x.to_string();
            while x.len() < 7 {
                match x.len() {
                    6 => x = format!(" {}", x),
                    _ => x = format!(" {} ", x),
                }
            }
            x
        }
    }
}

fn shift_vec_right(vec: Vec<u64>) -> Vec<u64> {
    let rev_vec: Vec<u64> = vec.into_iter().rev().collect();
    shift_vec_left(rev_vec).iter().rev().map(|&x| x).collect()
}

fn shift_vec_left(mut vec: Vec<u64>) -> Vec<u64> {
    for i in 0..4 {
        calculate_left_shift(&mut vec[i..]);
    }
    vec
}

fn calculate_left_shift(slice: &mut [u64]) {
    let mut acc = 0;
    for idx in 0..slice.len() {
        let val = slice[idx];
        if acc != 0 && acc == val {
            slice[idx] = 0;
            acc += 1;
            break;
        } else if acc != 0 && val != 0 && acc != val {
            break;
        } else if acc == 0 && val != 0 {
            slice[idx] = 0;
            acc = val;
        };
    }
    slice[0] = acc;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shift_vec_left() {
        assert_eq!(shift_vec_left(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
        assert_eq!(shift_vec_left(vec![1, 2, 1, 2]), vec![1, 2, 1, 2]);
        assert_eq!(shift_vec_left(vec![1, 1, 2, 2]), vec![2, 3, 0, 0]);
        assert_eq!(shift_vec_left(vec![1, 0, 0, 1]), vec![2, 0, 0, 0]);
    }

    #[test]
    fn it_shift_vec_right() {
        assert_eq!(shift_vec_right(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
        assert_eq!(shift_vec_right(vec![1, 2, 1, 2]), vec![1, 2, 1, 2]);
        assert_eq!(shift_vec_right(vec![1, 1, 2, 2]), vec![0, 0, 2, 3]);
        assert_eq!(shift_vec_right(vec![5, 0, 0, 5]), vec![0, 0, 0, 6]);
        assert_eq!(shift_vec_right(vec![0, 2, 2, 2]), vec![0, 0, 2, 3]);
    }

    #[test]
    fn it_test_update_state_by_idx() {
        let game = 0;
        let game = update_state_by_idx(game, 15, 2);
        let game = update_state_by_idx(game, 12, 3);
        assert_eq!(game, 0x0000000000003002);
    }

    #[test]
    fn it_test_get_empty_tile_idxs() {
        let game = 0x0000111122220000;
        assert_eq!(get_empty_tile_idxs(game), vec![0, 1, 2, 3, 12, 13, 14, 15]);
    }

    #[test]
    fn it_test_generate_random_tile() {
        let mut game = 0;
        for _ in 0..16 {
            game = generate_random_tile(game);
        }
        assert_eq!(get_empty_tile_idxs(game).len(), 0);
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(shift_left_or_right(0x0000, Move::Left), 0x0000);
        assert_eq!(shift_left_or_right(0x0002, Move::Left), 0x2000);
        assert_eq!(shift_left_or_right(0x2020, Move::Left), 0x3000);
        assert_eq!(shift_left_or_right(0x1332, Move::Left), 0x1420);
        assert_eq!(shift_left_or_right(0x1234, Move::Left), 0x1234);
        assert_eq!(shift_left_or_right(0x1002, Move::Left), 0x1200);
        assert_ne!(shift_left_or_right(0x1210, Move::Left), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(shift_left_or_right(0x0000, Move::Right), 0x0000);
        assert_eq!(shift_left_or_right(0x2000, Move::Right), 0x0002);
        assert_eq!(shift_left_or_right(0x2020, Move::Right), 0x0003);
        assert_eq!(shift_left_or_right(0x1332, Move::Right), 0x0142);
        assert_eq!(shift_left_or_right(0x1234, Move::Right), 0x1234);
        assert_eq!(shift_left_or_right(0x1002, Move::Right), 0x0012);
        assert_ne!(shift_left_or_right(0x0121, Move::Right), 0x0022);
    }

    #[test]
    fn test_move_left() {
        new_game();
        let game = 0x1234133220021002;
        let game = move_left_or_right(game, Move::Left);
        assert_eq!(game, 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        new_game();
        let game = 0x1121230033004222;
        let game = move_up_or_down(game, Move::Up);
        assert_eq!(game, 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        new_game();
        let game = 0x1234133220021002;
        let game = move_left_or_right(game, Move::Right);
        assert_eq!(game, 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        new_game();
        let game = 0x1121230033004222;
        let game = move_up_or_down(game, Move::Down);
        assert_eq!(game, 0x1000210034014232);
    }

    #[test]
    fn it_count_empty() {
        let game = 0x1111000011110000;
        assert_eq!(count_empty(game), 8);
        assert_eq!(game, 0x1111000011110000);
        let game = 0x1100000000000000;
        assert_eq!(count_empty(game), 14);
        assert_eq!(game, 0x1100000000000000);
    }

    #[test]
    fn it_calc_score() {
        assert_eq!(calc_score(0x1100), 201918.);
        assert_eq!(
            calc_score(0x4321),
            200000.
                - (11. * ((4 as f64).powf(3.5) + (3 as f64).powf(3.5) + (2 as f64).powf(3.5) + 1.))
        );
    }

    #[test]
    fn it_count_non_empty() {
        let game = 0x1134000000000000;
        assert_eq!(count_non_empty(game), 4);
    }

    #[test]
    fn it_count_unique() {
        let game = 0x1134000000000000;
        assert_eq!(count_unique(game), 3);
        let game = 0x0000010000000010;
        assert_eq!(count_unique(game), 1);
    }
}
