use crate::engine::GameEngine;
use rand::Rng;
use std::fmt;

type State = Vec<u32>;

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

    fn move_left(&mut self) {}

    fn move_right(&mut self) {}

    fn move_down(&mut self) {}

    fn move_up(&mut self) {}

    fn to_vec(&self) -> Vec<Option<u32>> {
        self.get_state()
            .iter()
            .map(|&x| {
                if x == 0 {
                    None
                } else {
                    Some((2 as u32).pow(x))
                }
            })
            .collect()
    }
}

impl Basic {
    fn from(state: State) -> Self {
        Basic(state)
    }

    fn generate_random_tile(&mut self) {
        let zero_tile_idxs: Vec<u32> = self
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
        let mut game = Basic::from(vec![0; 16]);
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
        let game = Basic::from(vec![0, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
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
}
