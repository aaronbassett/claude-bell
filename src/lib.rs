pub mod cli;
pub mod config;
pub mod error;
pub mod notification;
pub mod template;
pub mod alias;
pub mod doctor;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;
pub use error::{AppError, ExitCode};

pub fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    // For now, just print what we parsed
    println!("{:?}", cli);

    Ok(ExitCode::Success)
}
