#[macro_use]
extern crate criterion;

use std::fs::read_to_string;

use advent_of_code::ex7::Day7;
use aoc_utils::*;
use criterion::Criterion;

fn ex7_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| {
        let t = read_to_string("data_files/ex7.txt").unwrap();
        b.iter(|| Day7::solution(t.as_str()))
    });
}

criterion_group!(benches, ex7_benchmark);
criterion_main!(benches);
