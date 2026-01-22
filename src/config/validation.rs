//! Configuration validation

use super::Config;
use crate::error::AppError;

const VALID_LOG_LEVELS: &[&str] = &["error", "warn", "info", "debug", "trace"];

/// Validate a configuration
///
/// Checks that all configuration values are valid:
/// - Version is supported (currently only version 1)
/// - Log level is one of: error, warn, info, debug, trace
/// - Timeout format is valid (e.g., "30s", "5m", "1h")
///
/// # Errors
///
/// Returns `AppError::ConfigError` if any validation fails.
pub fn validate_config(config: &Config) -> Result<(), AppError> {
    // Validate version
    if config.version != 1 {
        return Err(AppError::ConfigError(format!(
            "Unsupported config version: {}",
            config.version
        )));
    }

    // Validate log level
    if !VALID_LOG_LEVELS.contains(&config.defaults.log_level.as_str()) {
        return Err(AppError::ConfigError(format!(
            "Invalid log level: '{}'. Valid values: {}",
            config.defaults.log_level,
            VALID_LOG_LEVELS.join(", ")
        )));
    }

    // Validate timeout format if present
    if let Some(ref timeout) = config.timeout {
        validate_duration(timeout)?;
    }

    Ok(())
}

/// Validate a duration string (e.g., "30s", "5m", "1h")
///
/// # Supported formats
///
/// - `s` - seconds (e.g., "30s")
/// - `m` - minutes (e.g., "5m")
/// - `h` - hours (e.g., "1h")
fn validate_duration(duration: &str) -> Result<(), AppError> {
    let duration = duration.trim();
    if duration.is_empty() {
        return Err(AppError::ConfigError("Empty duration".to_string()));
    }

    // Must have at least 2 characters (number + unit)
    if duration.len() < 2 {
        return Err(AppError::ConfigError(format!(
            "Invalid duration format: {}",
            duration
        )));
    }

    let (num_str, unit) = duration.split_at(duration.len() - 1);
    let _num: u64 = num_str.parse().map_err(|_| {
        AppError::ConfigError(format!("Invalid duration format: {}", duration))
    })?;

    match unit {
        "s" | "m" | "h" => Ok(()),
        _ => Err(AppError::ConfigError(format!(
            "Invalid duration unit: '{}'. Use s (seconds), m (minutes), or h (hours)",
            unit
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_duration_seconds() {
        assert!(validate_duration("30s").is_ok());
        assert!(validate_duration("1s").is_ok());
        assert!(validate_duration("999s").is_ok());
    }

    #[test]
    fn test_validate_duration_minutes() {
        assert!(validate_duration("5m").is_ok());
        assert!(validate_duration("60m").is_ok());
    }

    #[test]
    fn test_validate_duration_hours() {
        assert!(validate_duration("1h").is_ok());
        assert!(validate_duration("24h").is_ok());
    }

    #[test]
    fn test_validate_duration_invalid_unit() {
        assert!(validate_duration("30x").is_err());
        assert!(validate_duration("10d").is_err());
    }

    #[test]
    fn test_validate_duration_invalid_number() {
        assert!(validate_duration("abcs").is_err());
        assert!(validate_duration("s").is_err());
    }

    #[test]
    fn test_validate_duration_empty() {
        assert!(validate_duration("").is_err());
        assert!(validate_duration("  ").is_err());
    }
}
