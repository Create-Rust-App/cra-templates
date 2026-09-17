//! Runtime configuration loaded from the environment.

/// Runtime configuration for the gRPC service.
///
/// Every field has a default so the service boots with no setup; override
/// with `HOST` or `PORT` (see `.env.example`).
#[derive(Debug, Clone)]
pub struct Config {
    /// Interface to bind, e.g. `0.0.0.0`.
    pub host: String,
    /// TCP port to listen on.
    pub port: u16,
}

impl Config {
    /// Load configuration from the environment, falling back to defaults.
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(50051);
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        Self { host, port }
    }

    /// Socket address to bind, e.g. `0.0.0.0:50051`.
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 50051,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_env_example() {
        let config = Config::default();
        assert_eq!(config.addr(), "0.0.0.0:50051");
    }
}
