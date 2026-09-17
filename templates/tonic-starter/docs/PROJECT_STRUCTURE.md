# Project structure

```text
proto/
  hello.proto             # contract source of truth
  REGENERATE.md           # binding regeneration contract
src/
  main.rs                 # tracing setup + serve loop + shutdown
  lib.rs                  # module root (config, proto, service)
  config.rs               # Config::from_env with defaults
  service.rs              # Greeter implementation
  proto.rs                # binding module root + descriptor set
  proto/
    hello.rs              # generated bindings (checked in, do not edit)
    hello_descriptor.bin  # reflection descriptor set (generated)
tests/
  test_grpc.rs            # end-to-end RPC tests (ephemeral port)
```
