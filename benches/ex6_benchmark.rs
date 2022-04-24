#[macro_use]
extern crate criterion;

use advent_of_code::ex6;
use criterion::Criterion;

fn count_answers_benchmark(c: &mut Criterion) {
    c.bench_function("HashSet", |b| {
        b.iter(|| ex6::count_answers("data_files/ex6.txt"))
    });
}

fn count_answers2_benchmark(c: &mut Criterion) {
    c.bench_function("Sort Dedup", |b| {
        b.iter(|| ex6::count_answers2("data_files/ex6.txt"))
    });
}

fn count_answers3_benchmark(c: &mut Criterion) {
    c.bench_function("Mapper", |b| {
        b.iter(|| ex6::count_answers3("data_files/ex6.txt"))
    });
}

criterion_group!(
    benches,
    count_answers_benchmark,
    count_answers2_benchmark,
    count_answers3_benchmark
);
criterion_main!(benches);
