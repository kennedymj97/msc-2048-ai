use std::fmt;

use rand::Rng;

fn main() {
    let game1 = GameState::new();
    println!("{}", game1);
    let game = GameState::from(81985529216486895);
    println!("{}", game);
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
                _ => arr[idx] = Some((2 as u32).pow(num)),
            }

            arr
        });
        board
    }

    fn shift_left(tiles: &mut [u32; 4]) -> &mut [u32; 4] {
        for i in 0..4 {
            let slice = &mut tiles[i..4];
            GameState::calc_val(slice);
        }
        tiles
    }

    fn calc_val(slice: &mut [u32]) {
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

    fn extract_tile(&self, idx: usize) -> u32 {
        ((self.0 >> ((15 - idx) * 4)) & 15) as u32
    }

    fn extract_row(&self, row_num: u32) -> u32 {
        ((self.0 >> ((3 - row_num) * 16)) & 65535) as u32
    }

    fn extract_col(&self, col_num: u32) -> u32 {
        // extract the 4 cells
        let mut tile_vals: [u32; 4] = (0..4).fold([0; 4], |mut arr, idx| {
            let tile_val = self.extract_tile((col_num + (idx * 4)) as usize);
            arr[idx as usize] = tile_val;
            arr
        });
        // shift the cells appropriately
        tile_vals[0] = tile_vals[0] << 12;
        tile_vals[1] = tile_vals[1] << 8;
        tile_vals[2] = tile_vals[2] << 4;

        // or the shifted vals together for the 16 bit column value
        tile_vals[0] | tile_vals[1] | tile_vals[2] | tile_vals[3]
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
