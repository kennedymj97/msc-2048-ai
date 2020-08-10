#![allow(unused_imports)]
use msc_2048_ai::ai::snake::*;
use msc_2048_ai::engine::*;

fn main() {
    search::random::random_search(3, 4, 3000, 1000);
    search::greedy::greedy(
        3,
        4,
        search::greedy::Greedy::PrioritiseTry,
        mann_whitney::Confidence::P05,
        1000,
    );
}
