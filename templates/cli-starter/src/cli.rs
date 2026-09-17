//! Command-line definition (clap derive).

use clap::{Parser, Subcommand};

/// Command-line interface for the starter tool.
#[derive(Debug, Parser)]
#[command(
    name = "cli-starter",
    version,
    about = "Clap CLI starter",
    after_help = "Examples:\n  cli-starter greet --name Ferris\n  cli-starter greet --name Ferris --shout\n  cli-starter completions bash > ~/.local/share/bash-completion/completions/cli-starter"
)]
pub struct Cli {
    /// Increase log verbosity (repeat for more detail).
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Subcommand to run. Defaults to `greet` when omitted.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands; one variant per feature module.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print a greeting.
    Greet(crate::commands::greet::GreetArgs),
    /// Print shell completions to stdout.
    Completions(crate::commands::completions::CompletionsArgs),
}
