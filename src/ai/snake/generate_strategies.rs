use permutohedron::Heap;

//pub fn generate_strategies<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
//    let power_set = set.iter().fold(vec![vec![]], |mut power_set, set_item| {
//        let i = power_set.clone().into_iter().map(|mut sub_set| {
//            sub_set.push(set_item.clone());
//            sub_set
//        });
//        power_set.extend(i);
//        power_set
//    });
//
//    let mut all_strategies = Vec::new();
//    for mut set in power_set {
//        let heap = Heap::new(&mut set);
//        for data in heap {
//            all_strategies.push(data.clone());
//        }
//    }
//    all_strategies
//}
use super::rules::Rules;

pub fn generate_strategies(generators: &[Variations]) -> Vec<Rules> {
    // Takes in a list of generators
    // Need to generate all variations of each generator and concat them together
    let set = generators.iter().fold(Vec::new(), |mut set, subset| {
        let mut subset = subset.clone();
        set.append(&mut subset);
        set
    });

    let power_set = set.iter().fold(vec![vec![]], |mut power_set, set_item| {
        let i = power_set.clone().into_iter().map(|mut sub_set| {
            sub_set.push(set_item.clone());
            sub_set
        });
        power_set.extend(i);
        power_set
    });

    let mut all_strategies = Vec::new();
    for mut item in power_set {
        let heap = Heap::new(&mut item);
        for data in heap {
            all_strategies.push(data.clone());
        }
    }
    all_strategies
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
