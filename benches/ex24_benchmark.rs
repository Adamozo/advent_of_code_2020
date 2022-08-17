#[macro_use]
extern crate criterion;

use std::fs::read_to_string;

use advent_of_code::ex24::Day24;
use advent_of_code::ex24pdo::Day24b;
use aoc_utils::*;
use criterion::Criterion;

fn ex24_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| {
        let t = read_to_string("data_files/ex24.txt").unwrap();
        b.iter(|| Day24::solution(t.as_str()))
    });
}

fn ex24b_benchmark(c: &mut Criterion) {
    c.bench_function("cleaned", |b| {
        let t = read_to_string("data_files/ex24.txt").unwrap();
        b.iter(|| Day24b::solution(t.as_str()))
    });
}

criterion_group!(benches, ex24_benchmark, ex24b_benchmark);
criterion_main!(benches);
