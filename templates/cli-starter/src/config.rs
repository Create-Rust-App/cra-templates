//! Runtime configuration loaded from the environment.

/// Runtime configuration for the CLI.
///
/// Every field has a default so the tool runs with no setup; override with
/// `CLI_DEFAULT_NAME` (see `.env.example`).
#[derive(Debug, Clone)]
pub struct Config {
    /// Name greeted when `--name` is omitted, e.g. `world`.
    pub default_name: String,
}

impl Config {
    /// Load configuration from the environment, falling back to defaults.
    pub fn from_env() -> Self {
        let default_name =
            std::env::var("CLI_DEFAULT_NAME").unwrap_or_else(|_| "world".to_string());
        Self { default_name }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_name: "world".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_env_example() {
        let config = Config::default();
        assert_eq!(config.default_name, "world");
    }
}
