pub mod cli;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // For now, just print what we parsed
    println!("{:?}", cli);

    Ok(())
}
