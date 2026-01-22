//! claude-bell - macOS notifications for Claude Code

pub mod alias;
pub mod cli;
pub mod commands;
pub mod config;
pub mod doctor;
pub mod error;
pub mod notification;
pub mod template;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;
pub use error::{AppError, ExitCode};

/// Run the application with parsed CLI arguments
pub fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    commands::dispatch(&cli)
}
