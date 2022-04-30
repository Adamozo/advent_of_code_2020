#[macro_use]
extern crate criterion;

use advent_of_code::ex6;
use criterion::Criterion;

fn get_data() -> String {
    "abc

    a
    b
    c
    
    ab
    ac
    
    a
    a
    a
    a
    
    b"
    .to_string()
}

fn count_answers_benchmark(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("HashSet", |b| {
        b.iter(|| ex6::count_answers_bench(data.as_str()))
    });
}

fn count_answers2_benchmark(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("Sort Dedup", |b| {
        b.iter(|| ex6::count_answers2_bench(data.as_str()))
    });
}

fn count_answers3_benchmark(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("Mapper", |b| {
        b.iter(|| ex6::count_answers3_bench(data.as_str()))
    });
}

fn count_answers4_benchmark(c: &mut Criterion) {
    let data = get_data();
    c.bench_function("Mentor", |b| b.iter(|| ex6::count_answers4(data.as_str())));
}

criterion_group!(
    benches,
    count_answers_benchmark,
    count_answers2_benchmark,
    count_answers3_benchmark,
    count_answers4_benchmark
);
criterion_main!(benches);
