//! Generate a man page for the CLI.
//!
//! Run: `cargo run --example gen-man > <name>.1`. The page derives from the
//! clap definition, so flags and subcommands can never drift from `--help`.

use clap::CommandFactory;
use cli_starter::cli::Cli;

fn main() {
    let command = Cli::command();
    let man = clap_mangen::Man::new(command);
    let mut buffer = Vec::new();
    man.render(&mut buffer).expect("render man page");
    print!("{}", String::from_utf8(buffer).expect("man page is UTF-8"));
}
