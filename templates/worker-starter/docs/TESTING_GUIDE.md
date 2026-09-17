# Testing guide

- Unit tests live next to the code (job executors, config defaults).
- `tests/test_queue.rs` runs real pools: submit-and-await plus shutdown.
- Run everything with `cargo test`.
