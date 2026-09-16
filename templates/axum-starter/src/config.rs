//! Runtime configuration loaded from the environment.

/// Runtime configuration for the HTTP service.
///
/// Every field has a default so the service boots with no setup;
/// override with `HOST`, `PORT`, or `API_PREFIX` (see `.env.example`).
#[derive(Debug, Clone)]
pub struct Config {
    /// Interface to bind, e.g. `0.0.0.0`.
    pub host: String,
    /// TCP port to listen on.
    pub port: u16,
    /// URL prefix for versioned API routes, e.g. `/api/v1`.
    pub api_prefix: String,
}

impl Config {
    /// Load configuration from the environment, falling back to defaults.
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8080);
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let api_prefix = std::env::var("API_PREFIX").unwrap_or_else(|_| "/api/v1".to_string());
        Self {
            host,
            port,
            api_prefix,
        }
    }

    /// Socket address to bind, e.g. `0.0.0.0:8080`.
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            api_prefix: "/api/v1".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_env_example() {
        let config = Config::default();
        assert_eq!(config.addr(), "0.0.0.0:8080");
        assert_eq!(config.api_prefix, "/api/v1");
    }
}
