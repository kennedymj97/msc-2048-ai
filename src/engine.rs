use bitintr::Popcnt;
use rand::Rng;
use std::fmt;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn iterator() -> impl Iterator<Item = Move> {
        [Move::Left, Move::Right, Move::Up, Move::Down]
            .iter()
            .copied()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Left => write!(f, "Left"),
            Move::Right => write!(f, "Right"),
            Move::Up => write!(f, "Up"),
            Move::Down => write!(f, "Down"),
        }
    }
}

pub type Board = u64;
type Line = u64;
type Tile = u64;
pub type Score = u64;

pub struct GameEngine {
    shift_left: [Board; 0xffff],
    shift_right: [Board; 0xffff],
    shift_up: [Board; 0xffff],
    shift_down: [Board; 0xffff],
    score: [Score; 0xffff],
}

impl GameEngine {
    pub fn new() -> Self {
        let mut game_engine = GameEngine {
            shift_left: [0; 0xffff],
            shift_right: [0; 0xffff],
            shift_up: [0; 0xffff],
            shift_down: [0; 0xffff],
            score: [0; 0xffff],
        };

        for val in 0..0xffff {
            game_engine.shift_left[val] = Self::shift_line(val as u64, Move::Left);
            game_engine.shift_right[val] = Self::shift_line(val as u64, Move::Right);

            game_engine.shift_up[val] = Self::shift_line(val as u64, Move::Up);
            game_engine.shift_down[val] = Self::shift_line(val as u64, Move::Down);

            game_engine.score[val] = Self::calc_score(val as u64);
        }

        game_engine
    }

    pub fn new_board() -> Board {
        let board = Self::insert_random_tile(0);
        Self::insert_random_tile(board)
    }

    pub fn get_score(&self, board: Board) -> Score {
        (0..4).fold(0, |acc, idx| {
            let row_val = Self::extract_line(board, idx);
            let row_score;
            unsafe {
                row_score = self.score.get_unchecked(row_val as usize);
            }
            acc + row_score
        })
    }

    pub fn make_move(&self, board: Board, direction: Move) -> Board {
        let new_board = self.shift(board, direction);
        if board != new_board {
            return Self::insert_random_tile(new_board);
        }
        board
    }

    pub fn shift(&self, board: Board, direction: Move) -> Board {
        match direction {
            Move::Left | Move::Right => self.shift_rows(board, direction),
            Move::Up | Move::Down => self.shift_cols(board, direction),
        }
    }

    fn shift_rows(&self, board: Board, move_dir: Move) -> Board {
        (0..4).fold(0, |new_board, row_idx| {
            let row_val = Self::extract_line(board, row_idx);
            let new_row_val = match move_dir {
                Move::Left => unsafe { self.shift_left.get_unchecked(row_val as usize) },
                Move::Right => unsafe { self.shift_right.get_unchecked(row_val as usize) },
                _ => panic!("Trying to move up or down in shift rows"),
            };
            new_board | (new_row_val << (48 - (16 * row_idx)))
        })
    }

    fn shift_cols(&self, board: Board, move_dir: Move) -> Board {
        let transpose_board = Self::transpose(board);
        (0..4).fold(0, |new_board, col_idx| {
            let col_val = Self::extract_line(transpose_board, col_idx);
            let new_col_val = match move_dir {
                Move::Up => unsafe { self.shift_up.get_unchecked(col_val as usize) },
                Move::Down => unsafe { self.shift_down.get_unchecked(col_val as usize) },
                _ => panic!("Trying to move left or right in shift cols"),
            };
            new_board | (new_col_val << (12 - (4 * col_idx)))
        })
    }

    // Credit to Nneonneo
    pub fn transpose(x: Board) -> Board {
        let a1 = x & 0xF0F00F0FF0F00F0F;
        let a2 = x & 0x0000F0F00000F0F0;
        let a3 = x & 0x0F0F00000F0F0000;
        let a = a1 | (a2 << 12) | (a3 >> 12);
        let b1 = a & 0xFF00FF0000FF00FF;
        let b2 = a & 0x00FF00FF00000000;
        let b3 = a & 0x00000000FF00FF00;
        return b1 | (b2 >> 24) | (b3 << 24);
    }

    pub fn extract_line(board: Board, line_idx: u64) -> Line {
        (board >> ((3 - line_idx) * 16)) & 0xffff
    }

    pub fn get_tile(board: Board, idx: usize) -> Tile {
        (board >> (60 - (4 * idx))) & 0xf
    }

    pub fn get_highest_tile_val(board: Board) -> Tile {
        let max_tile = (0..16)
            .map(|idx| Self::get_tile(board, idx))
            .max()
            .expect("Could not extract max tile");
        2_u64.pow(max_tile as u32)
    }

    pub fn get_tile_val(board: Board, idx: usize) -> u16 {
        2_u16.pow(((board >> (60 - (4 * idx))) & 0xf) as u32)
    }

    pub fn line_to_vec(line: Line) -> Vec<Tile> {
        let tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(line >> ((3 - tile_idx) * 4) & 0xf);
            tiles
        });
        tiles
    }

    pub fn is_game_over(&self, board: Board) -> bool {
        for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
            let new_board = self.shift(board, direction);
            if new_board != board {
                return false;
            }
        }
        true
    }

    // https://stackoverflow.com/questions/38225571/count-number-of-zero-nibbles-in-an-unsigned-64-bit-integer
    pub fn count_empty(board: Board) -> u64 {
        16 - Self::count_non_empty(board)
    }

    pub fn to_str(board: Board) -> String {
        let board: Vec<_> = Self::to_vec(board)
            .iter()
            .map(|x| Self::format_val(x))
            .collect();
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

    // Credit to Nneonneo
    fn insert_random_tile(board: Board) -> Board {
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0, Self::count_empty(board));
        let mut tmp = board;
        let mut tile = Self::generate_random_tile();
        loop {
            while (tmp & 0xf) != 0 {
                tmp >>= 4;
                tile <<= 4;
            }
            if index == 0 {
                break;
            }
            index -= 1;
            tmp >>= 4;
            tile <<= 4;
        }
        return board | tile;
    }

    fn generate_random_tile() -> Tile {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 10) < 9 {
            1
        } else {
            2
        }
    }

    fn shift_line(line: Line, direction: Move) -> Line {
        let tiles = Self::line_to_vec(line);
        match direction {
            Move::Left | Move::Right => Self::vec_to_row(Self::shift_vec(tiles, direction)),
            Move::Up | Move::Down => Self::vec_to_col(Self::shift_vec(tiles, direction)),
        }
    }

    fn vec_to_row(tiles: Vec<Tile>) -> Line {
        tiles[0] << 12 | tiles[1] << 8 | tiles[2] << 4 | tiles[3]
    }

    fn vec_to_col(tiles: Vec<Tile>) -> Line {
        tiles[0] << 48 | tiles[1] << 32 | tiles[2] << 16 | tiles[3]
    }

    fn shift_vec(vec: Vec<Tile>, direction: Move) -> Vec<Tile> {
        match direction {
            Move::Left | Move::Up => Self::shift_vec_left(vec),
            Move::Right | Move::Down => Self::shift_vec_right(vec),
        }
    }

    fn shift_vec_right(vec: Vec<Tile>) -> Vec<Tile> {
        let rev_vec: Vec<Tile> = vec.into_iter().rev().collect();
        Self::shift_vec_left(rev_vec)
            .iter()
            .rev()
            .map(|&x| x)
            .collect()
    }

    fn shift_vec_left(mut vec: Vec<Tile>) -> Vec<Tile> {
        for i in 0..4 {
            Self::calculate_left_shift(&mut vec[i..]);
        }
        vec
    }

    fn calculate_left_shift(slice: &mut [Tile]) {
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

    // Credit to Nneonneo
    fn calc_score(line: Line) -> Score {
        let mut score = 0;
        let tiles = Self::line_to_vec(line);
        for i in 0..4 {
            let tile_val = tiles[i];
            if tile_val >= 2 {
                // the score is the total sum of the tile and all intermediate merged tiles
                score += (tile_val - 1) * (1 << tile_val);
            }
        }
        score
    }

    fn count_non_empty(board: Board) -> u64 {
        let mut board_copy = board;
        board_copy |= board_copy >> 1;
        board_copy |= board_copy >> 2;
        board_copy &= 0x1111111111111111;
        board_copy.popcnt()
    }

    pub fn to_vec(board: Board) -> Vec<u8> {
        (0..16).fold(Vec::new(), |mut vec, idx| {
            let num = Self::extract_tile(board, idx);
            vec.push(num as u8);
            vec
        })
    }

    fn extract_tile(board: Board, idx: usize) -> Tile {
        (board >> ((15 - idx) * 4)) & 0xf
    }

    fn format_val(val: &u8) -> String {
        match val {
            0 => return String::from("       "),
            &x => {
                let mut x = (2_i32.pow(x as u32)).to_string();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shift_vec_left() {
        assert_eq!(
            GameEngine::shift_vec_left(vec![0, 0, 0, 0]),
            vec![0, 0, 0, 0]
        );
        assert_eq!(
            GameEngine::shift_vec_left(vec![1, 2, 1, 2]),
            vec![1, 2, 1, 2]
        );
        assert_eq!(
            GameEngine::shift_vec_left(vec![1, 1, 2, 2]),
            vec![2, 3, 0, 0]
        );
        assert_eq!(
            GameEngine::shift_vec_left(vec![1, 0, 0, 1]),
            vec![2, 0, 0, 0]
        );
    }

    #[test]
    fn it_shift_vec_right() {
        assert_eq!(
            GameEngine::shift_vec_right(vec![0, 0, 0, 0]),
            vec![0, 0, 0, 0]
        );
        assert_eq!(
            GameEngine::shift_vec_right(vec![1, 2, 1, 2]),
            vec![1, 2, 1, 2]
        );
        assert_eq!(
            GameEngine::shift_vec_right(vec![1, 1, 2, 2]),
            vec![0, 0, 2, 3]
        );
        assert_eq!(
            GameEngine::shift_vec_right(vec![5, 0, 0, 5]),
            vec![0, 0, 0, 6]
        );
        assert_eq!(
            GameEngine::shift_vec_right(vec![0, 2, 2, 2]),
            vec![0, 0, 2, 3]
        );
    }

    #[test]
    fn it_test_insert_random_tile() {
        let mut game = 0;
        for _ in 0..16 {
            game = GameEngine::insert_random_tile(game);
        }
        assert_eq!(GameEngine::count_empty(game), 0);
    }

    #[test]
    fn test_shift_left() {
        let engine = GameEngine::new();
        assert_eq!(engine.shift(0x0000, Move::Left), 0x0000);
        assert_eq!(engine.shift(0x0002, Move::Left), 0x2000);
        assert_eq!(engine.shift(0x2020, Move::Left), 0x3000);
        assert_eq!(engine.shift(0x1332, Move::Left), 0x1420);
        assert_eq!(engine.shift(0x1234, Move::Left), 0x1234);
        assert_eq!(engine.shift(0x1002, Move::Left), 0x1200);
        assert_ne!(engine.shift(0x1210, Move::Left), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        let engine = GameEngine::new();
        assert_eq!(engine.shift(0x0000, Move::Right), 0x0000);
        assert_eq!(engine.shift(0x2000, Move::Right), 0x0002);
        assert_eq!(engine.shift(0x2020, Move::Right), 0x0003);
        assert_eq!(engine.shift(0x1332, Move::Right), 0x0142);
        assert_eq!(engine.shift(0x1234, Move::Right), 0x1234);
        assert_eq!(engine.shift(0x1002, Move::Right), 0x0012);
        assert_ne!(engine.shift(0x0121, Move::Right), 0x0022);
    }

    #[test]
    fn test_move_left() {
        let engine = GameEngine::new();
        let game = 0x1234133220021002;
        let game = engine.shift(game, Move::Left);
        assert_eq!(game, 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        let engine = GameEngine::new();
        let game = 0x1121230033004222;
        let game = engine.shift(game, Move::Up);
        assert_eq!(game, 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        let engine = GameEngine::new();
        let game = 0x1234133220021002;
        let game = engine.shift(game, Move::Right);
        assert_eq!(game, 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        let engine = GameEngine::new();
        let game = 0x1121230033004222;
        let game = engine.shift(game, Move::Down);
        assert_eq!(game, 0x1000210034014232);
    }

    #[test]
    fn it_count_empty() {
        let game = 0x1111000011110000;
        assert_eq!(GameEngine::count_empty(game), 8);
        assert_eq!(game, 0x1111000011110000);
        let game = 0x1100000000000000;
        assert_eq!(GameEngine::count_empty(game), 14);
        assert_eq!(game, 0x1100000000000000);
    }

    #[test]
    fn it_count_non_empty() {
        let game = 0x1134000000000000;
        assert_eq!(GameEngine::count_non_empty(game), 4);
    }

    #[test]
    fn it_get_tile_val() {
        let game = 0x123456789abcdef;
        assert_eq!(GameEngine::get_tile_val(game, 3), 8);
        assert_eq!(GameEngine::get_tile_val(game, 10), 1024);
        assert_eq!(GameEngine::get_tile_val(game, 15), 32768);
    }
}
