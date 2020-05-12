use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Move;
use msc_2048_ai::engine::Optimised;
use msc_2048_ai::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn get_score_bench(c: &mut Criterion) {
    let mut game = Optimised::new();
    game.update_state(0x1111222233334444);
    c.bench_function("get score", |b| b.iter(|| black_box(game.get_score())));
}

pub fn move_left_or_right_bench(c: &mut Criterion) {
    let mut game = Optimised::new();
    game.update_state(0x1111222233334444);
    c.bench_function("move left or right", |b| {
        b.iter(|| {
            black_box({
                game.move_left_or_right(Move::Left);
                game.move_left_or_right(Move::Right);
            })
        })
    });
}

pub fn move_up_or_down_bench(c: &mut Criterion) {
    let mut game = Optimised::new();
    game.update_state(0x1234123412341234);
    c.bench_function("move up or down", |b| {
        b.iter(|| {
            black_box({
                game.move_up_or_down(Move::Up);
                game.move_up_or_down(Move::Down);
            })
        })
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    get_score_bench,
    move_left_or_right_bench,
    move_up_or_down_bench,
);
criterion_main!(benches);
