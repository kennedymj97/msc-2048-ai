use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msc_2048_ai::engine as GameEngine;
use msc_2048_ai::engine::Move;

pub fn get_score_bench(c: &mut Criterion) {
    let game = 0x1111222233334444;
    c.bench_function("get score", |b| {
        b.iter(|| black_box(GameEngine::get_score(game)))
    });
}

pub fn move_left_or_right_bench(c: &mut Criterion) {
    let mut game = 0x1111222233334444;
    c.bench_function("move left or right", |b| {
        b.iter(|| {
            black_box({
                game = GameEngine::shift(game, Move::Left);
                game = GameEngine::shift(game, Move::Right);
            })
        })
    });
}

pub fn move_up_or_down_bench(c: &mut Criterion) {
    let mut game = 0x1234123412341234;
    c.bench_function("move up or down", |b| {
        b.iter(|| {
            black_box({
                game = GameEngine::shift(game, Move::Up);
                game = GameEngine::shift(game, Move::Down);
            })
        })
    });
}

pub fn count_empty_bench(c: &mut Criterion) {
    let game = 0x1230300000000000;
    c.bench_function("count non empty", |b| {
        b.iter(|| black_box(GameEngine::count_empty(game)))
    });
}

criterion_group!(
    benches,
    get_score_bench,
    move_left_or_right_bench,
    move_up_or_down_bench,
    count_empty_bench,
);
criterion_main!(benches);
