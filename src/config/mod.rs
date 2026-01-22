//! Configuration loading, validation, and management

mod schema;

pub use schema::{Config, Defaults};

use crate::error::AppError;
use std::path::{Path, PathBuf};

/// Get the default config directory path
pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".claude-bell")
}

/// Load configuration from file
pub fn load_config(path: Option<&Path>) -> Result<Config, AppError> {
    let config_path = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| config_dir().join("config.json"));

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config, path: Option<&Path>) -> Result<(), AppError> {
    let config_path = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| config_dir().join("config.json"));

    // Ensure directory exists
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(&config_path, content)?;
    Ok(())
}
