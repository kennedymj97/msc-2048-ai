mod expectimax;
mod random;

pub use self::expectimax::BasicExpectimax;
pub use self::random::BasicRandom;

pub trait AI {
    fn evaluate(num_iters: u64) {
        let average_score = (0..num_iters).fold(0, |acc, _| {
            let score = Self::run();
            acc + (score / num_iters)
        });
        println!("Average score: {}", average_score);
    }

    fn run() -> u64;
}
