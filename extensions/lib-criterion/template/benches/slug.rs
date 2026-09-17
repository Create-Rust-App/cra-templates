//! Slug benchmarks: track `slugify` throughput across changes.
//!
//! Run with `cargo bench`. Baselines persist under `target/criterion`, so
//! regressions surface as soon as a change lands.

use criterion::{criterion_group, criterion_main, Criterion};
use lib_starter::slug::slugify;

fn bench_slugify(criterion: &mut Criterion) {
    criterion.bench_function("slugify display name", |benches| {
        benches.iter(|| slugify("Hello, Beautiful World of Rust!"))
    });
}

criterion_group!(benches, bench_slugify);
criterion_main!(benches);
