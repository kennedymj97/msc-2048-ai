use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msc_2048_ai::engine::Basic;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn get_score_bench(c: &mut Criterion) {
    let mut game = Basic::new();
    game.update_state(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]);
    c.bench_function("get score basic", |b| {
        b.iter(|| black_box(game.get_score()))
    });
}

criterion_group!(benches, criterion_benchmark, get_score_bench);
criterion_main!(benches);
