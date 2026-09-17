//! Exponential backoff iterator for retry loops.
//!
//! Pure computation: yields successive delays without sleeping, so callers
//! choose blocking, async, or simulated clocks.

use std::time::Duration;

/// Exponential backoff schedule: `base * factor^attempt`, capped at `max`.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    base: Duration,
    factor: u32,
    max: Duration,
    attempt: u32,
}

impl ExponentialBackoff {
    /// Build a schedule from `base`, multiplying by `factor`, capped at `max`.
    pub fn new(base: Duration, factor: u32, max: Duration) -> Self {
        Self {
            base,
            factor,
            max,
            attempt: 0,
        }
    }

    /// Delay for the next attempt (saturating at `max`).
    pub fn next_delay(&mut self) -> Duration {
        let multiplier = self.factor.saturating_pow(self.attempt);
        self.attempt = self.attempt.saturating_add(1);
        let delay = self.base.saturating_mul(multiplier);
        delay.min(self.max)
    }
}

impl Iterator for ExponentialBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_delay())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_from_base() {
        let delays: Vec<_> =
            ExponentialBackoff::new(Duration::from_millis(100), 2, Duration::from_secs(10))
                .take(3)
                .collect();
        assert_eq!(
            delays,
            vec![
                Duration::from_millis(100),
                Duration::from_millis(200),
                Duration::from_millis(400),
            ]
        );
    }

    #[test]
    fn saturates_at_max() {
        let mut backoff =
            ExponentialBackoff::new(Duration::from_secs(1), 10, Duration::from_secs(5));
        backoff.next_delay();
        assert_eq!(backoff.next_delay(), Duration::from_secs(5));
    }
}
