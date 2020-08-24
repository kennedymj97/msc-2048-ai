use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msc_2048_ai::engine;
use msc_2048_ai::engine::Move;
use msc_2048_ai::engine::{GameEngine, GameEngineNoStores, GameEngineStores, UnoptimiseEngine};

pub fn stores_shift_left(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let game = 0x1111222233334444;
    c.bench_function("stores_shift_left", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Left);
            })
        })
    });
}

pub fn stores_shift_right(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let game = 0x1111222233334444;
    c.bench_function("stores_shift_right", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Right);
            })
        })
    });
}

pub fn stores_shift_up(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let game = 0x1234123412341234;
    c.bench_function("stores shift up", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Up);
            })
        })
    });
}

pub fn stores_shift_down(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let game = 0x1234123412341234;
    c.bench_function("stores shift down", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Down);
            })
        })
    });
}

pub fn no_stores_shift_left(c: &mut Criterion) {
    let engine = GameEngineNoStores;
    let game = 0x1111222233334444;
    c.bench_function("no stores_shift_left", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Left);
            })
        })
    });
}

pub fn no_stores_shift_right(c: &mut Criterion) {
    let engine = GameEngineNoStores;
    let game = 0x1111222233334444;
    c.bench_function("no stores_shift_right", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Right);
            })
        })
    });
}

pub fn no_stores_shift_up(c: &mut Criterion) {
    let engine = GameEngineNoStores;
    let game = 0x1234123412341234;
    c.bench_function("no stores shift up", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Up);
            })
        })
    });
}

pub fn no_stores_shift_down(c: &mut Criterion) {
    let engine = GameEngineNoStores;
    let game = 0x1234123412341234;
    c.bench_function("no stores shift down", |b| {
        b.iter(|| {
            black_box({
                engine.shift(game, Move::Down);
            })
        })
    });
}

pub fn unoptimised_shift_left(c: &mut Criterion) {
    let game = vec![
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        vec![3, 3, 3, 3],
        vec![4, 4, 4, 4],
    ];
    c.bench_function("unoptimised shift left", |b| {
        b.iter(|| {
            black_box({
                UnoptimiseEngine::shift(game.clone(), Move::Left);
            })
        })
    });
}

pub fn unoptimised_shift_right(c: &mut Criterion) {
    let game = vec![
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        vec![3, 3, 3, 3],
        vec![4, 4, 4, 4],
    ];
    c.bench_function("unoptimised shift right", |b| {
        b.iter(|| {
            black_box({
                UnoptimiseEngine::shift(game.clone(), Move::Right);
            })
        })
    });
}

pub fn unoptimised_shift_up(c: &mut Criterion) {
    let game = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
    ];
    c.bench_function("unoptimised shift up", |b| {
        b.iter(|| {
            black_box({
                UnoptimiseEngine::shift(game.clone(), Move::Up);
            })
        })
    });
}

pub fn unoptimised_shift_down(c: &mut Criterion) {
    let game = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
    ];
    c.bench_function("unoptimised shift down", |b| {
        b.iter(|| {
            black_box({
                UnoptimiseEngine::shift(game.clone(), Move::Down);
            })
        })
    });
}

pub fn new_stores_game(c: &mut Criterion) {
    c.bench_function("initiate stores game", |b| {
        b.iter(|| {
            black_box({
                GameEngineStores::new();
            })
        })
    });
}

pub fn get_score_bench(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let game = 0x1111222233334444;
    c.bench_function("get score", |b| {
        b.iter(|| black_box(engine.get_score(game)))
    });
}

pub fn move_left_or_right_bench(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let mut game = 0x1111222233334444;
    c.bench_function("move left or right", |b| {
        b.iter(|| {
            black_box({
                game = engine.shift(game, Move::Left);
                game = engine.shift(game, Move::Right);
            })
        })
    });
}

pub fn move_up_or_down_bench(c: &mut Criterion) {
    let engine = GameEngineStores::new();
    let mut game = 0x1234123412341234;
    c.bench_function("move up or down", |b| {
        b.iter(|| {
            black_box({
                game = engine.shift(game, Move::Up);
                game = engine.shift(game, Move::Down);
            })
        })
    });
}

pub fn count_empty_bench(c: &mut Criterion) {
    let game = 0x1230300000000000;
    c.bench_function("count non empty", |b| {
        b.iter(|| black_box(engine::count_empty(game)))
    });
}

criterion_group!(
    benches,
    //new_stores_game,
    stores_shift_left,
    stores_shift_right,
    stores_shift_up,
    stores_shift_down,
    no_stores_shift_left,
    no_stores_shift_right,
    no_stores_shift_up,
    no_stores_shift_down,
    unoptimised_shift_left,
    unoptimised_shift_right,
    unoptimised_shift_up,
    unoptimised_shift_down,
    get_score_bench,
    move_left_or_right_bench,
    move_up_or_down_bench,
    count_empty_bench,
);
criterion_main!(benches);
