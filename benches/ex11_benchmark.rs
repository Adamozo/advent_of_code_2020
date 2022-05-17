#[macro_use]
extern crate criterion;

use advent_of_code::ex11;
use criterion::Criterion;

fn count_occupied_seats_benchmark_once_allocated(c: &mut Criterion) {
    let data = ex11::get_data("data_files/ex11.txt").unwrap();
    c.bench_function("Once allocated", |b| {
        b.iter(|| ex11::count_occupied_seats(data.as_str()))
    });
}

fn count_occupied_seats_benchmark_result_board_creation(c: &mut Criterion) {
    let data = ex11::get_data("data_files/ex11.txt").unwrap();
    c.bench_function("New board", |b| {
        b.iter(|| ex11::count_occupied_seats2(data.as_str()))
    });
}

criterion_group!(
    benches,
    count_occupied_seats_benchmark_once_allocated,
    count_occupied_seats_benchmark_result_board_creation
);
criterion_main!(benches);
