#[macro_use]
extern crate criterion;

use advent_of_code::ex17::*;
use advent_of_code::ex17a::*;
use aoc_utils::{read_to_string, DaySolver};
use criterion::Criterion;

fn ex17_benchmark(c: &mut Criterion) {
    let s = read_to_string(Day17::INFO.file_name).unwrap();
    c.bench_function("vector tuple", |b| b.iter(|| Day17::solution(&s)));
}

fn ex17_benchmark_hashmap(c: &mut Criterion) {
    let s = read_to_string(Day17A::INFO.file_name).unwrap();
    c.bench_function("hashmap", |b| b.iter(|| Day17A::solution(&s)));
}

criterion_group!(benches, ex17_benchmark, ex17_benchmark_hashmap);
criterion_main!(benches);
