#![allow(unused_imports)]
use msc_2048_ai::ai::snake::*;
use msc_2048_ai::engine::*;

fn main() {
    //Ban Rules: ban move down if left column not locked
    //Try Rules: try move left if creates monotonic left column->try move down if produces merge->try move up if produces merge->try move up if creates monotonic middle bottom row
    //Fallback: left->down->up->right
    //
    //
    //
    //search::test_different_lengths("./different_lengths.csv");
    //search::test_chosen_length("./chosen_lengths.csv");
    //search::search(3, 4);
    //search::test_search_method(
    //    |engine, max_ban_length, max_try_length| {
    //        //let greedy =
    //        //    search::greedy::greedy_prioritise_best(engine, max_ban_length, max_try_length);
    //        search::iterated_local::ils_mutate_try_always_accept_save(engine, greedy)
    //    },
    //    "./test_cancel.csv",
    //    10,
    //);
    //let engine = GameEngine::new();
    //let init = search::random::random(&engine, 2, 4);
    //let init = search::random::random_search(&engine, 2, 4);
    //let init = search::greedy::greedy_prioritise_best(&engine, 2, 4);
    //search::iterated_local::ils_mutate_try_accept_if_better_save(
    //    &engine,
    //    init,
    //    "./ils_mutate_try_accept_if_better_greedy_init.csv",
    //);
    //search::random::random_search(3, 4, 3000, 1000);
    //search::greedy::greedy(
    //    3,
    //    4,
    //    search::greedy::Greedy::PrioritiseTry,
    //    mann_whitney::Confidence::P05,
    //    1000,
    //);
}
