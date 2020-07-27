use super::rules::BanMove;
use super::rules::TryMove;
use super::Snake;
use crate::engine::Move;
use permutohedron::Heap;
use std::iter::Iterator;

pub struct Iter {
    ban_sets: Vec<Vec<BanMove>>,
    try_sets: Vec<Vec<TryMove>>,
    fallback_sets: Vec<Vec<Move>>,
    ban_idx: usize,
    try_idx: usize,
    fallback_idx: usize,
}

impl Iter {
    pub fn len(&self) -> usize {
        self.ban_sets.len() * self.try_sets.len() * self.fallback_sets.len()
    }
}

impl Iterator for Iter {
    type Item = Snake;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fallback_idx == self.fallback_sets.len() {
            self.try_idx += 1;
            self.fallback_idx = 0;
        }

        if self.try_idx == self.try_sets.len() {
            self.ban_idx += 1;
            self.try_idx = 0;
        }

        if self.ban_idx == self.ban_sets.len() {
            return None;
        }

        let snake = Snake::new(
            &self.ban_sets[self.ban_idx],
            &self.try_sets[self.try_idx],
            &self.fallback_sets[self.fallback_idx],
        );
        self.fallback_idx += 1;
        Some(snake)
    }
}

pub fn get_snake_iterator<'a>(max_ban_length: usize, max_try_length: usize) -> Iter {
    // Generate all possible ban variations
    // power_set and permuations up to certain length
    let ban_variations = BanMove::generate_all_variations();
    let ban_sets = power_set(&ban_variations, max_ban_length);
    assert_eq!(
        num_of_power_sets(ban_variations.len(), max_ban_length),
        ban_sets.len() as u64
    );
    // Generate all possbile try variations
    // power_set and permutation up to certain length
    let try_variations = TryMove::generate_all_variations();
    let try_sets = permuations(power_set(&try_variations, max_try_length));
    assert_eq!(
        num_of_possible_sets(try_variations.len(), max_try_length),
        try_sets.len() as u64
    );

    // Generate all possible fallback variations
    // Only permuations of the 4 moves
    let fallback_sets = permuations(vec![vec![Move::Left, Move::Right, Move::Up, Move::Down]]);
    assert_eq!(factorial(4), fallback_sets.len() as u64);

    println!(
        "Total number of snakes = {}",
        ban_sets.len() * try_sets.len() * fallback_sets.len()
    );

    Iter {
        ban_sets,
        try_sets,
        fallback_sets,
        ban_idx: 0,
        try_idx: 0,
        fallback_idx: 0,
    }
}

pub fn generate_snakes(max_ban_length: usize, max_try_length: usize) -> Vec<Snake> {
    // Generate all possible ban variations
    // power_set and permuations up to certain length
    let ban_variations = BanMove::generate_all_variations();
    let ban_sets = power_set(&ban_variations, max_ban_length);
    assert_eq!(
        num_of_power_sets(ban_variations.len(), max_ban_length),
        ban_sets.len() as u64
    );
    // Generate all possbile try variations
    // power_set and permutation up to certain length
    let try_variations = TryMove::generate_all_variations();
    let try_sets = permuations(power_set(&try_variations, max_try_length));
    assert_eq!(
        num_of_possible_sets(try_variations.len(), max_try_length),
        try_sets.len() as u64
    );

    // Generate all possible fallback variations
    // Only permuations of the 4 moves
    let fallback_sets = permuations(vec![vec![Move::Left, Move::Right, Move::Up, Move::Down]]);
    assert_eq!(factorial(4), fallback_sets.len() as u64);

    println!(
        "Total number of snakes = {}",
        ban_sets.len() * try_sets.len() * fallback_sets.len()
    );

    // 3 nest for loops, for each ban variation add every try variation, for every ban and try
    //   variation and every fallback variation
    let mut snakes = Vec::new();
    for ban_set in ban_sets.iter() {
        for try_set in try_sets.iter() {
            for fallback_set in fallback_sets.iter() {
                snakes.push(Snake::new(ban_set, try_set, fallback_set));
            }
        }
    }
    assert_eq!(
        ban_sets.len() * try_sets.len() * fallback_sets.len(),
        snakes.len()
    );
    snakes
}

fn power_set<T: Copy>(set: &[T], max_length: usize) -> Vec<Vec<T>> {
    let power_set: Vec<Vec<T>> = set.iter().fold(vec![vec![]], |mut power_set, &set_item| {
        let i = power_set.clone().into_iter().map(|mut sub_set| {
            sub_set.push(set_item);
            sub_set
        });
        power_set.extend(i);
        power_set
    });

    power_set
        .into_iter()
        .filter(|item| item.len() <= max_length)
        .collect::<Vec<Vec<T>>>()
}

fn permuations<T: Copy>(set: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut permuations = Vec::new();
    for mut item in set {
        let heap = Heap::new(&mut item);
        for data in heap {
            permuations.push(data);
        }
    }
    permuations
}

// n is size of original set, k is max length of power set and permutations
fn num_of_possible_sets(n: usize, k: usize) -> u64 {
    assert!(n >= k);
    (0..k + 1).fold(0, |acc, size_of_subset| {
        acc + (factorial(n) / factorial(n - size_of_subset))
    })
}

fn num_of_power_sets(n: usize, k: usize) -> u64 {
    assert!(n >= k);
    (0..k + 1).fold(0, |acc, size_of_subset| {
        acc + (factorial(n) / (factorial(size_of_subset) * factorial(n - size_of_subset)))
    })
}

fn factorial(n: usize) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_snake_iter_size() {
        let snake_iter = get_snake_iterator(2, 4);
        assert_eq!(snake_iter.len(), snake_iter.count());
    }
}
