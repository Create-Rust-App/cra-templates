//! Subcommand implementations.
//!
//! Each feature owns a submodule exposing its arguments struct and a `run`
//! function. Copy `greet.rs` (or the `_feature_template` below) to add a new
//! subcommand, declare it here, and add a variant in [`crate::cli::Commands`].

pub mod _feature_template;
pub mod completions;
pub mod greet;

use crate::{cli::Cli, config::Config};

/// Dispatch the parsed CLI to the selected subcommand.
pub fn run(cli: Cli, config: &Config) -> anyhow::Result<()> {
    match cli.command {
        Some(crate::cli::Commands::Greet(args)) => greet::run(args, config),
        Some(crate::cli::Commands::Completions(args)) => completions::run(args),
        // Bare invocation greets with defaults.
        None => greet::run(
            greet::GreetArgs {
                name: None,
                shout: false,
            },
            config,
        ),
    }
}
