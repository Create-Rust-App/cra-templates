# Testing guide

- Unit tests live next to the code (`greet::greeting`, `Config` defaults).
- `tests/test_cli.rs` drives the built binary end to end; the binary is
  located via `CARGO_PKG_NAME`, so tests survive project renames.
- Run everything with `cargo test`.
