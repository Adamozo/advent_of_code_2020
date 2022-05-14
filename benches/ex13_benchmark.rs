#[macro_use]
extern crate criterion;

use advent_of_code::ex13::*;
use criterion::Criterion;

fn ex13_benchmark(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("n^2", |b| b.iter(|| get_bus_mult_minutes(data.0, &data.1)));
}

fn ex13_benchmark2(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("modulo", |b| {
        b.iter(|| get_bus_mult_minutes2(data.0, &data.1))
    });
}

fn ex13_benchmark3(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("min_by_key", |b| {
        b.iter(|| get_bus_mult_minutes3(data.0, &data.1))
    });
}

criterion_group!(benches, ex13_benchmark, ex13_benchmark2, ex13_benchmark3);
criterion_main!(benches);
