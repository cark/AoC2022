use criterion::{criterion_group, criterion_main, Criterion};
use day13::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(|| part1(INPUT)));
    c.bench_function("part 2", |b| b.iter(|| part2(INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
