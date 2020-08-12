#![allow(unused_imports)]
use msc_2048_ai::ai::snake::*;
use msc_2048_ai::engine::*;

fn main() {
    //search::search(3, 4);
    search::test_search_method(
        |engine, max_ban_length, max_try_length| {
            let random = search::random::random(engine, max_ban_length, max_try_length);
            search::iterated_local::ils_mutate_try_always_accept(engine, random)
        },
        "./ils_mutate_try_always_accept_100_repeats_random_init.csv",
        100,
    );
    //search::random::random_search(3, 4, 3000, 1000);
    //search::greedy::greedy(
    //    3,
    //    4,
    //    search::greedy::Greedy::PrioritiseTry,
    //    mann_whitney::Confidence::P05,
    //    1000,
    //);
}
