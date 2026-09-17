# Tonic Starter

[gRPC](https://grpc.io) microservice starter on [tonic](https://github.com/hyperium/tonic):
Protobuf contract, checked-in bindings, server reflection for `grpcurl`,
graceful shutdown, and `cargo fmt` / `clippy` / `test` quality gates.

## Run

```sh
cargo run
```

Call it (reflection enabled, no codegen needed):

```sh
grpcurl -plaintext localhost:50051 hello.Greeter/SayHello
grpcurl -plaintext localhost:50051 list
```

Configure with environment variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `HOST` | `0.0.0.0` | Interface to bind |
| `PORT` | `50051` | TCP port |
| `RUST_LOG` | `tonic_starter=debug` | Log filter |

## Protobuf workflow

`proto/hello.proto` is the source of truth; `src/proto/hello.rs` and
`src/proto/hello_descriptor.bin` are checked-in generator outputs so builds
never need `protoc`. To evolve the contract, follow `proto/REGENERATE.md`.

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — RPC reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — deployment
