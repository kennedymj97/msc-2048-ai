use bitintr::Popcnt;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Stores {
    move_left: Vec<u64>,
    move_right: Vec<u64>,
    move_up: Vec<u64>,
    move_down: Vec<u64>,
    score: Vec<u64>,
}

#[derive(Clone)]
pub struct GameEngine {
    board: u64,
    stores: Rc<Stores>,
}

impl GameEngine {
    pub fn new() -> Self {
        let stores = GameEngine::create_stores();
        let mut game = GameEngine {
            board: 0,
            stores: Rc::new(stores),
        };
        game.generate_random_tile();
        game.generate_random_tile();
        game
    }

    pub fn get_state(&self) -> u64 {
        self.board
    }

    pub fn update_state(&mut self, new_state: u64) {
        self.board = new_state;
    }

    pub fn update_state_by_idx(&mut self, idx: usize, new_value: u64) {
        let shift_amount = (15 - idx) * 4;
        self.board = (self.get_state() & !(0xf << shift_amount)) | (new_value << shift_amount);
    }

    pub fn move_left(&mut self) {
        self.execute_move(Move::Left);
    }

    pub fn move_right(&mut self) {
        self.execute_move(Move::Right);
    }

    pub fn move_up(&mut self) {
        self.execute_move(Move::Up);
    }

    pub fn move_down(&mut self) {
        self.execute_move(Move::Down);
    }

    fn execute_move(&mut self, dir: Move) {
        let old_state = self.get_state();
        match dir {
            Move::Left | Move::Right => self.move_left_or_right(dir),
            Move::Up | Move::Down => self.move_up_or_down(dir),
        }
        let new_state = self.get_state();
        if old_state != new_state {
            self.generate_random_tile();
        }
    }

    pub fn move_left_or_right(&mut self, move_dir: Move) {
        let mut new_board: u64 = 0;
        for row_idx in 0..4 {
            let row_val = self.extract_row(row_idx);
            let new_row_val = match move_dir {
                Move::Left => self.stores.move_left.get(row_val as usize),
                Move::Right => self.stores.move_right.get(row_val as usize),
                _ => panic!("Trying to move up or down in move_left_or_right"),
            };
            match new_row_val {
                Some(value) => new_board = new_board | (value << (48 - (16 * row_idx))),
                None => panic!(format!("The row: {} was not found in the stores", row_val)),
            }
        }
        self.update_state(new_board);
    }

    pub fn move_up_or_down(&mut self, move_dir: Move) {
        let mut new_board: u64 = 0;
        for col_idx in 0..4 {
            let col_val = self.extract_col(col_idx);
            let new_col_val = match move_dir {
                Move::Up => self.stores.move_up.get(col_val as usize),
                Move::Down => self.stores.move_down.get(col_val as usize),
                _ => panic!("Trying to move left or right in move up or down"),
            };
            match new_col_val {
                Some(value) => new_board = new_board | (value << (12 - (4 * col_idx))),
                None => panic!(format!("The col: {} was not found in the stores", col_val)),
            }
        }
        self.update_state(new_board);
    }

    fn generate_random_tile(&mut self) {
        let zero_tiles = self.get_empty_tile_idxs();
        let num_zero_tiles = zero_tiles.len();
        if num_zero_tiles == 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, zero_tiles.len());
        let rand_val = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
        self.update_state_by_idx(zero_tiles[rand_idx], rand_val);
    }

    pub fn get_empty_tile_idxs(&self) -> Vec<usize> {
        (0..16).fold(Vec::new(), |mut vec, idx| {
            let tile_val = self.extract_tile(idx);
            if tile_val == 0 {
                vec.push(idx)
            };
            vec
        })
    }

    fn to_vec(&self) -> Vec<Option<u64>> {
        (0..16).fold(Vec::new(), |mut vec, idx| {
            let num = self.extract_tile(idx);

            if num == 0 {
                vec.push(None)
            } else {
                vec.push(Some((2 as u64).pow(num as u32)))
            }

            vec
        })
    }

    pub fn get_score(&self) -> u64 {
        (0..4).fold(0, |acc, idx| {
            let row_val = self.extract_row(idx);
            let col_val = self.extract_col(idx);
            let row_score = self.stores.score.get(row_val as usize);
            let col_score = self.stores.score.get(col_val as usize);
            match row_score {
                Some(row_score_val) => match col_score {
                    Some(col_score_val) => acc + row_score_val + col_score_val,
                    None => panic!("Could not find col value in store"),
                },
                None => panic!("Could not find row value in store"),
            }
        })
    }

    fn shift_left_or_right(row: u64, direction: Move) -> u64 {
        let mut tiles = GameEngine::row_to_vec(row);
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
        let mut tiles = GameEngine::row_to_vec(col);
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

    fn extract_tile(&self, idx: usize) -> u64 {
        (self.get_state() >> ((15 - idx) * 4)) & 0xf
    }

    fn extract_row(&self, row_num: u64) -> u64 {
        (self.get_state() >> ((3 - row_num) * 16)) & 0xffff
    }

    fn extract_col(&self, col_num: u64) -> u64 {
        let board = self.get_state();
        let tile1 = (board >> (48 - (4 * col_num))) & 0xf000;
        let tile2 = (board >> (36 - (4 * col_num))) & 0x0f00;
        let tile3 = (board >> (24 - (4 * col_num))) & 0x00f0;
        let tile4 = (board >> (12 - (4 * col_num))) & 0x000f;
        tile1 | tile2 | tile3 | tile4
    }

    pub fn count_empty(&self) -> u64 {
        let mut board = self.get_state();
        board |= board >> 1;
        board |= board >> 2;
        board &= 0x1111111111111111;
        return board.popcnt();
    }

    fn create_stores() -> Stores {
        let mut move_left_store = Vec::new();
        let mut move_right_store = Vec::new();
        let mut move_up_store = Vec::new();
        let mut move_down_store = Vec::new();
        let mut score_store = Vec::new();

        for val in 0..0xffff {
            move_left_store.insert(
                val as usize,
                GameEngine::shift_left_or_right(val, Move::Left),
            );
            move_right_store.insert(
                val as usize,
                GameEngine::shift_left_or_right(val, Move::Right),
            );

            move_up_store.insert(val as usize, GameEngine::shift_up_or_down(val, Move::Up));
            move_down_store.insert(val as usize, GameEngine::shift_up_or_down(val, Move::Down));

            score_store.insert(val as usize, GameEngine::calc_score(val));
        }

        Stores {
            move_left: move_left_store,
            move_right: move_right_store,
            move_up: move_up_store,
            move_down: move_down_store,
            score: score_store,
        }
    }

    fn calc_score(row: u64) -> u64 {
        let tiles = GameEngine::row_to_vec(row);
        tiles.iter().fold(0, |acc, tile_val| acc + tile_val.pow(2))
    }

    pub fn is_game_over(&mut self) -> bool {
        for direction in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
            let mut engine_copy = self.clone();
            let old_state = engine_copy.get_state();
            match direction {
                Move::Up => engine_copy.move_up_or_down(Move::Up),
                Move::Down => engine_copy.move_up_or_down(Move::Down),
                Move::Left => engine_copy.move_left_or_right(Move::Left),
                Move::Right => engine_copy.move_left_or_right(Move::Right),
            }
            let new_state = engine_copy.get_state();
            if old_state != new_state {
                return false;
            }
        }
        true
    }

    fn to_str(&self) -> String {
        let board: Vec<_> = self.to_vec().iter().map(|x| format_val(x)).collect();
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
}

impl fmt::Display for GameEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
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
        let mut game = GameEngine::new();
        game.update_state(0);
        game.update_state_by_idx(15, 2);
        game.update_state_by_idx(12, 3);
        assert_eq!(game.get_state(), 0x0000000000003002);
    }

    #[test]
    fn it_test_get_empty_tile_idxs() {
        let mut game = GameEngine::new();
        game.update_state(0x0000111122220000);
        assert_eq!(game.get_empty_tile_idxs(), vec![0, 1, 2, 3, 12, 13, 14, 15]);
    }

    #[test]
    fn it_test_generate_random_tile() {
        let mut game = GameEngine::new();
        game.update_state(0);
        for _ in 0..16 {
            game.generate_random_tile();
        }
        assert_eq!(game.get_empty_tile_idxs().len(), 0);
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(GameEngine::shift_left_or_right(0x0000, Move::Left), 0x0000);
        assert_eq!(GameEngine::shift_left_or_right(0x0002, Move::Left), 0x2000);
        assert_eq!(GameEngine::shift_left_or_right(0x2020, Move::Left), 0x3000);
        assert_eq!(GameEngine::shift_left_or_right(0x1332, Move::Left), 0x1420);
        assert_eq!(GameEngine::shift_left_or_right(0x1234, Move::Left), 0x1234);
        assert_eq!(GameEngine::shift_left_or_right(0x1002, Move::Left), 0x1200);
        assert_ne!(GameEngine::shift_left_or_right(0x1210, Move::Left), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(GameEngine::shift_left_or_right(0x0000, Move::Right), 0x0000);
        assert_eq!(GameEngine::shift_left_or_right(0x2000, Move::Right), 0x0002);
        assert_eq!(GameEngine::shift_left_or_right(0x2020, Move::Right), 0x0003);
        assert_eq!(GameEngine::shift_left_or_right(0x1332, Move::Right), 0x0142);
        assert_eq!(GameEngine::shift_left_or_right(0x1234, Move::Right), 0x1234);
        assert_eq!(GameEngine::shift_left_or_right(0x1002, Move::Right), 0x0012);
        assert_ne!(GameEngine::shift_left_or_right(0x0121, Move::Right), 0x0022);
    }

    #[test]
    fn test_move_left() {
        let mut game = GameEngine::new();
        game.update_state(0x1234133220021002);
        game.move_left_or_right(Move::Left);
        assert_eq!(game.get_state(), 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        let mut game = GameEngine::new();
        game.update_state(0x1121230033004222);
        game.move_up_or_down(Move::Up);
        assert_eq!(game.get_state(), 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        let mut game = GameEngine::new();
        game.update_state(0x1234133220021002);
        game.move_left_or_right(Move::Right);
        assert_eq!(game.get_state(), 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        let mut game = GameEngine::new();
        game.update_state(0x1121230033004222);
        game.move_up_or_down(Move::Down);
        assert_eq!(game.get_state(), 0x1000210034014232);
    }

    #[test]
    fn it_count_empty() {
        let mut game = GameEngine::new();
        game.update_state(0x1111000011110000);
        assert_eq!(game.count_empty(), 8);
        assert_eq!(game.get_state(), 0x1111000011110000);
    }
}
