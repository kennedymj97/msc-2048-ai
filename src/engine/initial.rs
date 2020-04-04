use rand::Rng;
use std::fmt;

use crate::engine::*;

type State = u64;

#[derive(Clone)]
pub struct Initial(pub State);

impl GameEngine for Initial {
    type Board = State;

    fn new() -> Self {
        let mut game = Initial(0);
        game.generate_random_tile();
        game.generate_random_tile();
        game
    }

    fn get_state(&self) -> Self::Board {
        self.0
    }

    fn update_state(&mut self, new_state: Self::Board) {
        self.0 = new_state;
    }

    fn move_left_or_right(&mut self, move_dir: Move) {
        // for each row calculate the new state and update the bit board
        let rows = (0..4).fold(Vec::new(), |mut rows, row_idx| {
            rows.push(self.extract_row(row_idx));
            rows
        });

        let mut new_rows: Vec<u64> = rows
            .iter()
            .map(|row| match move_dir {
                Move::Left => Initial::shift_left(*row),
                Move::Right => Initial::shift_right(*row),
                _ => panic!("Trying to move up or down in move_left_or_right()"),
            })
            .collect();
        new_rows[0] <<= 48;
        new_rows[1] <<= 32;
        new_rows[2] <<= 16;
        self.update_state(new_rows[0] | new_rows[1] | new_rows[2] | new_rows[3]);
    }

    fn move_up_or_down(&mut self, move_dir: Move) {
        let cols = (0..4).fold(Vec::new(), |mut cols, col_idx| {
            cols.push(self.extract_col(col_idx));
            cols
        });
        let mut new_cols: Vec<u64> = cols
            .iter()
            .map(|col| {
                let col_val;
                match move_dir {
                    Move::Up => col_val = Initial::shift_left(*col),
                    Move::Down => col_val = Initial::shift_right(*col),
                    _ => panic!("Trying to move left or right in move_up_or_down()"),
                }
                let tile0 = (col_val & 0xf000) << 36;
                let tile1 = (col_val & 0x0f00) << 24;
                let tile2 = (col_val & 0x00f0) << 12;
                let tile3 = col_val & 0x000f;
                tile0 | tile1 | tile2 | tile3
            })
            .collect();
        new_cols[0] <<= 12;
        new_cols[1] <<= 8;
        new_cols[2] <<= 4;
        self.update_state(new_cols[0] | new_cols[1] | new_cols[2] | new_cols[3]);
    }

    fn generate_random_tile(&mut self) {
        let zero_tiles = self.get_zero_tiles();
        let num_zero_tiles = zero_tiles.len();
        if num_zero_tiles == 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, zero_tiles.len());
        let rand_val = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
        self.update_state(self.get_state() | (rand_val << ((15 - zero_tiles[rand_idx]) * 4)));
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
}

impl Initial {
    fn from(num: u64) -> Initial {
        Initial(num)
    }

    fn shift_right(col: u64) -> u64 {
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(col >> ((3 - tile_idx) * 4) & 0xf);
            tiles
        });
        tiles = shift_vec_right(tiles);
        tiles[0] <<= 12;
        tiles[1] <<= 8;
        tiles[2] <<= 4;
        tiles[0] | tiles[1] | tiles[2] | tiles[3]
    }

    fn shift_left(row: u64) -> u64 {
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(row >> ((3 - tile_idx) * 4) & 0xf);
            tiles
        });
        tiles = shift_vec_left(tiles);
        tiles[0] <<= 12;
        tiles[1] <<= 8;
        tiles[2] <<= 4;
        tiles[0] | tiles[1] | tiles[2] | tiles[3]
    }

    fn extract_tile(&self, idx: usize) -> u64 {
        (self.get_state() >> ((15 - idx) * 4)) & 0xf
    }

    fn extract_row(&self, row_num: u64) -> u64 {
        (self.get_state() >> ((3 - row_num) * 16)) & 0xffff
    }

    fn extract_col(&self, col_num: u64) -> u64 {
        // extract the 4 cells
        let mut tiles: Vec<u64> = (0..4).fold(Vec::new(), |mut tiles, idx| {
            let tile_val = self.extract_tile((col_num + (idx * 4)) as usize);
            tiles.push(tile_val);
            tiles
        });
        // shift the cells appropriately
        tiles[0] <<= 12;
        tiles[1] <<= 8;
        tiles[2] <<= 4;

        // or the shifted vals together for the 16 bit column value
        tiles[0] | tiles[1] | tiles[2] | tiles[3]
    }

    fn get_zero_tiles(&self) -> Vec<usize> {
        (0..16).fold(Vec::new(), |mut vec, idx| {
            let tile_val = self.extract_tile(idx);
            if tile_val == 0 {
                vec.push(idx)
            };
            vec
        })
    }
}

impl fmt::Display for Initial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_shift_left() {
        assert_eq!(Initial::shift_left(0x0000), 0x0000);
        assert_eq!(Initial::shift_left(0x0002), 0x2000);
        assert_eq!(Initial::shift_left(0x2020), 0x3000);
        assert_eq!(Initial::shift_left(0x1332), 0x1420);
        assert_eq!(Initial::shift_left(0x1234), 0x1234);
        assert_eq!(Initial::shift_left(0x1002), 0x1200);
        assert_ne!(Initial::shift_left(0x1210), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(Initial::shift_right(0x0000), 0x0000);
        assert_eq!(Initial::shift_right(0x2000), 0x0002);
        assert_eq!(Initial::shift_right(0x2020), 0x0003);
        assert_eq!(Initial::shift_right(0x1332), 0x0142);
        assert_eq!(Initial::shift_right(0x1234), 0x1234);
        assert_eq!(Initial::shift_right(0x1002), 0x0012);
        assert_ne!(Initial::shift_right(0x0121), 0x0022);
    }

    #[test]
    fn test_move_left() {
        let mut game = Initial::from(0x1234133220021002);
        game.move_left_or_right(Move::Left);
        assert_eq!(game.0, 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        let mut game = Initial::from(0x1121230033004222);
        game.move_up_or_down(Move::Up);
        assert_eq!(game.0, 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        let mut game = Initial::from(0x1234133220021002);
        game.move_left_or_right(Move::Right);
        assert_eq!(game.0, 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        let mut game = Initial::from(0x1121230033004222);
        game.move_up_or_down(Move::Down);
        assert_eq!(game.0, 0x1000210034014232);
    }

    #[bench]
    fn bench_extract_row(b: &mut Bencher) {
        let game = Initial::from(0x1111222233334444);
        b.iter(|| {
            let row = game.extract_row(0);
            println!("{}", row);
        });
    }

    #[bench]
    fn bench_test(b: &mut Bencher) {
        b.iter(|| std::thread::sleep(std::time::Duration::from_nanos(100)));
    }
}
