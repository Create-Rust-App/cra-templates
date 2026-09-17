//! Binary entry point: tracing setup plus CLI dispatch.

use cli_starter::{cli::Cli, config::Config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    // `--verbose` raises the crate target level when `RUST_LOG` is unset;
    // an explicit `RUST_LOG` always wins.
    let crate_level = match cli.verbose {
        0 => "debug",
        _ => "trace",
    };
    let fallback: tracing_subscriber::EnvFilter =
        format!("{}={crate_level}", env!("CARGO_PKG_NAME").replace('-', "_"))
            .parse()
            .expect("valid default filter");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(fallback))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    if let Err(error) = cli_starter::commands::run(cli, &config) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
