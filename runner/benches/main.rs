use criterion::{criterion_group, criterion_main};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

aoc_solutions::bench_days! {
    _2024: [seven],
}
