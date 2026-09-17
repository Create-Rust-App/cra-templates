# Lib Starter

Publishable Rust library starter: feature-based modules, documentation
examples, demo binary, and `cargo fmt` / `clippy` / `test` quality gates.

## Use

```rust
use lib_starter::{backoff::ExponentialBackoff, slug::slugify};
use std::time::Duration;

let slug = slugify("Hello, World!"); // "hello-world"
let delays: Vec<_> =
    ExponentialBackoff::new(Duration::from_millis(100), 2, Duration::from_secs(5))
        .take(3)
        .collect();
```

Run the demo binary:

```sh
cargo run -- "Hello, World!"
```

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Publishing

The manifest ships publish-ready metadata (`description`, `license`,
`readme`, `repository`, `keywords`, `categories`): bump `version` and run
`cargo publish --dry-run`, then `cargo publish`.

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — API reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — publishing
