# Testing guide

- Unit tests live next to the code (backoff math, slug edge cases).
- `tests/test_lib.rs` covers the public API surface.
- Doc examples on public items run as doctests under `cargo test`.
- Run everything with `cargo test`.
