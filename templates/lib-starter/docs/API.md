# API reference

## `slug::slugify(text: &str) -> String`

Lowercase kebab-case slug: splits on non-alphanumeric runs, drops empties,
joins with `-`. Returns `""` when nothing alphanumeric remains.

## `backoff::ExponentialBackoff`

Pure iterator schedule: `base * factor^attempt`, capped at `max`.

```rust
use lib_starter::backoff::ExponentialBackoff;
use std::time::Duration;

let mut schedule =
    ExponentialBackoff::new(Duration::from_millis(100), 2, Duration::from_secs(5));
assert_eq!(schedule.next_delay(), Duration::from_millis(100));
```
