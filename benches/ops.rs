#![allow(clippy::wildcard_imports)]
use criterion::{Criterion, criterion_group, criterion_main};

#[path = "../tests/add.rs"]
mod add;

#[path = "../tests/div.rs"]
mod div;

#[path = "../tests/mul.rs"]
mod mul;

fn add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");
    group.bench_function("all_c", |b| b.iter(add::all_c_tests));
}
fn mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");
    group.bench_function("all_c", |b| b.iter(mul::all_c_tests));
}
fn div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");
    group.bench_function("all_c", |b| b.iter(div::all_c_tests));
}

criterion_group!(benches, add, mul, div);
criterion_main!(benches);
