#[macro_use]
extern crate criterion;

use advent_of_code::ex15::*;
use advent_of_code::ex15b::*;
use advent_of_code::ex15c::*;
use aoc_utils::DaySolver;
use criterion::Criterion;

fn ex13_benchmark(c: &mut Criterion) {
    c.bench_function("vector tuple", |b| {
        b.iter(|| Day15VersionA::solution("0,3,6"))
    });
}

fn ex13_benchmark2(c: &mut Criterion) {
    c.bench_function("hash map", |b| {
        b.iter(|| Day15VersionB::solution("0,3,6"))
    });
}

fn ex13_benchmark3(c: &mut Criterion) {
    c.bench_function("vector with capacity", |b| {
        b.iter(|| Day15VersionC::solution("0,3,6"))
    });
}

criterion_group!(benches, ex13_benchmark, ex13_benchmark2, ex13_benchmark3);
criterion_main!(benches);
