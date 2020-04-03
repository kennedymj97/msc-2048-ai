pub mod basic;
pub mod initial;

pub use basic::Basic;
pub use initial::Initial;
use std::fmt::Display;

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub trait GameEngine: Display + Clone {
    type Board: PartialEq + Eq;

    fn new() -> Self;

    fn get_state(&self) -> Self::Board;

    fn update_state(&mut self, state: Self::Board);

    fn move_left(&mut self);

    fn move_right(&mut self);

    fn move_up(&mut self);

    fn move_down(&mut self);

    fn is_game_over(&mut self) -> bool {
        let mut game_clone = self.clone();
        let old_state = game_clone.get_state();
        game_clone.move_down();
        game_clone.move_up();
        game_clone.move_left();
        game_clone.move_right();
        let new_state = game_clone.get_state();
        old_state == new_state
    }

    fn to_vec(&self) -> Vec<Option<u32>>;

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
