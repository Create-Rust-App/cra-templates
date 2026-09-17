//! Demo binary exercising the public library API end to end.

use lib_starter::{backoff::ExponentialBackoff, slug::slugify};
use std::time::Duration;

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Hello, World!".to_string());
    println!("slug: {}", slugify(&name));
    let schedule = ExponentialBackoff::new(Duration::from_millis(100), 2, Duration::from_secs(5));
    for (attempt, delay) in schedule.take(4).enumerate() {
        println!("attempt {}: wait {:?}", attempt + 1, delay);
    }
}
