use rand::Rng;
use std::fmt;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut game = GameState::new();
    println!("{}", game);
    run_game(&mut game);
}

fn run_game(game: &mut GameState) {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Use the arrow keys to control the game.\r\n{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        game,
        termion::cursor::Hide
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        let current_state = game.0;

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => game.move_left(),
            Key::Right => game.move_right(),
            Key::Up => game.move_up(),
            Key::Down => game.move_down(),
            _ => (),
        }

        let new_state = game.0;

        if current_state != new_state {
            game.generate_random_tile();
        }
        write!(stdout, "{}", game).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

struct GameState(u64);

impl GameState {
    fn new() -> GameState {
        let mut game = GameState(0);
        game.generate_random_tile();
        game.generate_random_tile();
        game
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
        self.move_left_or_right(Move::Left);
    }

    fn move_right(&mut self) {
        self.move_left_or_right(Move::Right);
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
                Move::Left => GameState::shift_left(*row),
                Move::Right => GameState::shift_right(*row),
                _ => panic!("Trying to move up or down in move_left_or_right()"),
            })
            .collect();
        new_rows[0] <<= 48;
        new_rows[1] <<= 32;
        new_rows[2] <<= 16;
        self.0 = new_rows[0] | new_rows[1] | new_rows[2] | new_rows[3];
    }

    fn move_up(&mut self) {
        self.move_up_or_down(Move::Up);
    }

    fn move_down(&mut self) {
        self.move_up_or_down(Move::Down);
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
                    Move::Up => col_val = GameState::shift_left(*col),
                    Move::Down => col_val = GameState::shift_right(*col),
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
        self.0 = new_cols[0] | new_cols[1] | new_cols[2] | new_cols[3];
    }

    fn shift_right(col: u64) -> u64 {
        let tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(col >> ((3 - tile_idx) * 4) & 0xf);
            tiles
        });
        let mut tiles: Vec<u64> = tiles.iter().rev().map(|x| *x).collect();
        for i in 0..4 {
            let slice = &mut tiles[i..4];
            GameState::calc_val(slice);
        }
        tiles[3] <<= 12;
        tiles[2] <<= 8;
        tiles[1] <<= 4;
        tiles[3] | tiles[2] | tiles[1] | tiles[0]
    }

    fn shift_left(row: u64) -> u64 {
        let mut tiles = (0..4).fold(Vec::new(), |mut tiles, tile_idx| {
            tiles.push(row >> ((3 - tile_idx) * 4) & 0xf);
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

    fn extract_tile(&self, idx: usize) -> u64 {
        (self.0 >> ((15 - idx) * 4)) & 0xf
    }

    fn extract_row(&self, row_num: u64) -> u64 {
        (self.0 >> ((3 - row_num) * 16)) & 0xffff
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

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board: Vec<String> = self.parse_board().iter().map(|x| format_val(x)).collect();
        let out = format!(
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
        assert_eq!(GameState::shift_left(0x0000), 0x0000);
        assert_eq!(GameState::shift_left(0x0002), 0x2000);
        assert_eq!(GameState::shift_left(0x2020), 0x3000);
        assert_eq!(GameState::shift_left(0x1332), 0x1420);
        assert_eq!(GameState::shift_left(0x1234), 0x1234);
        assert_eq!(GameState::shift_left(0x1002), 0x1200);
        assert_ne!(GameState::shift_left(0x1210), 0x2200);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(GameState::shift_right(0x0000), 0x0000);
        assert_eq!(GameState::shift_right(0x2000), 0x0002);
        assert_eq!(GameState::shift_right(0x2020), 0x0003);
        assert_eq!(GameState::shift_right(0x1332), 0x0142);
        assert_eq!(GameState::shift_right(0x1234), 0x1234);
        assert_eq!(GameState::shift_right(0x1002), 0x0012);
        assert_ne!(GameState::shift_right(0x0121), 0x0022);
    }

    #[test]
    fn test_move_left() {
        let mut game = GameState::from(0x1234133220021002);
        game.move_left();
        assert_eq!(game.0, 0x1234142030001200);
    }

    #[test]
    fn test_move_up() {
        let mut game = GameState::from(0x1121230033004222);
        game.move_up();
        assert_eq!(game.0, 0x1131240232004000);
    }

    #[test]
    fn test_move_right() {
        let mut game = GameState::from(0x1234133220021002);
        game.move_right();
        assert_eq!(game.0, 0x1234014200030012);
    }

    #[test]
    fn test_move_down() {
        let mut game = GameState::from(0x1121230033004222);
        game.move_down();
        assert_eq!(game.0, 0x1000210034014232);
    }
}
