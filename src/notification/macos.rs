//! macOS notification sending using mac-notification-sys
//!
//! This module uses the mac-notification-sys crate which handles the complexity of
//! sending notifications from CLI tools without requiring app bundle packaging.
//!
//! The crate uses NSUserNotificationCenter (deprecated but functional) with bundle
//! identifier swizzling to work around the requirement for a proper .app bundle.

use crate::cli::args::Cli;
use crate::error::{AppError, ExitCode};
use mac_notification_sys::{get_bundle_identifier_or_default, send_notification as sys_send, set_application, Notification};
use std::time::Duration;

/// Notification configuration built from CLI arguments
pub struct NotificationConfig {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub sound: Option<String>,
    pub actions: Vec<String>,
    pub reply: Option<String>,
    pub url: Option<String>,
    pub persistent: bool,
    pub timeout: Option<Duration>,
    pub default_value: Option<String>,
    pub on_dismiss: Option<String>,
    pub on_timeout: Option<String>,
}

impl NotificationConfig {
    /// Build configuration from CLI arguments
    pub fn from_cli(cli: &Cli) -> Result<Self, AppError> {
        // Title is required for notifications
        let title = cli
            .title
            .clone()
            .ok_or_else(|| AppError::InvalidArgument("Title is required".to_string()))?;

        // Parse timeout if provided
        let timeout = if let Some(ref timeout_str) = cli.timeout {
            Some(parse_duration(timeout_str)?)
        } else {
            None
        };

        // Determine persistence
        let mut persistent = cli.persistent;
        if cli.not_persistent {
            persistent = false;
        } else if cli.actions.is_some() || cli.reply.is_some() {
            // Implicit persistence for interactive notifications
            persistent = true;
        }

        Ok(Self {
            title,
            subtitle: cli.subtitle.clone(),
            message: cli.message.clone(),
            image: cli.image.clone(),
            icon: cli.icon.clone(),
            sound: cli.sound.clone(),
            actions: cli.actions.clone().unwrap_or_default(),
            reply: cli.reply.clone(),
            url: cli.url.clone(),
            persistent,
            timeout,
            default_value: cli.default.clone(),
            on_dismiss: cli.on_dismiss.clone(),
            on_timeout: cli.on_timeout.clone(),
        })
    }

    /// Check if this is an interactive notification
    pub fn is_interactive(&self) -> bool {
        !self.actions.is_empty() || self.reply.is_some()
    }
}

/// Parse duration string (e.g., "30s", "5m", "1h")
fn parse_duration(s: &str) -> Result<Duration, AppError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AppError::InvalidArgument("Empty duration".to_string()));
    }

    let (number_part, unit_part) = s.split_at(s.len() - 1);
    let value: u64 = number_part
        .parse()
        .map_err(|_| AppError::InvalidArgument(format!("Invalid duration: {}", s)))?;

    let seconds = match unit_part {
        "s" => value,
        "m" => value * 60,
        "h" => value * 3600,
        _ => {
            return Err(AppError::InvalidArgument(format!(
                "Invalid duration unit: {}. Use s, m, or h",
                unit_part
            )))
        }
    };

    Ok(Duration::from_secs(seconds))
}

/// Send a notification with the given configuration
pub fn send_notification(config: NotificationConfig) -> Result<ExitCode, AppError> {
    // Warn about unimplemented features
    if config.url.is_some() {
        eprintln!("Warning: --url field not yet implemented (planned for Phase 10.2)");
    }
    if config.is_interactive() {
        eprintln!("Warning: Interactive notifications (actions/reply) not yet fully implemented");
        eprintln!("Warning: Notification will be sent as fire-and-forget for now");
    }

    // Set application to use Terminal's bundle ID (widely compatible)
    let bundle = get_bundle_identifier_or_default("Terminal");
    set_application(&bundle)
        .map_err(|e| AppError::SystemError(format!("Failed to set application: {:?}", e)))?;

    // Prepare subtitle and message
    let subtitle = config.subtitle.as_deref();
    let message = config.message.as_deref().unwrap_or("");

    // Build notification options if needed and store them
    let mut base_notification = Notification::new();
    let opts_default = base_notification.sound("NSUserNotificationDefaultSoundName");

    let mut base_notification2 = Notification::new();
    let opts_custom;

    // Send as fire-and-forget for now
    // Full interactive support will be added in follow-up work
    let result = if let Some(ref sound) = config.sound {
        if sound == "default" {
            sys_send(&config.title, subtitle, message, Some(&opts_default))
        } else {
            opts_custom = base_notification2.sound(sound);
            sys_send(&config.title, subtitle, message, Some(&opts_custom))
        }
    } else {
        sys_send(&config.title, subtitle, message, None)
    };

    result.map_err(|e| AppError::NotificationError(format!("Failed to send notification: {:?}", e)))?;

    // For non-interactive notifications, print default value if specified
    if let Some(ref default_val) = config.default_value {
        println!("{}", default_val);
    }

    Ok(ExitCode::Success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));

        assert!(parse_duration("").is_err());
        assert!(parse_duration("30x").is_err());
        assert!(parse_duration("abc").is_err());
    }

    #[test]
    fn test_notification_config_interactive() {
        let mut cli = Cli {
            command: None,
            title: Some("Test".to_string()),
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: Some(vec!["OK".to_string()]),
            reply: None,
            url: None,
            persistent: false,
            not_persistent: false,
            timeout: None,
            default: None,
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(config.is_interactive());
        assert!(config.persistent); // Should be implicit for interactive

        cli.actions = None;
        cli.reply = Some("Type here".to_string());
        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(config.is_interactive());

        cli.reply = None;
        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(!config.is_interactive());
    }

    #[test]
    fn test_notification_config_from_cli() {
        let cli = Cli {
            command: None,
            title: Some("Test Title".to_string()),
            subtitle: Some("Test Subtitle".to_string()),
            message: Some("Test Message".to_string()),
            image: None,
            icon: None,
            sound: Some("default".to_string()),
            actions: None,
            reply: None,
            url: None,
            persistent: true,
            not_persistent: false,
            timeout: Some("30s".to_string()),
            default: Some("default_val".to_string()),
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert_eq!(config.title, "Test Title");
        assert_eq!(config.subtitle, Some("Test Subtitle".to_string()));
        assert_eq!(config.message, Some("Test Message".to_string()));
        assert_eq!(config.sound, Some("default".to_string()));
        assert!(config.persistent);
        assert_eq!(config.timeout, Some(Duration::from_secs(30)));
        assert_eq!(config.default_value, Some("default_val".to_string()));
    }

    #[test]
    fn test_notification_config_requires_title() {
        let cli = Cli {
            command: None,
            title: None,
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: None,
            reply: None,
            url: None,
            persistent: false,
            not_persistent: false,
            timeout: None,
            default: None,
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        assert!(NotificationConfig::from_cli(&cli).is_err());
    }
}
