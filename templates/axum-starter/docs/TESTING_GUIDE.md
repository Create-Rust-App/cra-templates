# Testing Guide — axum-starter

## Commands

```sh
cargo test                                     # unit + integration
cargo test -- --nocapture                      # with log output
cargo fmt --all -- --check                     # formatting gate
cargo clippy --all-targets -- -D warnings      # lint gate
```

## Strategy

- Unit tests live next to the code (`#[cfg(test)]`), e.g. config defaults.
- Integration tests live in `tests/` and exercise the full router in-process
  with `tower::ServiceExt::oneshot` — no socket is bound.
- Every route needs at least one integration test asserting status and body.
