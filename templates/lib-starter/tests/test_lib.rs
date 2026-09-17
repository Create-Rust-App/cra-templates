//! Integration tests for the public library API.

use lib_starter::{backoff::ExponentialBackoff, slug::slugify};
use std::time::Duration;

#[test]
fn public_backoff_schedule() {
    let delays: Vec<_> =
        ExponentialBackoff::new(Duration::from_millis(50), 2, Duration::from_secs(1))
            .take(2)
            .collect();
    assert_eq!(
        delays,
        vec![Duration::from_millis(50), Duration::from_millis(100)]
    );
}

#[test]
fn public_slugify() {
    assert_eq!(slugify("Create Rust App"), "create-rust-app");
}
