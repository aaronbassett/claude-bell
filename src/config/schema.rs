//! Configuration schema definitions

use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Config schema version
    #[serde(default = "default_version")]
    pub version: u32,

    /// Default values for notifications
    #[serde(default)]
    pub defaults: Defaults,

    /// JSON output targets when --json is used without arguments
    #[serde(default)]
    pub json_targets: Vec<String>,

    /// Default timeout (null means no timeout)
    pub timeout: Option<String>,

    /// Default value on dismiss
    pub on_dismiss: Option<String>,

    /// Default value on timeout
    pub on_timeout: Option<String>,
}

fn default_version() -> u32 {
    1
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            defaults: Defaults::default(),
            json_targets: vec!["stdout".to_string()],
            timeout: None,
            on_dismiss: None,
            on_timeout: None,
        }
    }
}

/// Default notification settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Defaults {
    /// Default sound
    pub sound: Option<String>,

    /// Default icon
    pub icon: Option<String>,

    /// Default to JSON output
    #[serde(default)]
    pub json: bool,

    /// Default log level
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Default persistent mode
    #[serde(default)]
    pub persistent: bool,
}

fn default_log_level() -> String {
    "warn".to_string()
}
