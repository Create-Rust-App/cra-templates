# RPC reference

Package `hello`, service `Greeter` (`proto/hello.proto`):

| RPC | Request | Reply |
|-----|---------|-------|
| `SayHello` | `HelloRequest { name }` | `HelloReply { message: "Hello, {name}!" }` |

Server reflection (v1) is enabled: explore with `grpcurl -plaintext
$HOST:$PORT list` and call with `grpcurl -plaintext $HOST:$PORT
hello.Greeter/SayHello`.
