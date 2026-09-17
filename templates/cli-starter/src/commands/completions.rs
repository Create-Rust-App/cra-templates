//! Completions subcommand: print shell completions to stdout.

/// Supported shells for completion generation.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    /// Bourne Again Shell.
    Bash,
    /// Z shell.
    Zsh,
    /// Fish shell.
    Fish,
    /// PowerShell.
    Powershell,
}

/// Arguments for the `completions` subcommand.
#[derive(Debug, clap::Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    pub shell: Shell,
}

/// Run the `completions` subcommand.
pub fn run(args: CompletionsArgs) -> anyhow::Result<()> {
    use clap_complete::{generate, shells};
    use std::io::stdout;

    let mut command = <crate::cli::Cli as clap::CommandFactory>::command();
    match args.shell {
        Shell::Bash => generate(shells::Bash, &mut command, "cli-starter", &mut stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut command, "cli-starter", &mut stdout()),
        Shell::Fish => generate(shells::Fish, &mut command, "cli-starter", &mut stdout()),
        Shell::Powershell => generate(
            shells::PowerShell,
            &mut command,
            "cli-starter",
            &mut stdout(),
        ),
    }
    Ok(())
}
