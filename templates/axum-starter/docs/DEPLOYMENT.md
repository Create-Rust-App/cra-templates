# Deployment — axum-starter

## Release build

```sh
cargo build --release
./target/release/axum-starter
```

The release profile enables LTO with a single codegen unit (see
`Cargo.toml`).

## Containers

Apply the `axum-docker` extension for a multi-stage Dockerfile, or build
your own from `rust:1.82` pinning the same toolchain as
`rust-toolchain.toml`.

## Runtime

- The service reads `HOST`, `PORT`, and `API_PREFIX` from the environment.
- `/ping` is the load-balancer health probe; `GET {API_PREFIX}/healthz` is
  the versioned service check.
