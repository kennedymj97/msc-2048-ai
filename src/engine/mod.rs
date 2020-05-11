pub mod basic;
pub mod optimised;

pub use basic::Basic;
pub use optimised::Optimised;
use std::fmt::Display;

#[derive(Debug)]
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

    fn update_state_by_idx(&mut self, idx: usize, new_value: u64);

    fn get_score(&self) -> u64 {
        self.to_vec()
            .iter()
            .map(|&x| match x {
                Some(y) => y,
                None => 0,
            })
            .sum()
    }

    fn move_left(&mut self) {
        execute(self, Move::Left);
    }

    fn move_right(&mut self) {
        execute(self, Move::Right);
    }

    fn move_up(&mut self) {
        execute(self, Move::Up);
    }

    fn move_down(&mut self) {
        execute(self, Move::Down);
    }

    fn move_left_or_right(&mut self, dir: Move);

    fn move_up_or_down(&mut self, dir: Move);

    fn is_game_over(&mut self) -> bool {
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

    fn generate_random_tile(&mut self);

    fn get_empty_tile_idxs(&self) -> Vec<usize>;

    fn to_vec(&self) -> Vec<Option<u64>>;

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

fn execute(engine: &mut impl GameEngine, dir: Move) {
    let old_state = engine.get_state();
    match dir {
        Move::Left | Move::Right => engine.move_left_or_right(dir),
        Move::Up | Move::Down => engine.move_up_or_down(dir),
    }
    let new_state = engine.get_state();
    if old_state != new_state {
        engine.generate_random_tile();
    }
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
}
