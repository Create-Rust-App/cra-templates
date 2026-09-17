# Testing guide

- Unit tests live next to the code (service method, config defaults).
- `tests/test_grpc.rs` serves on an OS-assigned port and drives a real
  `GreeterClient` over the wire: greeting content plus config defaults.
- Run everything with `cargo test`.
