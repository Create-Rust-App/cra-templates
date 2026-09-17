# Lib Criterion

Feature extension: Criterion benchmarks for library starters. Needs engine
`create-rust-app` 0.4.0+, which merges `[[bench]]` target sections (earlier
releases ignore them).

## What it adds

- `criterion` v0.7 (merged into `[dev-dependencies]`)
- `[[bench]]` target `slug` with `harness = false` (merged by target name)
- `benches/slug.rs` — throughput benchmark for `slug::slugify`, runnable
  with `cargo bench`

## Compatibility

Applies to `library` templates exposing `slug::slugify` (i.e.
`lib-starter`).
