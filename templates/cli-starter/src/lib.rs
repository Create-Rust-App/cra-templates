//! Clap command-line starter.
//!
//! Feature-based layout: each subcommand lives under `src/commands/<feature>/`
//! with its own arguments and behavior. Shared CLI definition lives in
//! [`cli`]; runtime configuration in [`config`].

pub mod cli;
pub mod commands;
pub mod config;
