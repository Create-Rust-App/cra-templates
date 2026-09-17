# Deployment

```sh
cargo build --release            # -> target/release/leptos-starter
```

Serve the single binary behind a reverse proxy; no static assets are
required (htmx loads from CDN).

## Hydrated islands (optional upgrade)

1. Install `cargo install cargo-leptos`.
2. Mark interactive components with `#[island]` and add a hydration script.
3. `cargo leptos build --release` produces `target/site` + the server binary.

See the [cargo-leptos book](https://github.com/leptos-rs/cargo-leptos) for
the full workflow.
