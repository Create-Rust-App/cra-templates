# Deployment

```sh
cargo build --release            # -> target/release/worker-starter
```

Ship the single binary under a supervisor (systemd, Kubernetes); SIGINT
stops the scheduler and drains workers via the shutdown watch.
