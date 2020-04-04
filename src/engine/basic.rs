use crate::engine::*;
use rand::Rng;
use std::fmt;

type State = Vec<u64>;

#[derive(Clone)]
pub struct Basic(State);

impl GameEngine for Basic {
    type Board = State;

    fn new() -> Self {
        let mut new_game = Basic(vec![0; 16]);
        new_game.generate_random_tile();
        new_game.generate_random_tile();
        new_game
    }

    fn get_state(&self) -> Self::Board {
        self.0.clone()
    }

    fn update_state(&mut self, new_state: Self::Board) {
        self.0 = new_state;
    }

    fn move_left_or_right(&mut self, dir: Move) {
        let rows = (0..4).fold(Vec::new(), |mut vec, row_idx| {
            let starting_idx = row_idx * 4;
            let finishing_idx = starting_idx + 4;
            vec.push(self.get_state()[starting_idx..finishing_idx].to_vec());
            vec
        });

        let mut result = vec![];
        for mut row in rows {
            match dir {
                Move::Left => row = shift_vec_left(row),
                Move::Right => row = shift_vec_right(row),
                _ => panic!("trying to move up or down in move left or right"),
            }
            result.append(&mut row);
        }
        self.update_state(result);
    }

    fn move_up_or_down(&mut self, dir: Move) {
        let cols = (0..4).fold(Vec::new(), |mut vec, col_idx| {
            vec.push(self.get_col(col_idx));
            vec
        });

        let mut result = vec![0; 16];
        for (col_idx, mut col) in cols.into_iter().enumerate() {
            match dir {
                Move::Up => col = shift_vec_left(col),
                Move::Down => col = shift_vec_right(col),
                _ => panic!("trying to move left or right in move up or down"),
            }
            for (row_idx, tile) in col.into_iter().enumerate() {
                let tile_idx = row_idx * 4 + col_idx;
                result[tile_idx] = tile;
            }
        }
        self.update_state(result);
    }

    fn generate_random_tile(&mut self) {
        let zero_tile_idxs: Vec<u64> = self
            .get_state()
            .iter()
            .zip(0..)
            .filter_map(|(&ele, idx)| if ele == 0 { Some(idx) } else { None })
            .collect();
        let mut rng = rand::thread_rng();
        let rand_idx: usize = zero_tile_idxs[rng.gen_range(0, zero_tile_idxs.len())] as usize;
        let new_tile_val = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
        self.update_state(
            self.get_state()
                .iter()
                .enumerate()
                .map(|(idx, &ele)| if idx == rand_idx { new_tile_val } else { ele })
                .collect(),
        );
    }

    fn to_vec(&self) -> Vec<Option<u64>> {
        self.get_state()
            .iter()
            .map(|&x| {
                if x == 0 {
                    None
                } else {
                    Some((2 as u64).pow(x as u32))
                }
            })
            .collect()
    }
}

impl Basic {
    fn get_col(&self, col_idx: usize) -> Vec<u64> {
        (0..4).fold(Vec::new(), |mut vec, row_idx| {
            let tile_idx = row_idx * 4 + col_idx;
            vec.push(self.get_state()[tile_idx]);
            vec
        })
    }
}

impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_tile() {
        let mut game = Basic::new();
        game.update_state(vec![0; 16]);
        for _ in 0..16 {
            game.generate_random_tile();
        }
        assert_eq!(
            game.get_state()
                .iter()
                .filter(|&x| *x != 0)
                .collect::<Vec<_>>()
                .len(),
            16
        );
    }

    #[test]
    fn test_to_vec() {
        let mut game = Basic::new();
        game.update_state(vec![0, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
        assert_eq!(
            game.to_vec(),
            vec![
                None,
                None,
                Some(2),
                Some(2),
                Some(4),
                Some(4),
                Some(4),
                Some(4),
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                Some(16),
                Some(16),
                Some(16),
                Some(16)
            ]
        );
    }

    #[test]
    fn it_move_left() {
        let mut game = Basic::new();
        game.update_state(vec![0, 2, 0, 2, 3, 4, 3, 4, 5, 5, 5, 0, 6, 6, 6, 6]);
        game.move_left_or_right(Move::Left);
        assert_eq!(
            game.get_state(),
            vec![3, 0, 0, 0, 3, 4, 3, 4, 6, 5, 0, 0, 7, 7, 0, 0]
        );
    }

    #[test]
    fn it_move_right() {
        let mut game = Basic::new();
        game.update_state(vec![0, 7, 8, 7, 1, 1, 2, 1, 3, 5, 5, 1, 5, 6, 7, 7]);
        game.move_left_or_right(Move::Right);
        assert_eq!(
            game.get_state(),
            vec![0, 7, 8, 7, 0, 2, 2, 1, 0, 3, 6, 1, 0, 5, 6, 8]
        );
    }

    #[test]
    fn it_move_up() {
        let mut game = Basic::new();
        game.update_state(vec![1, 0, 4, 7, 2, 0, 3, 7, 1, 2, 3, 6, 2, 2, 3, 6]);
        game.move_up_or_down(Move::Up);
        assert_eq!(
            game.get_state(),
            vec![1, 3, 4, 8, 2, 0, 4, 7, 1, 0, 3, 0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn it_move_down() {
        let mut game = Basic::new();
        game.update_state(vec![1, 0, 4, 7, 2, 0, 3, 7, 1, 2, 3, 6, 2, 2, 3, 6]);
        game.move_up_or_down(Move::Down);
        assert_eq!(
            game.get_state(),
            vec![1, 0, 0, 0, 2, 0, 4, 0, 1, 0, 3, 8, 2, 3, 4, 7]
        );
    }
}
