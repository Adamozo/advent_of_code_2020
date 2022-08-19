#[macro_use]
extern crate criterion;

use std::fs::read_to_string;

use advent_of_code::ex25::Day25;
use advent_of_code::ex25b::Day25b;
use aoc_utils::*;
use criterion::Criterion;

fn ex25_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| {
        let t = read_to_string("data_files/ex25.txt").unwrap();
        b.iter(|| Day25::solution(t.as_str()))
    });

    c.bench_function("perf", |b| {
        let t = read_to_string("data_files/ex25.txt").unwrap();
        b.iter(|| Day25b::solution(t.as_str()))
    });
}

criterion_group!(benches, ex25_benchmark);
criterion_main!(benches);
