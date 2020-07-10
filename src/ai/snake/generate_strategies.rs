use super::rules::Rules;
use permutohedron::Heap;

pub fn generate_strategies(generators: &[Variations], max_length: usize) -> Vec<Rules> {
    // Takes in a list of generators
    // Need to generate all variations of each generator and concat them together
    let mut n = 0;
    let set = generators.iter().fold(Vec::new(), |mut set, subset| {
        n += subset.len();
        let mut subset = subset.clone();
        set.append(&mut subset);
        set
    });

    let power_set: Vec<Rules> = set.iter().fold(vec![vec![]], |mut power_set, set_item| {
        let i = power_set.clone().into_iter().map(|mut sub_set| {
            sub_set.push(set_item.clone());
            sub_set
        });
        power_set.extend(i);
        power_set
    });

    let power_set = power_set
        .into_iter()
        .filter(|item| item.len() <= max_length)
        .collect::<Vec<Rules>>();

    let mut all_strategies = Vec::new();
    for mut item in power_set {
        let heap = Heap::new(&mut item);
        for data in heap {
            all_strategies.push(data.clone());
        }
    }
    assert!(all_strategies.len() as u64 == num_of_possible_strategies(n, max_length));
    all_strategies
}

fn num_of_possible_strategies(n: usize, k: usize) -> u64 {
    assert!(n >= k);
    (0..k + 1).fold(0, |acc, size_of_subset| {
        acc + (factorial(n) / factorial(n - size_of_subset))
    })
}

fn factorial(n: usize) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n as u64,
    }
}

use super::rules::Rule;
use crate::engine::Move;

pub type Variations<'a> = Vec<Box<dyn Rule>>;

pub fn generate_rule_variations(f: fn(Move) -> Box<dyn Rule>, move_dirs: &[Move]) -> Variations {
    move_dirs
        .iter()
        .fold(Vec::new(), |mut variations, &move_dir| {
            variations.push(f(move_dir));
            variations
        })
}

//pub trait Generator {
//    fn get_all_variations(&self) -> Variations;
//}
//
//pub struct BanMoveIfLeftColumnLockedGenerator(Vec<Box<dyn Rule>>);
//
//impl BanMoveIfLeftColumnLockedGenerator {
//    pub fn new() -> Box<Self> {
//        Box::new(BanMoveIfLeftColumnLockedGenerator(vec![
//            BanMoveIfLeftColumnLocked::new(Move::Up),
//        ]))
//    }
//}
//
//impl Generator for BanMoveIfLeftColumnLockedGenerator {
//    fn get_all_variations(&self) -> Variations {
//        self.0.clone()
//    }
//}
//
//pub struct TryMoveIfMergePossibleGenerator(Vec<Box<dyn Rule>>);
//
//impl TryMoveIfMergePossibleGenerator {
//    pub fn new() -> Box<Self> {
//        Box::new(TryMoveIfMergePossibleGenerator(vec![
//            TryMoveIfMergePossible::new(Move::Left),
//        ]))
//    }
//}
//
//impl Generator for TryMoveIfMergePossibleGenerator {
//    fn get_all_variations(&self) -> Variations {
//        self.0.clone()
//    }
//}
//
//pub struct TryMoveIfProducesLeftMergeGenerator(Vec<Box<dyn Rule>>);
//
//impl TryMoveIfProducesLeftMergeGenerator {
//    pub fn new() -> Box<Self> {
//        Box::new(TryMoveIfProducesLeftMergeGenerator(vec![
//            TryMoveIfProducesLeftMerge::new(Move::Down),
//            TryMoveIfProducesLeftMerge::new(Move::Up),
//        ]))
//    }
//}
//
//impl Generator for TryMoveIfProducesLeftMergeGenerator {
//    fn get_all_variations(&self) -> Variations {
//        self.0.clone()
//    }
//}
