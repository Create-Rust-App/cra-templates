//! Runtime configuration loaded from the environment.

/// Runtime configuration for the worker pool.
///
/// Every field has a default so workers boot with no setup; override with
/// `WORKER_COUNT` or `TICK_SECS` (see `.env.example`).
#[derive(Debug, Clone)]
pub struct Config {
    /// Number of worker tasks in the pool.
    pub workers: usize,
    /// Seconds between demo scheduler ticks.
    pub tick_secs: u64,
}

impl Config {
    /// Load configuration from the environment, falling back to defaults.
    pub fn from_env() -> Self {
        let workers = std::env::var("WORKER_COUNT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(4);
        let tick_secs = std::env::var("TICK_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(30);
        Self { workers, tick_secs }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workers: 4,
            tick_secs: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_env_example() {
        let config = Config::default();
        assert_eq!(config.workers, 4);
        assert_eq!(config.tick_secs, 30);
    }
}
