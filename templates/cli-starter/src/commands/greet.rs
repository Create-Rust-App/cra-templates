//! Greet subcommand: print a greeting.

use crate::config::Config;

/// Arguments for the `greet` subcommand.
#[derive(Debug, clap::Args)]
pub struct GreetArgs {
    /// Name to greet. Defaults to `CLI_DEFAULT_NAME`.
    #[arg(long)]
    pub name: Option<String>,

    /// Uppercase the greeting.
    #[arg(long)]
    pub shout: bool,
}

/// Render the greeting for `name`.
pub fn greeting(name: &str, shout: bool) -> String {
    let message = format!("Hello, {name}!");
    if shout {
        message.to_uppercase()
    } else {
        message
    }
}

/// Run the `greet` subcommand.
pub fn run(args: GreetArgs, config: &Config) -> anyhow::Result<()> {
    let name = args.name.as_deref().unwrap_or(&config.default_name);
    println!("{}", greeting(name, args.shout));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_formats_name() {
        assert_eq!(greeting("world", false), "Hello, world!");
    }

    #[test]
    fn shout_uppercases_greeting() {
        assert_eq!(greeting("world", true), "HELLO, WORLD!");
    }
}
