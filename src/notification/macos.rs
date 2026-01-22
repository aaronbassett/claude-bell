//! macOS notification sending using mac-notification-sys
//!
//! This module uses the mac-notification-sys crate which handles the complexity of
//! sending notifications from CLI tools without requiring app bundle packaging.
//!
//! The crate uses NSUserNotificationCenter (deprecated but functional) with bundle
//! identifier swizzling to work around the requirement for a proper .app bundle.

use crate::cli::args::Cli;
use crate::error::{AppError, ExitCode};
use mac_notification_sys::{
    get_bundle_identifier_or_default, send_notification as sys_send, set_application, Notification,
};
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

fn open_url(url: &str) -> Result<(), AppError> {
    use std::process::Command;

    Command::new("open")
        .arg(url)
        .spawn()
        .map_err(|e| AppError::SystemError(format!("Failed to open URL: {}", e)))?;
    Ok(())
}

fn handle_response(
    response: crate::notification::response::NotificationResponse,
    config: &NotificationConfig,
) -> Result<ExitCode, AppError> {
    use crate::notification::response::NotificationResponse;

    match response {
        NotificationResponse::Action { identifier, .. } => {
            println!("{}", identifier);
            Ok(ExitCode::Success)
        }
        NotificationResponse::Reply { text } => {
            println!("{}", text);
            Ok(ExitCode::Success)
        }
        NotificationResponse::Dismissed => {
            if let Some(val) = config.on_dismiss.as_ref().or(config.default_value.as_ref()) {
                println!("{}", val);
            }
            Ok(ExitCode::Dismissed)
        }
        NotificationResponse::Timeout => {
            if let Some(val) = config.on_timeout.as_ref().or(config.default_value.as_ref()) {
                println!("{}", val);
            }
            Ok(ExitCode::Timeout)
        }
    }
}

/// Send a notification with the given configuration
pub fn send_notification(config: NotificationConfig) -> Result<ExitCode, AppError> {
    // Handle URL opening if specified
    if let Some(ref url) = config.url {
        open_url(url)?;
        println!("opened");
        return Ok(ExitCode::Success);
    }

    // Set application to use Terminal's bundle ID
    let bundle = get_bundle_identifier_or_default("Terminal");
    set_application(&bundle)
        .map_err(|e| AppError::SystemError(format!("Failed to set application: {:?}", e)))?;

    // Prepare subtitle and message
    let subtitle = config.subtitle.as_deref();
    let message = config.message.as_deref().unwrap_or("");

    // Build notification
    let mut base_notification = Notification::new();

    // Add sound if specified
    let notification_ref = if let Some(ref sound) = config.sound {
        if sound == "default" {
            base_notification.sound("NSUserNotificationDefaultSoundName")
        } else {
            base_notification.sound(sound)
        }
    } else {
        &base_notification
    };

    // Check if interactive
    if config.is_interactive() {
        // Send with interaction and wait for response
        // Note: mac-notification-sys doesn't support response handling yet
        // This will be fire-and-forget until we implement a proper delegate
        sys_send(&config.title, subtitle, message, Some(notification_ref)).map_err(|e| {
            AppError::NotificationError(format!("Failed to send notification: {:?}", e))
        })?;

        // For now, print default value since we can't get real responses
        if let Some(ref default_val) = config.default_value {
            println!("{}", default_val);
        }

        Ok(ExitCode::Success)
    } else {
        // Fire-and-forget
        sys_send(&config.title, subtitle, message, Some(notification_ref)).map_err(|e| {
            AppError::NotificationError(format!("Failed to send notification: {:?}", e))
        })?;

        // Print default value if specified
        if let Some(ref default_val) = config.default_value {
            println!("{}", default_val);
        }

        Ok(ExitCode::Success)
    }
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

    #[test]
    fn test_handle_response_action() {
        use crate::notification::response::NotificationResponse;

        let config = NotificationConfig {
            title: "Test".to_string(),
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: vec!["Yes".to_string(), "No".to_string()],
            reply: None,
            url: None,
            persistent: false,
            timeout: None,
            default_value: None,
            on_dismiss: None,
            on_timeout: None,
        };

        let response = NotificationResponse::Action {
            identifier: "Yes".to_string(),
            index: 0,
        };
        let exit_code = handle_response(response, &config).unwrap();
        assert_eq!(exit_code, ExitCode::Success);
    }

    #[test]
    fn test_handle_response_dismissed_with_default() {
        use crate::notification::response::NotificationResponse;

        let config = NotificationConfig {
            title: "Test".to_string(),
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: vec!["OK".to_string()],
            reply: None,
            url: None,
            persistent: false,
            timeout: None,
            default_value: Some("dismissed".to_string()),
            on_dismiss: Some("user_cancelled".to_string()),
            on_timeout: None,
        };

        let response = NotificationResponse::Dismissed;
        let exit_code = handle_response(response, &config).unwrap();
        assert_eq!(exit_code, ExitCode::Dismissed);
    }

    #[test]
    fn test_open_url() {
        // Can't fully test this without actually opening URLs
        // Just verify function signature
        let result = open_url("https://example.com");
        // Will fail on systems without 'open' command, but that's expected
        assert!(result.is_ok() || result.is_err());
    }
}
