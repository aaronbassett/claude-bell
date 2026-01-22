//! Health check implementations

use crate::alias::{load_icon_aliases, load_sound_aliases, sounds_dir};
use crate::config;
use std::path::PathBuf;

/// Health check result
#[derive(Debug)]
pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Ok,
    Warning,
    Error,
}

impl CheckResult {
    pub fn ok(name: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Ok,
            message: message.to_string(),
            hint: None,
        }
    }

    pub fn warning(name: &str, message: &str, hint: &str) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Warning,
            message: message.to_string(),
            hint: Some(hint.to_string()),
        }
    }

    pub fn error(name: &str, message: &str, hint: &str) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Error,
            message: message.to_string(),
            hint: Some(hint.to_string()),
        }
    }
}

/// Run all health checks
pub fn run_all_checks() -> Vec<CheckResult> {
    let mut results = vec![];

    results.push(check_config_directory());
    results.push(check_config_file());
    results.extend(check_sound_aliases());
    results.extend(check_icon_aliases());

    results
}

/// Check config directory exists
pub fn check_config_directory() -> CheckResult {
    let config_dir = config::config_dir();

    if config_dir.exists() {
        CheckResult::ok(
            "Config directory",
            &format!("exists at {}", config_dir.display()),
        )
    } else {
        CheckResult::warning(
            "Config directory",
            "missing",
            "Run 'cb setup' to create configuration directory",
        )
    }
}

/// Check config file is valid
pub fn check_config_file() -> CheckResult {
    let config_path = config::config_dir().join("config.json");

    if !config_path.exists() {
        return CheckResult::ok("Config file", "not present (using defaults)");
    }

    match config::load_config(None) {
        Ok(config) => match config::validate_config(&config) {
            Ok(()) => CheckResult::ok("Config file", "valid"),
            Err(e) => CheckResult::error(
                "Config file",
                &format!("invalid: {}", e),
                "Run 'cb config validate' for details",
            ),
        },
        Err(e) => CheckResult::error(
            "Config file",
            &format!("parse error: {}", e),
            "Check JSON syntax in config.json",
        ),
    }
}

/// Check sound aliases
pub fn check_sound_aliases() -> Vec<CheckResult> {
    let mut results = vec![];

    match load_sound_aliases() {
        Ok(aliases) => {
            let mut dangling = vec![];

            for (alias, path) in &aliases.aliases {
                let full_path = if path.starts_with("files/") {
                    sounds_dir().join(path)
                } else {
                    PathBuf::from(path)
                };

                if !full_path.exists() {
                    dangling.push(alias.clone());
                }
            }

            if dangling.is_empty() {
                results.push(CheckResult::ok(
                    "Sound aliases",
                    &format!("{} aliases, all valid", aliases.aliases.len()),
                ));
            } else {
                results.push(CheckResult::warning(
                    "Sound aliases",
                    &format!("dangling aliases: {}", dangling.join(", ")),
                    "Run 'cb sound prune aliases' to fix",
                ));
            }
        }
        Err(e) => {
            results.push(CheckResult::error(
                "Sound aliases",
                &format!("load error: {}", e),
                "Check sounds/aliases.json",
            ));
        }
    }

    results
}

/// Check icon aliases
pub fn check_icon_aliases() -> Vec<CheckResult> {
    let mut results = vec![];

    match load_icon_aliases() {
        Ok(aliases) => {
            results.push(CheckResult::ok(
                "Icon aliases",
                &format!("{} aliases configured", aliases.aliases.len()),
            ));
        }
        Err(e) => {
            results.push(CheckResult::error(
                "Icon aliases",
                &format!("load error: {}", e),
                "Check icons/aliases.json",
            ));
        }
    }

    results
}
