//! Error types and exit codes

use thiserror::Error;

/// Exit codes for the CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    /// Success - action clicked, reply received, or fire-and-forget sent
    Success = 0,
    /// Timeout elapsed without response
    Timeout = 1,
    /// User dismissed without selecting action
    Dismissed = 2,
    /// User error - invalid args, template not found, malformed JSON
    UserError = 3,
    /// System error - missing permissions, notification service unavailable
    SystemError = 4,
    /// Application error - bug in claude-bell
    AppError = 5,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

/// Application errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Alias not found: {0}")]
    AliasNotFound(String),

    #[error("Notification error: {0}")]
    NotificationError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl AppError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::InvalidArgument(_) => ExitCode::UserError,
            Self::TemplateNotFound(_) => ExitCode::UserError,
            Self::TemplateError(_) => ExitCode::UserError,
            Self::ConfigError(_) => ExitCode::UserError,
            Self::AliasNotFound(_) => ExitCode::UserError,
            Self::NotificationError(_) => ExitCode::SystemError,
            Self::PermissionDenied(_) => ExitCode::SystemError,
            Self::SystemError(_) => ExitCode::SystemError,
            Self::Io(_) => ExitCode::SystemError,
            Self::Json(_) => ExitCode::UserError,
        }
    }
}
