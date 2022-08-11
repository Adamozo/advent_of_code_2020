#[macro_use]
extern crate criterion;

use std::fs::read_to_string;

use advent_of_code::ex20::*;
use advent_of_code::ex20b::*;
use aoc_utils::*;
use criterion::Criterion;

fn ex20_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| {
        let t = read_to_string("data_files/ex20.txt").unwrap();
        b.iter(|| Day20::solution(t.as_str()))
    });
}

fn ex20b_benchmark(c: &mut Criterion) {
    c.bench_function("fast", |b| {
        let t = read_to_string("data_files/ex20.txt").unwrap();
        b.iter(|| Day20b::solution(t.as_str()))
    });
}

criterion_group!(benches, ex20_benchmark, ex20b_benchmark);
criterion_main!(benches);
