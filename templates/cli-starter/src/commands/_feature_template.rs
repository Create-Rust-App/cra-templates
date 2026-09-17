//! Template for a new subcommand module.
//!
//! Copy this file to `<feature>.rs`, declare it in `mod.rs`, and add a
//! variant in [`crate::cli::Commands`]. Delete this module if unused.

use crate::config::Config;

/// Arguments for the feature subcommand.
#[derive(Debug, clap::Args)]
pub struct FeatureArgs {
    /// Placeholder flag.
    #[arg(long)]
    pub flag: bool,
}

/// Run the feature subcommand.
pub fn run(_args: FeatureArgs, _config: &Config) -> anyhow::Result<()> {
    println!("feature ok");
    Ok(())
}
