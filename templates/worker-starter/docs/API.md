# Job reference

Jobs are serde payloads dispatched by `jobs::dispatch`:

| Variant | Payload | Effect |
|---------|---------|--------|
| `Job::Greet` | `GreetPayload { name }` | Logs and reports `greeted {name}` |

Submit awaitably with `Producer::submit` (returns the `JobResult`) or
fire-and-forget with `Producer::enqueue`.
