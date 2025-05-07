mod matrix;
mod naive;
mod tiled;
mod perf;

use crate::matrix::{generate_matrix};
use crate::naive::naive_multiply;
use crate::tiled::tiled_multiply;
use crate::perf::measure;

const SIZE: usize = 1024;
const BLOCK_SIZE: usize = 64;
const RUNS: usize = 10;

fn benchmark<F>(label: &str, mut f: F)
where
    F: FnMut() + Copy,
{
    let mut durations = Vec::new();
    let mut l1s = Vec::new();
    let mut l2s = Vec::new();

    for _ in 0..RUNS {
        let result = measure(f);
        durations.push(result.duration);
        l1s.push(result.l1_misses);
        l2s.push(result.l2_misses);
    }

    let avg = |v: &Vec<f64>| v.iter().sum::<f64>() / v.len() as f64;
    let avg_u64 = |v: &Vec<u64>| v.iter().sum::<u64>() / v.len() as u64;

    println!("{}:", label);
    println!("  Avg Time: {:.6} seconds", avg(&durations));
    println!("  Avg L1 Misses: {}", avg_u64(&l1s));
    println!("  Avg L2 Misses: {}", avg_u64(&l2s));
    println!();
}

fn main() {
    let a = generate_matrix(SIZE);
    let b = generate_matrix(SIZE);

    benchmark("Naive", || {
        let _ = naive_multiply(&a, &b);
    });

    benchmark("Tiled", || {
        let _ = tiled_multiply(&a, &b, BLOCK_SIZE);
    });
}