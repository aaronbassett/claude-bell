//! Notification response handling for interactive notifications
//!
//! This module handles responses from interactive notifications, including:
//! - Action button clicks
//! - Reply text input
//! - Dismissal events
//! - Timeouts

use crate::error::ExitCode;
use std::collections::HashMap;

/// Notification response types
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationResponse {
    /// User clicked an action button
    Action { identifier: String, index: usize },
    /// User submitted reply text
    Reply { text: String },
    /// User dismissed without selecting action
    Dismissed,
    /// Timeout elapsed without response
    Timeout,
}

impl NotificationResponse {
    /// Get the exit code for this response
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Action { .. } | Self::Reply { .. } => ExitCode::Success,
            Self::Dismissed => ExitCode::Dismissed,
            Self::Timeout => ExitCode::Timeout,
        }
    }

    /// Get the output value for this response
    ///
    /// This determines what gets printed to stdout based on the response type
    /// and any default values specified.
    pub fn output_value(
        &self,
        default: &Option<String>,
        on_dismiss: &Option<String>,
        on_timeout: &Option<String>,
    ) -> String {
        match self {
            Self::Action {
                identifier,
                index: _,
            } => {
                // For actions, return the identifier or index
                identifier.clone()
            }
            Self::Reply { text } => text.clone(),
            Self::Dismissed => on_dismiss
                .clone()
                .or_else(|| default.clone())
                .unwrap_or_default(),
            Self::Timeout => on_timeout
                .clone()
                .or_else(|| default.clone())
                .unwrap_or_default(),
        }
    }

    /// Convert to JSON output
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Action { identifier, index } => {
                serde_json::json!({
                    "type": "action",
                    "identifier": identifier,
                    "index": index
                })
            }
            Self::Reply { text } => {
                serde_json::json!({
                    "type": "reply",
                    "text": text
                })
            }
            Self::Dismissed => {
                serde_json::json!({
                    "type": "dismissed"
                })
            }
            Self::Timeout => {
                serde_json::json!({
                    "type": "timeout"
                })
            }
        }
    }
}

/// Response handler for notifications
///
/// This will be used to implement the delegate pattern for UserNotifications
/// in a future phase. For now, it provides the structure for response handling.
pub struct ResponseHandler {
    /// Default value to return
    pub default: Option<String>,
    /// Value to return on dismiss
    pub on_dismiss: Option<String>,
    /// Value to return on timeout
    pub on_timeout: Option<String>,
    /// Action identifiers mapped to their display names
    pub action_map: HashMap<String, usize>,
}

impl ResponseHandler {
    /// Create a new response handler
    pub fn new(
        default: Option<String>,
        on_dismiss: Option<String>,
        on_timeout: Option<String>,
        actions: Vec<String>,
    ) -> Self {
        let mut action_map = HashMap::new();
        for (idx, _action) in actions.iter().enumerate() {
            action_map.insert(format!("action_{}", idx), idx);
        }

        Self {
            default,
            on_dismiss,
            on_timeout,
            action_map,
        }
    }

    /// Process a response and return the appropriate output
    pub fn process_response(&self, response: &NotificationResponse) -> (String, ExitCode) {
        let output = response.output_value(&self.default, &self.on_dismiss, &self.on_timeout);
        let exit_code = response.exit_code();
        (output, exit_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_exit_codes() {
        let action = NotificationResponse::Action {
            identifier: "action_0".to_string(),
            index: 0,
        };
        assert_eq!(action.exit_code(), ExitCode::Success);

        let reply = NotificationResponse::Reply {
            text: "Hello".to_string(),
        };
        assert_eq!(reply.exit_code(), ExitCode::Success);

        let dismissed = NotificationResponse::Dismissed;
        assert_eq!(dismissed.exit_code(), ExitCode::Dismissed);

        let timeout = NotificationResponse::Timeout;
        assert_eq!(timeout.exit_code(), ExitCode::Timeout);
    }

    #[test]
    fn test_output_value() {
        let action = NotificationResponse::Action {
            identifier: "action_0".to_string(),
            index: 0,
        };
        assert_eq!(
            action.output_value(&None, &None, &None),
            "action_0".to_string()
        );

        let reply = NotificationResponse::Reply {
            text: "Hello".to_string(),
        };
        assert_eq!(reply.output_value(&None, &None, &None), "Hello".to_string());

        let dismissed = NotificationResponse::Dismissed;
        assert_eq!(
            dismissed.output_value(
                &Some("default".to_string()),
                &Some("dismissed".to_string()),
                &None
            ),
            "dismissed".to_string()
        );
        assert_eq!(
            dismissed.output_value(&Some("default".to_string()), &None, &None),
            "default".to_string()
        );
        assert_eq!(dismissed.output_value(&None, &None, &None), "".to_string());

        let timeout = NotificationResponse::Timeout;
        assert_eq!(
            timeout.output_value(
                &Some("default".to_string()),
                &None,
                &Some("timed_out".to_string())
            ),
            "timed_out".to_string()
        );
        assert_eq!(
            timeout.output_value(&Some("default".to_string()), &None, &None),
            "default".to_string()
        );
    }

    #[test]
    fn test_response_to_json() {
        let action = NotificationResponse::Action {
            identifier: "action_0".to_string(),
            index: 0,
        };
        let json = action.to_json();
        assert_eq!(json["type"], "action");
        assert_eq!(json["identifier"], "action_0");
        assert_eq!(json["index"], 0);

        let reply = NotificationResponse::Reply {
            text: "Hello".to_string(),
        };
        let json = reply.to_json();
        assert_eq!(json["type"], "reply");
        assert_eq!(json["text"], "Hello");

        let dismissed = NotificationResponse::Dismissed;
        let json = dismissed.to_json();
        assert_eq!(json["type"], "dismissed");

        let timeout = NotificationResponse::Timeout;
        let json = timeout.to_json();
        assert_eq!(json["type"], "timeout");
    }

    #[test]
    fn test_response_handler() {
        let handler = ResponseHandler::new(
            Some("default".to_string()),
            Some("dismissed".to_string()),
            Some("timed_out".to_string()),
            vec!["OK".to_string(), "Cancel".to_string()],
        );

        let action = NotificationResponse::Action {
            identifier: "action_0".to_string(),
            index: 0,
        };
        let (output, code) = handler.process_response(&action);
        assert_eq!(output, "action_0");
        assert_eq!(code, ExitCode::Success);

        let dismissed = NotificationResponse::Dismissed;
        let (output, code) = handler.process_response(&dismissed);
        assert_eq!(output, "dismissed");
        assert_eq!(code, ExitCode::Dismissed);

        let timeout = NotificationResponse::Timeout;
        let (output, code) = handler.process_response(&timeout);
        assert_eq!(output, "timed_out");
        assert_eq!(code, ExitCode::Timeout);
    }
}
