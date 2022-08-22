#[macro_use]
extern crate criterion;

use advent_of_code::ex23::*;
use advent_of_code::ex23b::*;
use aoc_utils::DaySolver;
use criterion::Criterion;

fn ex23_benchmark(c: &mut Criterion) {
    c.bench_function("base", |b| {
        b.iter(|| Day23::solution("389125467"))
    });
}

fn ex23b_benchmark(c: &mut Criterion) {
    c.bench_function("new cycle list", |b| {
        b.iter(|| Day23b::solution("389125467"))
    });
}

criterion_group!(benches, ex23_benchmark, ex23b_benchmark);
criterion_main!(benches);
