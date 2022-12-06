use criterion::{criterion_group, criterion_main, Criterion};
use day06::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve 4", |b| b.iter(|| solve::<PACKET_MARKER_SIZE>(INPUT)));
    c.bench_function("solve 14", |b| {
        b.iter(|| solve::<MESSAGE_MARKER_SIZE>(INPUT))
    });
    c.bench_function("solve_faster 4", |b| {
        b.iter(|| solve_faster::<PACKET_MARKER_SIZE>(INPUT))
    });
    c.bench_function("solve_faster 14", |b| {
        b.iter(|| solve_faster::<MESSAGE_MARKER_SIZE>(INPUT))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
