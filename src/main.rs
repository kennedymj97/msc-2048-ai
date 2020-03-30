use std::fmt;

use rand::Rng;

fn main() {
    let game1 = GameState::new();
    println!("{}", game1);
    let mut game = GameState::from(81985529216486895);
    println!("{}", game);
    game.move_left();
    println!("{}", game);
    let mut game2 = GameState::from(81985529216486895);
    game2.move_up();
    println!("{}", game2);
}

struct GameState(u64);

impl GameState {
    fn new() -> GameState {
        let mut game = GameState(0);
        game.generate_random_tile();
        game.generate_random_tile();
        game
    }

    fn from(val: u64) -> GameState {
        GameState(val)
    }

    fn parse_board(&self) -> [Option<u32>; 16] {
        let board = (0..16).fold([None; 16], |mut arr, idx| {
            let num = self.extract_tile(idx);

            match num {
                0 => arr[idx] = None,
                _ => arr[idx] = Some((2 as u32).pow(num as u32)),
            }

            arr
        });
        board
    }

    fn move_left(&mut self) {
        // for each row calculate the new state and update the bit board
        let rows = (0..4).fold(Vec::new(), |mut rows, row_idx| {
            rows.push(self.extract_row(row_idx));
            rows
        });

        let mut new_rows: Vec<u64> = rows.iter().map(|row| GameState::shift_left(*row)).collect();
        new_rows[0] <<= 48;
        new_rows[1] <<= 32;
        new_rows[2] <<= 16;
        self.0 = new_rows[0] | new_rows[1] | new_rows[2] | new_rows[3];
    }

    fn move_up(&mut self) {
        let cols = (0..4).fold(Vec::new(), |mut cols, col_idx| {
            cols.push(self.extract_col(col_idx));
            cols
        });
        let mut new_cols: Vec<u64> = cols
            .iter()
            .map(|col| {
                let col_val = GameState::shift_left(*col);
                let tile0 = (col_val & 0b1111000000000000) << 36;
                let tile1 = (col_val & 0b0000111100000000) << 24;
                let tile2 = (col_val & 0b0000000011110000) << 12;
                let tile3 = col_val & 0b0000000000001111;
                tile0 | tile1 | tile2 | tile3
            })
            .collect();
        new_cols[0] <<= 12;
        new_cols[1] <<= 8;
        new_cols[2] <<= 4;
        self.0 = new_cols[0] | new_cols[1] | new_cols[2] | new_cols[3];
    }

    fn move_right(&self) {}
    fn move_down(&self) {}

    fn shift_left(row: u64) -> u64 {
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(row >> ((3 - tile_idx) * 4) & 0b1111);
            tiles
        });
        for i in 0..4 {
            let slice = &mut tiles[i..4];
            GameState::calc_val(slice);
        }
        tiles[0] <<= 12;
        tiles[1] <<= 8;
        tiles[2] <<= 4;
        tiles[0] | tiles[1] | tiles[2] | tiles[3]
    }

    fn calc_val(slice: &mut [u64]) {
        slice[0] = slice.iter_mut().fold(0, |acc, val| {
            let temp;
            if acc != 0 && acc == *val {
                temp = 1;
                *val = 0;
            } else if acc == 0 && *val != 0 {
                temp = *val;
                *val = 0;
            } else {
                temp = 0;
            }
            acc + temp
        });
        //let mut acc = 0;
        //for idx in 0..slice.len() {
        //    let val = slice[idx];
        //    if acc != 0 && acc == val {
        //        slice[idx] = 0;
        //        acc += 1;
        //        break;
        //    } else if acc == 0 && val != 0 {
        //        slice[idx] = 0;
        //        acc = val;
        //    };
        //}
        //slice[0] = acc;
    }

    fn extract_tile(&self, idx: usize) -> u64 {
        (self.0 >> ((15 - idx) * 4)) & 0b1111
    }

    fn extract_row(&self, row_num: u64) -> u64 {
        (self.0 >> ((3 - row_num) * 16)) & 0b1111111111111111
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

    /// Function to generate random tiles
    /// The random tiles are either 2 or 4
    /// There is a 90% chance the tile generated is a 2 and 10% of a 4
    /// At the start of the game 2 tiles are generated randomly and after a tile is generated after
    /// a move has been completed
    fn generate_random_tile(&mut self) {
        let zero_tiles = self.get_zero_tiles();
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, zero_tiles.len());
        let rand_val = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
        self.0 = self.0 | (rand_val << ((15 - zero_tiles[rand_idx]) * 4));
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

// Temporary function just used to sanity check the bitwise operations
fn to_binary(num: u64) -> String {
    let mut temp = num;
    let mut bits = String::new();
    while temp > 0 {
        if temp % 2 == 0 {
            bits.push('0')
        } else {
            bits.push('1')
        };

        temp /= 2;
    }
    while bits.len() < 64 {
        bits.push('0');
    }
    bits.chars().rev().collect::<String>()
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board: Vec<String> = self.parse_board().iter().map(|x| format_val(x)).collect();
        let out = format!(
            "
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
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
        );
        write!(f, "{}", out)
    }
}

fn format_val(val: &Option<u32>) -> String {
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

enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_left() {
        assert_eq!(GameState::shift_left(&mut [0, 0, 0, 0]), &mut [0, 0, 0, 0]);
        assert_eq!(GameState::shift_left(&mut [0, 0, 0, 2]), &mut [2, 0, 0, 0]);
        assert_eq!(GameState::shift_left(&mut [2, 0, 2, 0]), &mut [3, 0, 0, 0]);
        assert_eq!(GameState::shift_left(&mut [1, 3, 3, 2]), &mut [1, 4, 2, 0]);
        assert_eq!(GameState::shift_left(&mut [1, 2, 3, 4]), &mut [1, 2, 3, 4]);
        assert_eq!(GameState::shift_left(&mut [1, 0, 0, 2]), &mut [1, 2, 0, 0]);
    }
}
