//! Command implementations for CLI subcommands

pub mod config;
pub mod doctor;
pub mod icon;
pub mod sound;
pub mod template;

use crate::cli::args::{Cli, Commands};
use crate::error::ExitCode;
use anyhow::Result;

/// Dispatch command based on CLI args
pub fn dispatch(cli: &Cli) -> Result<ExitCode> {
    match &cli.command {
        Some(Commands::Template { action }) => template::handle(action),
        Some(Commands::Sound { action }) => sound::handle(action),
        Some(Commands::Icon { action }) => icon::handle(action),
        Some(Commands::Config { action }) => config::handle(action),
        Some(Commands::Setup) => {
            println!("Running setup wizard...");
            // TODO: Implement setup wizard
            Ok(ExitCode::Success)
        }
        Some(Commands::Doctor) => doctor::handle(),
        None => {
            // No subcommand - send notification
            if cli.title.is_some() {
                // TODO: Send notification
                println!("Would send notification: {:?}", cli.title);
                Ok(ExitCode::Success)
            } else {
                // No title and no command - show help
                println!("Use --help for usage information");
                Ok(ExitCode::Success)
            }
        }
    }
}
