use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::engine::*;

type State = u64;

#[derive(Clone)]
struct Stores {
    move_left: HashMap<u64, u64>,
    move_right: HashMap<u64, u64>,
    move_up: HashMap<u64, u64>,
    move_down: HashMap<u64, u64>,
}

#[derive(Clone)]
pub struct Optimised {
    state: State,
    stores: Rc<Stores>,
}

impl GameEngine for Optimised {
    type Board = State;

    fn new() -> Self {
        let stores = Optimised::create_stores();
        let mut game = Optimised {
            state: 0,
            stores: Rc::new(stores),
        };
        game.generate_random_tile();
        game.generate_random_tile();
        game
    }

    fn get_state(&self) -> Self::Board {
        self.state
    }

    fn update_state(&mut self, new_state: Self::Board) {
        self.state = new_state;
    }

    fn update_state_by_idx(&mut self, idx: usize, new_value: u64) {
        let shift_amount = (15 - idx) * 4;
        self.state = (self.get_state() & !(0xf << shift_amount)) | (new_value << shift_amount);
    }

    fn move_left_or_right(&mut self, move_dir: Move) {
        let mut new_board: u64 = 0;
        for row_idx in 0..4 {
            let row_val = self.extract_row(row_idx);
            let new_row_val = match move_dir {
                Move::Left => self.stores.move_left.get(&row_val),
                Move::Right => self.stores.move_right.get(&row_val),
                _ => panic!("Trying to move up or down in move_left_or_right"),
            };
            match new_row_val {
                Some(value) => new_board = new_board | (value << (48 - (16 * row_idx))),
                None => panic!(format!("The row: {} was not found in the stores", row_val)),
            }
        }
        self.update_state(new_board);
    }

    fn move_up_or_down(&mut self, move_dir: Move) {
        let mut new_board: u64 = 0;
        for col_idx in 0..4 {
            let col_val = self.extract_col(col_idx);
            let new_col_val = match move_dir {
                Move::Up => self.stores.move_up.get(&col_val),
                Move::Down => self.stores.move_down.get(&col_val),
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

    fn get_empty_tile_idxs(&self) -> Vec<usize> {
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

    fn get_score(&self) -> u64 {
        (0..16).fold(0, |acc, idx| {
            let num = self.extract_tile(idx);

            acc + (2 as u64).pow(num as u32)
        })
    }
}

impl Optimised {
    fn shift_left_or_right(row: u64, direction: Move) -> u64 {
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(row >> ((3 - tile_idx) * 4) & 0xf);
            tiles
        });
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
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(col >> ((3 - tile_idx) * 16) & 0xf);
            tiles
        });
        match direction {
            Move::Up => tiles = shift_vec_left(tiles),
            Move::Down => tiles = shift_vec_right(tiles),
            _ => panic!("trying to shift left or right in shift up or down"),
        }
        tiles[0] <<= 48;
        tiles[1] <<= 32;
        tiles[2] <<= 16;
        tiles[0] | tiles[1] | tiles[2] | tiles[3]
    }

    fn extract_tile(&self, idx: usize) -> u64 {
        (self.get_state() >> ((15 - idx) * 4)) & 0xf
    }

    fn extract_row(&self, row_num: u64) -> u64 {
        (self.get_state() >> ((3 - row_num) * 16)) & 0xffff
    }

    fn extract_col(&self, col_num: u64) -> u64 {
        (self.get_state() >> ((3 - col_num) * 4)) & 0x000f000f000f000f
    }

    fn create_stores() -> Stores {
        let mut move_left_store = HashMap::new();
        let mut move_right_store = HashMap::new();
        let mut move_up_store = HashMap::new();
        let mut move_down_store = HashMap::new();

        for val in 0..0xffff {
            move_left_store.insert(val, Optimised::shift_left_or_right(val, Move::Left));
            move_right_store.insert(val, Optimised::shift_left_or_right(val, Move::Right));

            let col_val = ((val << 36) | (val << 24) | (val << 12) | val) & 0x000f000f000f000f;
            move_up_store.insert(col_val, Optimised::shift_up_or_down(col_val, Move::Up));
            move_down_store.insert(col_val, Optimised::shift_up_or_down(col_val, Move::Down));
        }

        Stores {
            move_left: move_left_store,
            move_right: move_right_store,
            move_up: move_up_store,
            move_down: move_down_store,
        }
    }
}

impl fmt::Display for Optimised {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_test_update_state_by_idx() {
        let mut game = Optimised::new();
        game.update_state(0);
        game.update_state_by_idx(15, 2);
        game.update_state_by_idx(12, 3);
        assert_eq!(game.get_state(), 0x0000000000003002);
    }

    #[test]
    fn it_test_get_empty_tile_idxs() {
        let mut game = Optimised::new();
        game.update_state(0x0000111122220000);
        assert_eq!(game.get_empty_tile_idxs(), vec![0, 1, 2, 3, 12, 13, 14, 15]);
    }

    #[test]
    fn it_test_generate_random_tile() {
        let mut game = Optimised::new();
        game.update_state(0);
        for _ in 0..16 {
            game.generate_random_tile();
        }
        assert_eq!(game.get_empty_tile_idxs().len(), 0);
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(Optimised::shift_left_or_right(0x0000, Move::Left), 0x0000);
        assert_eq!(Optimised::shift_left_or_right(0x0002, Move::Left), 0x2000);
        assert_eq!(Optimised::shift_left_or_right(0x2020, Move::Left), 0x3000);
        assert_eq!(Optimised::shift_left_or_right(0x1332, Move::Left), 0x1420);
        assert_eq!(Optimised::shift_left_or_right(0x1234, Move::Left), 0x1234);
        assert_eq!(Optimised::shift_left_or_right(0x1002, Move::Left), 0x1200);
        assert_ne!(Optimised::shift_left_or_right(0x1210, Move::Left), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(Optimised::shift_left_or_right(0x0000, Move::Right), 0x0000);
        assert_eq!(Optimised::shift_left_or_right(0x2000, Move::Right), 0x0002);
        assert_eq!(Optimised::shift_left_or_right(0x2020, Move::Right), 0x0003);
        assert_eq!(Optimised::shift_left_or_right(0x1332, Move::Right), 0x0142);
        assert_eq!(Optimised::shift_left_or_right(0x1234, Move::Right), 0x1234);
        assert_eq!(Optimised::shift_left_or_right(0x1002, Move::Right), 0x0012);
        assert_ne!(Optimised::shift_left_or_right(0x0121, Move::Right), 0x0022);
    }

    #[test]
    fn test_move_left() {
        let mut game = Optimised::new();
        game.update_state(0x1234133220021002);
        game.move_left_or_right(Move::Left);
        assert_eq!(game.get_state(), 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        let mut game = Optimised::new();
        game.update_state(0x1121230033004222);
        game.move_up_or_down(Move::Up);
        assert_eq!(game.get_state(), 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        let mut game = Optimised::new();
        game.update_state(0x1234133220021002);
        game.move_left_or_right(Move::Right);
        assert_eq!(game.get_state(), 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        let mut game = Optimised::new();
        game.update_state(0x1121230033004222);
        game.move_up_or_down(Move::Down);
        assert_eq!(game.get_state(), 0x1000210034014232);
    }
}
