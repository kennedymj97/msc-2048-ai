// #![feature(test)]

// extern crate test;

pub mod ai;
pub mod engine;
pub mod ui;

// https://dev.to/robertorojasr/rust-project-structure-example-step-by-step-3ee
// ^^ good summary of how the module system works in rust.

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
