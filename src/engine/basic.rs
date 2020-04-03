use crate::engine::GameEngine;
use rand::seq::IteratorRandom;
use rand::Rng;

type State = Vec<u32>;
pub struct Basic(State);

impl GameEngine for Basic {
    type Board = State;

    fn new() -> Self {
        let mut new_game = Basic(vec![0; 16]);
        new_game.generate_random_tile();
        new_game.generate_random_tile();
        new_game
    }

    fn get_state(&mut self) -> &mut Self::Board {
        &mut self.0
    }

    fn move_left(&mut self) {}

    fn move_right(&mut self) {}

    fn move_down(&mut self) {}

    fn move_up(&mut self) {}
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
            .filter(|(ele, _)| **ele == 0)
            .map(|(_, idx)| idx)
            .collect();
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, zero_tile_idxs.len());
        self.get_state()[rand_idx] = if rng.gen_range(0, 10) < 9 { 1 } else { 2 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_tile() {
        let mut game = Basic::from(vec![0; 16]);
        game.generate_random_tile();
        assert_eq!(
            game.get_state()
                .iter()
                .filter(|x| **x != 0)
                .collect::<Vec<_>>()
                .len(),
            1
        );
    }
}
