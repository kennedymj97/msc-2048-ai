use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msc_2048_ai::ai::Expectimax;
use msc_2048_ai::ai::AI;
use msc_2048_ai::engine::GameEngine;
use msc_2048_ai::engine::Move;

pub fn get_score_bench(c: &mut Criterion) {
    let mut game = GameEngine::new();
    game.update_state(0x1111222233334444);
    c.bench_function("get score", |b| b.iter(|| black_box(game.get_score())));
}

pub fn move_left_or_right_bench(c: &mut Criterion) {
    let mut game = GameEngine::new();
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
    let mut game = GameEngine::new();
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

pub fn evaluate_chance_bench(c: &mut Criterion) {
    let mut expectimax = Expectimax::new();
    c.bench_function("evaluate chance", |b| {
        b.iter(|| {
            black_box({
                expectimax.evaluate_chance(1);
            })
        })
    });
}

criterion_group!(
    benches,
    get_score_bench,
    move_left_or_right_bench,
    move_up_or_down_bench,
    evaluate_chance_bench,
);
criterion_main!(benches);
