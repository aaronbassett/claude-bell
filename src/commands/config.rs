//! Config command implementation - configuration management

use crate::cli::args::ConfigCommands;
use crate::config::{load_config, save_config, validate_config, Config};
use crate::error::ExitCode;
use anyhow::Result;

/// Handle config subcommands
pub fn handle(action: &ConfigCommands) -> Result<ExitCode> {
    match action {
        ConfigCommands::Show { pretty } => {
            let config = load_config(None)?;
            let json = if *pretty {
                serde_json::to_string_pretty(&config)?
            } else {
                serde_json::to_string(&config)?
            };
            println!("{}", json);
            Ok(ExitCode::Success)
        }
        ConfigCommands::Set { key, value } => {
            println!("Setting {} = {}", key, value);
            // TODO: Implement config set with key path parsing
            Ok(ExitCode::Success)
        }
        ConfigCommands::Unset { key } => {
            println!("Unsetting {}", key);
            // TODO: Implement config unset
            Ok(ExitCode::Success)
        }
        ConfigCommands::Reset => {
            let config = Config::default();
            save_config(&config, None)?;
            println!("Configuration reset to defaults");
            Ok(ExitCode::Success)
        }
        ConfigCommands::Validate { path } => {
            let config = load_config(path.as_ref().map(|p| std::path::Path::new(p)))?;
            match validate_config(&config) {
                Ok(()) => {
                    println!("Configuration is valid");
                    Ok(ExitCode::Success)
                }
                Err(e) => {
                    eprintln!("Configuration error: {}", e);
                    Ok(ExitCode::UserError)
                }
            }
        }
    }
}
