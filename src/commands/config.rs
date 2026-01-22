//! Config command implementation - configuration management

use crate::cli::args::ConfigCommands;
use crate::config::{load_config, save_config, validate_config, Config};
use crate::error::ExitCode;
use anyhow::Result;

fn set_from_key_path(config: &mut Config, key: &str, value: &str) -> Result<()> {
    let parts: Vec<&str> = key.split('.').collect();

    match parts.as_slice() {
        ["defaults", "sound"] => {
            config.defaults.sound = Some(value.to_string());
        }
        ["defaults", "icon"] => {
            config.defaults.icon = Some(value.to_string());
        }
        ["defaults", "persistent"] => {
            config.defaults.persistent = value.parse()
                .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
        }
        ["defaults", "log_level"] => {
            config.defaults.log_level = value.to_string();
        }
        ["timeout"] => {
            config.timeout = Some(value.to_string());
        }
        ["on_dismiss"] => {
            config.on_dismiss = Some(value.to_string());
        }
        ["on_timeout"] => {
            config.on_timeout = Some(value.to_string());
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown config key: {}", key));
        }
    }

    Ok(())
}

fn unset_key_path(config: &mut Config, key: &str) -> Result<()> {
    let parts: Vec<&str> = key.split('.').collect();

    match parts.as_slice() {
        ["defaults", "sound"] => {
            config.defaults.sound = None;
        }
        ["defaults", "icon"] => {
            config.defaults.icon = None;
        }
        ["defaults", "persistent"] => {
            config.defaults.persistent = false;
        }
        ["timeout"] => {
            config.timeout = None;
        }
        ["on_dismiss"] => {
            config.on_dismiss = None;
        }
        ["on_timeout"] => {
            config.on_timeout = None;
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown config key: {}", key));
        }
    }

    Ok(())
}

fn set_from_json(config: &mut Config) -> Result<()> {
    use std::io::Read;

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let partial: serde_json::Value = serde_json::from_str(&buffer)?;

    // Merge JSON into config
    if let Some(defaults) = partial.get("defaults") {
        if let Some(sound) = defaults.get("sound") {
            config.defaults.sound = sound.as_str().map(String::from);
        }
        if let Some(icon) = defaults.get("icon") {
            config.defaults.icon = icon.as_str().map(String::from);
        }
        if let Some(persistent) = defaults.get("persistent") {
            if let Some(b) = persistent.as_bool() {
                config.defaults.persistent = b;
            }
        }
        if let Some(log_level) = defaults.get("log_level") {
            if let Some(s) = log_level.as_str() {
                config.defaults.log_level = s.to_string();
            }
        }
    }

    if let Some(timeout) = partial.get("timeout") {
        config.timeout = timeout.as_str().map(String::from);
    }
    if let Some(on_dismiss) = partial.get("on_dismiss") {
        config.on_dismiss = on_dismiss.as_str().map(String::from);
    }
    if let Some(on_timeout) = partial.get("on_timeout") {
        config.on_timeout = on_timeout.as_str().map(String::from);
    }

    Ok(())
}

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
        ConfigCommands::Set { key, value, json } => {
            let mut config = load_config(None)?;

            if *json {
                set_from_json(&mut config)?;
            } else if let (Some(k), Some(v)) = (key, value) {
                set_from_key_path(&mut config, k, v)?;
            } else {
                return Err(anyhow::anyhow!("Either provide key/value or use --json").into());
            }

            save_config(&config, None)?;
            println!("Configuration updated");
            Ok(ExitCode::Success)
        }
        ConfigCommands::Unset { key } => {
            let mut config = load_config(None)?;
            unset_key_path(&mut config, key)?;
            save_config(&config, None)?;
            println!("Configuration key removed: {}", key);
            Ok(ExitCode::Success)
        }
        ConfigCommands::Reset => {
            let config = Config::default();
            save_config(&config, None)?;
            println!("Configuration reset to defaults");
            Ok(ExitCode::Success)
        }
        ConfigCommands::Validate { path } => {
            let config = load_config(path.as_ref().map(std::path::Path::new))?;
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
