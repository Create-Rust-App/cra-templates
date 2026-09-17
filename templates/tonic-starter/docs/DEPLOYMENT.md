# Deployment

```sh
cargo build --release            # -> target/release/tonic-starter
```

Ship the single binary; no assets or sidecars required. Terminate with
SIGINT/SIGTERM for graceful shutdown (in-flight RPCs drain).
