//! Template schema definition

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template definition for notification rendering
///
/// Templates support Tera syntax for dynamic field values.
/// Fields like `title`, `subtitle`, `message` can contain
/// template expressions such as `{{ variable | filter }}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Template name (identifier)
    pub name: String,

    /// Human-readable description
    #[serde(default)]
    pub description: String,

    /// Title template (supports Tera syntax)
    pub title: String,

    /// Subtitle template (optional)
    #[serde(default)]
    pub subtitle: Option<String>,

    /// Message template (optional)
    #[serde(default)]
    pub message: Option<String>,

    /// Image path or URL template
    #[serde(default)]
    pub image: Option<String>,

    /// Icon (path or @alias)
    #[serde(default)]
    pub icon: Option<String>,

    /// Sound (system name, path, or @alias)
    #[serde(default)]
    pub sound: Option<String>,

    /// Action buttons
    #[serde(default)]
    pub actions: Option<Vec<String>>,

    /// Reply placeholder
    #[serde(default)]
    pub reply: Option<String>,

    /// URL to open
    #[serde(default)]
    pub url: Option<String>,

    /// Persistent mode
    #[serde(default)]
    pub persistent: Option<bool>,

    /// Default variable values
    #[serde(default)]
    pub defaults: HashMap<String, String>,
}
