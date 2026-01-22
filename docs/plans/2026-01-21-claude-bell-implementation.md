# Claude Bell Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a macOS notification CLI (`cb`) optimized for Claude Code with templates, aliases, and plugin ecosystem.

**Architecture:** Single Rust binary using clap for CLI, objc2 for macOS UserNotifications, tera for templates. Data stored in `~/.claude-bell/`. Distributed via Homebrew, Cargo, and Claude Code plugins.

**Tech Stack:** Rust, clap (derive), tera, colored_json, objc2, thiserror/anyhow, serde/serde_json

---

## Phase 1: Project Scaffold

### Task 1.1: Initialize Cargo Project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "claude-bell"
version = "0.1.0"
edition = "2021"
description = "macOS notifications for Claude Code"
license = "MIT"
repository = "https://github.com/aaronbassett/claude-bell"
keywords = ["cli", "notifications", "macos", "claude"]
categories = ["command-line-utilities"]

[[bin]]
name = "cb"
path = "src/main.rs"

[[bin]]
name = "claude-bell"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive", "env"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"

[dev-dependencies]
tempfile = "3"
assert_cmd = "2"
predicates = "3"
```

**Step 2: Create src/main.rs**

```rust
use anyhow::Result;

fn main() -> Result<()> {
    claude_bell::run()
}
```

**Step 3: Create src/lib.rs**

```rust
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("Claude Bell v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
```

**Step 4: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 5: Verify both binaries work**

Run: `cargo run --bin cb`
Expected: `Claude Bell v0.1.0`

Run: `cargo run --bin claude-bell`
Expected: `Claude Bell v0.1.0`

**Step 6: Commit**

```bash
git add Cargo.toml src/
git commit -m "feat: initialize cargo project with dual binary names

Sets up claude-bell crate with both 'cb' and 'claude-bell' binaries
pointing to the same entry point.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 1.2: Set Up Basic CLI Structure with Clap

**Files:**
- Create: `src/cli/mod.rs`
- Create: `src/cli/args.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`

**Step 1: Write failing test for --version flag**

Create `tests/integration/cli_test.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("claude-bell"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("macOS notifications"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test cli_test`
Expected: FAIL (no tests/integration directory or test file)

**Step 3: Create directory and test file**

Run: `mkdir -p tests/integration`

Then create the test file with content from Step 1.

**Step 4: Run test again**

Run: `cargo test --test cli_test`
Expected: FAIL (version output doesn't match expected format)

**Step 5: Create src/cli/mod.rs**

```rust
pub mod args;

pub use args::Cli;
```

**Step 6: Create src/cli/args.rs**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "claude-bell",
    about = "macOS notifications for Claude Code",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Notification title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Notification subtitle
    #[arg(short, long)]
    pub subtitle: Option<String>,

    /// Notification message body
    #[arg(short, long)]
    pub message: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage notification templates
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
    /// Manage sound aliases
    Sound {
        #[command(subcommand)]
        action: SoundCommands,
    },
    /// Manage icon aliases
    Icon {
        #[command(subcommand)]
        action: IconCommands,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Run first-time setup wizard
    Setup,
    /// Check system health and configuration
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// List all templates
    List,
    /// Show a template
    Show { name: String },
    /// Create a new template
    Create,
    /// Update an existing template
    Update { name: String },
    /// Delete a template
    Delete { name: String },
    /// Validate templates
    Validate {
        /// Template name (or --all)
        name: Option<String>,
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum SoundCommands {
    /// List sound aliases
    List,
    /// Add a sound alias
    Add {
        alias: String,
        path: String,
        #[arg(long)]
        cache: bool,
    },
    /// Remove a sound alias
    Remove {
        alias: String,
        #[arg(long)]
        with_file: bool,
    },
    /// Prune orphaned sounds
    Prune {
        /// What to prune: files, aliases, or both
        target: Option<String>,
    },
    /// Check sound alias health
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum IconCommands {
    /// List icon aliases
    List,
    /// Add an icon alias
    Add { alias: String, path: String },
    /// Remove an icon alias
    Remove {
        alias: String,
        #[arg(long)]
        with_bundle: bool,
    },
    /// Prune orphaned icons
    Prune {
        /// What to prune: bundles, aliases, or both
        target: Option<String>,
    },
    /// Check icon alias health
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        #[arg(long)]
        pretty: bool,
    },
    /// Set a configuration value
    Set { key: String, value: String },
    /// Unset a configuration value
    Unset { key: String },
    /// Reset configuration to defaults
    Reset,
    /// Validate configuration
    Validate {
        #[arg(long)]
        path: Option<String>,
    },
}
```

**Step 7: Update src/lib.rs**

```rust
pub mod cli;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // For now, just print what we parsed
    println!("{:?}", cli);

    Ok(())
}
```

**Step 8: Update src/main.rs**

```rust
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    claude_bell::run()
}
```

**Step 9: Run tests**

Run: `cargo test --test cli_test`
Expected: PASS

**Step 10: Commit**

```bash
git add src/cli/ tests/
git commit -m "feat: add basic CLI structure with clap

Implements main command structure with subcommands for template,
sound, icon, config, setup, and doctor. Includes integration tests
for --version and --help flags.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 1.3: Add Notification Content Flags

**Files:**
- Modify: `src/cli/args.rs`
- Create: `tests/integration/notification_flags_test.rs`

**Step 1: Write failing test for notification flags**

Create `tests/integration/notification_flags_test.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_title_flag_short() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Title"));
}

#[test]
fn test_title_flag_long() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--title", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Title"));
}

#[test]
fn test_multiple_content_flags() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args([
        "-t", "Title",
        "-s", "Subtitle",
        "-m", "Message",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Title"))
    .stdout(predicate::str::contains("Subtitle"))
    .stdout(predicate::str::contains("Message"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test notification_flags_test`
Expected: PASS (we already have -t, -s, -m in args.rs)

**Step 3: Add remaining notification flags to args.rs**

Update `src/cli/args.rs` - add these fields to the `Cli` struct:

```rust
#[derive(Parser, Debug)]
#[command(
    name = "claude-bell",
    about = "macOS notifications for Claude Code",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    // Content flags
    /// Notification title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Notification subtitle
    #[arg(short, long)]
    pub subtitle: Option<String>,

    /// Notification message body
    #[arg(short, long)]
    pub message: Option<String>,

    /// Thumbnail image path or URL
    #[arg(short, long)]
    pub image: Option<String>,

    /// App icon (path or @alias)
    #[arg(long)]
    pub icon: Option<String>,

    /// Sound (system name, path, or @alias)
    #[arg(long)]
    pub sound: Option<String>,

    // Interaction flags
    /// Comma-separated action buttons
    #[arg(short, long, value_delimiter = ',')]
    pub actions: Option<Vec<String>>,

    /// Enable reply input with placeholder
    #[arg(short, long)]
    pub reply: Option<String>,

    /// URL to open when notification clicked
    #[arg(long)]
    pub url: Option<String>,

    // Behavior flags
    /// Keep notification on screen until dismissed
    #[arg(long)]
    pub persistent: bool,

    /// Override implicit persistence
    #[arg(long)]
    pub not_persistent: bool,

    /// Timeout duration (e.g., 30s, 5m)
    #[arg(long)]
    pub timeout: Option<String>,

    /// Default value on dismiss/timeout
    #[arg(long)]
    pub default: Option<String>,

    /// Value to return on dismiss
    #[arg(long)]
    pub on_dismiss: Option<String>,

    /// Value to return on timeout
    #[arg(long)]
    pub on_timeout: Option<String>,

    // Input flags
    /// Process newline-delimited JSON from stdin
    #[arg(long)]
    pub batch: bool,

    // Output flags
    /// JSON output targets (stdout, stderr, logs, response)
    #[arg(long, value_delimiter = ',')]
    pub json: Option<Vec<String>>,

    /// Pretty-print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Suppress stdout
    #[arg(long)]
    pub quiet: bool,

    /// Suppress all output
    #[arg(long)]
    pub silent: bool,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "warn")]
    pub log_level: String,

    // Template flags
    /// Use named template
    #[arg(long)]
    pub template: Option<String>,

    /// Template variables (key:value, repeatable)
    #[arg(long, value_delimiter = ',')]
    pub var: Option<Vec<String>>,
}
```

**Step 4: Run all tests**

Run: `cargo test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/cli/args.rs tests/integration/
git commit -m "feat: add all notification CLI flags

Adds content flags (title, subtitle, message, image, icon, sound),
interaction flags (actions, reply, url), behavior flags (persistent,
timeout, default, on-dismiss, on-timeout), input flags (batch),
output flags (json, pretty, quiet, silent, log-level), and template
flags (template, var).

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 1.4: Create Directory Structure

**Files:**
- Create: `src/notification/mod.rs`
- Create: `src/template/mod.rs`
- Create: `src/config/mod.rs`
- Create: `src/alias/mod.rs`
- Create: `src/doctor/mod.rs`
- Create: `src/error.rs`
- Modify: `src/lib.rs`

**Step 1: Create module files**

Create `src/notification/mod.rs`:
```rust
//! macOS notification sending and response handling
```

Create `src/template/mod.rs`:
```rust
//! Template management and Tera rendering
```

Create `src/config/mod.rs`:
```rust
//! Configuration loading, validation, and management
```

Create `src/alias/mod.rs`:
```rust
//! Sound and icon alias management

pub mod sound;
pub mod icon;
```

Create `src/alias/sound.rs`:
```rust
//! Sound alias management
```

Create `src/alias/icon.rs`:
```rust
//! Icon alias management
```

Create `src/doctor/mod.rs`:
```rust
//! System health checks and diagnostics
```

Create `src/error.rs`:
```rust
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
            Self::Io(_) => ExitCode::SystemError,
            Self::Json(_) => ExitCode::UserError,
        }
    }
}
```

**Step 2: Update src/lib.rs**

```rust
pub mod cli;
pub mod config;
pub mod error;
pub mod notification;
pub mod template;
pub mod alias;
pub mod doctor;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;
pub use error::{AppError, ExitCode};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // For now, just print what we parsed
    println!("{:?}", cli);

    Ok(())
}
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/
git commit -m "feat: create module directory structure

Sets up module structure for notification, template, config, alias
(sound/icon), doctor, and error handling. Defines ExitCode enum
and AppError types per design specification.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 1.5: Add Test Infrastructure

**Files:**
- Create: `tests/common/mod.rs`
- Create: `tests/fixtures/.gitkeep`

**Step 1: Create test utilities**

Create `tests/common/mod.rs`:

```rust
use std::path::PathBuf;
use tempfile::TempDir;

/// Create a temporary directory structure mimicking ~/.claude-bell/
pub fn setup_test_dir() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_path_buf();

    // Create subdirectories
    std::fs::create_dir_all(base_path.join("templates")).unwrap();
    std::fs::create_dir_all(base_path.join("sounds/files")).unwrap();
    std::fs::create_dir_all(base_path.join("icons/bundles")).unwrap();
    std::fs::create_dir_all(base_path.join("apps")).unwrap();

    (temp_dir, base_path)
}

/// Create a test config file
pub fn create_test_config(base_path: &PathBuf, content: &str) {
    std::fs::write(base_path.join("config.json"), content).unwrap();
}

/// Create a test template file
pub fn create_test_template(base_path: &PathBuf, name: &str, content: &str) {
    let template_path = base_path.join("templates").join(format!("{}.json", name));
    std::fs::write(template_path, content).unwrap();
}
```

**Step 2: Create fixtures directory**

Run: `mkdir -p tests/fixtures && touch tests/fixtures/.gitkeep`

**Step 3: Verify tests still pass**

Run: `cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add tests/
git commit -m "feat: add test infrastructure

Creates test utilities for setting up temporary directories and
fixtures that mimic the ~/.claude-bell/ structure.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 2: Exit Codes and Error Handling

### Task 2.1: Implement Exit Code Handling in Main

**Files:**
- Modify: `src/main.rs`
- Modify: `src/lib.rs`
- Create: `tests/integration/exit_codes_test.rs`

**Step 1: Write failing test for exit codes**

Create `tests/integration/exit_codes_test.rs`:

```rust
use assert_cmd::Command;

#[test]
fn test_success_exit_code() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--version")
        .assert()
        .code(0);
}

#[test]
fn test_user_error_exit_code() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    // Invalid subcommand should return exit code 3
    cmd.args(["invalid-subcommand"])
        .assert()
        .code(2); // clap uses 2 for invalid args by default
}
```

**Step 2: Run test**

Run: `cargo test --test exit_codes_test`
Expected: PASS (clap handles this)

**Step 3: Update main.rs to handle our custom exit codes**

```rust
use std::process::ExitCode as StdExitCode;

fn main() -> StdExitCode {
    match claude_bell::run() {
        Ok(code) => StdExitCode::from(code as u8),
        Err(e) => {
            eprintln!("Error: {}", e);
            StdExitCode::from(claude_bell::ExitCode::AppError as u8)
        }
    }
}
```

**Step 4: Update lib.rs to return ExitCode**

```rust
pub mod cli;
pub mod config;
pub mod error;
pub mod notification;
pub mod template;
pub mod alias;
pub mod doctor;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;
pub use error::{AppError, ExitCode};

pub fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    // For now, just print what we parsed
    if cli.command.is_none() && cli.title.is_none() {
        // No command and no title - just show help behavior
        println!("{:?}", cli);
    }

    Ok(ExitCode::Success)
}
```

**Step 5: Run tests**

Run: `cargo test`
Expected: PASS

**Step 6: Commit**

```bash
git add src/ tests/
git commit -m "feat: implement exit code handling

Main now returns proper exit codes. ExitCode::Success (0) for normal
operation, with error handling returning AppError (5) on unexpected
failures.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 3: Configuration System

### Task 3.1: Define Config Schema

**Files:**
- Create: `src/config/schema.rs`
- Modify: `src/config/mod.rs`

**Step 1: Write failing test for config parsing**

Create `tests/integration/config_test.rs`:

```rust
use std::path::PathBuf;

mod common;

#[test]
fn test_parse_valid_config() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "sound": "Glass",
            "icon": "@claude",
            "json": false,
            "log_level": "warn",
            "persistent": false
        }
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert_eq!(config.version, 1);
    assert_eq!(config.defaults.sound, Some("Glass".to_string()));
}

#[test]
fn test_config_defaults() {
    let config = claude_bell::config::Config::default();
    assert_eq!(config.version, 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test config_test`
Expected: FAIL (Config type doesn't exist)

**Step 3: Create src/config/schema.rs**

```rust
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
```

**Step 4: Update src/config/mod.rs**

```rust
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
```

**Step 5: Add dirs dependency to Cargo.toml**

Add to `[dependencies]`:
```toml
dirs = "5"
```

**Step 6: Run tests**

Run: `cargo test --test config_test`
Expected: PASS

**Step 7: Commit**

```bash
git add Cargo.toml src/config/ tests/
git commit -m "feat: implement config schema and loading

Adds Config and Defaults structs with serde serialization. Implements
load_config and save_config functions with default value handling.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 3.2: Implement Config Validation

**Files:**
- Create: `src/config/validation.rs`
- Modify: `src/config/mod.rs`

**Step 1: Write failing test for validation**

Add to `tests/integration/config_test.rs`:

```rust
#[test]
fn test_validate_valid_config() {
    let config = claude_bell::config::Config::default();
    assert!(claude_bell::config::validate_config(&config).is_ok());
}

#[test]
fn test_validate_invalid_log_level() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "log_level": "invalid"
        }
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test config_test`
Expected: FAIL (validate_config doesn't exist)

**Step 3: Create src/config/validation.rs**

```rust
use super::Config;
use crate::error::AppError;

const VALID_LOG_LEVELS: &[&str] = &["error", "warn", "info", "debug", "trace"];

/// Validate a configuration
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

/// Validate a duration string (e.g., "30s", "5m")
fn validate_duration(duration: &str) -> Result<(), AppError> {
    let duration = duration.trim();
    if duration.is_empty() {
        return Err(AppError::ConfigError("Empty duration".to_string()));
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
```

**Step 4: Update src/config/mod.rs**

Add after the existing code:

```rust
mod validation;

pub use validation::validate_config;
```

**Step 5: Run tests**

Run: `cargo test --test config_test`
Expected: PASS

**Step 6: Commit**

```bash
git add src/config/ tests/
git commit -m "feat: implement config validation

Validates config version, log level values, and duration format.
Returns detailed error messages for invalid configurations.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 4: Template System

### Task 4.1: Define Template Schema

**Files:**
- Create: `src/template/schema.rs`
- Modify: `src/template/mod.rs`
- Modify: `Cargo.toml`

**Step 1: Write failing test for template parsing**

Create `tests/integration/template_test.rs`:

```rust
#[test]
fn test_parse_valid_template() {
    let template_json = r#"{
        "name": "build-result",
        "description": "Notify when build finishes",
        "title": "Build {{ status | title }}",
        "message": "Completed",
        "sound": "@success",
        "defaults": {
            "status": "complete"
        }
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    assert_eq!(template.name, "build-result");
    assert_eq!(template.title, "Build {{ status | title }}");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test template_test`
Expected: FAIL (Template type doesn't exist)

**Step 3: Add tera dependency**

Add to Cargo.toml `[dependencies]`:
```toml
tera = "1"
```

**Step 4: Create src/template/schema.rs**

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template definition
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
```

**Step 5: Update src/template/mod.rs**

```rust
//! Template management and Tera rendering

mod schema;

pub use schema::Template;

use crate::error::AppError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

/// Get the templates directory path
pub fn templates_dir() -> PathBuf {
    crate::config::config_dir().join("templates")
}

/// Load a template by name
pub fn load_template(name: &str) -> Result<Template, AppError> {
    let template_path = templates_dir().join(format!("{}.json", name));

    if !template_path.exists() {
        return Err(AppError::TemplateNotFound(name.to_string()));
    }

    let content = std::fs::read_to_string(&template_path)?;
    let template: Template = serde_json::from_str(&content)?;
    Ok(template)
}

/// List all templates
pub fn list_templates() -> Result<Vec<String>, AppError> {
    let templates_path = templates_dir();

    if !templates_path.exists() {
        return Ok(vec![]);
    }

    let mut templates = vec![];
    for entry in std::fs::read_dir(&templates_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                templates.push(name.to_string());
            }
        }
    }

    templates.sort();
    Ok(templates)
}

/// Save a template
pub fn save_template(template: &Template) -> Result<(), AppError> {
    let templates_path = templates_dir();
    std::fs::create_dir_all(&templates_path)?;

    let template_path = templates_path.join(format!("{}.json", template.name));
    let content = serde_json::to_string_pretty(template)?;
    std::fs::write(&template_path, content)?;
    Ok(())
}

/// Delete a template
pub fn delete_template(name: &str) -> Result<(), AppError> {
    let template_path = templates_dir().join(format!("{}.json", name));

    if !template_path.exists() {
        return Err(AppError::TemplateNotFound(name.to_string()));
    }

    std::fs::remove_file(&template_path)?;
    Ok(())
}
```

**Step 6: Run tests**

Run: `cargo test --test template_test`
Expected: PASS

**Step 7: Commit**

```bash
git add Cargo.toml src/template/ tests/
git commit -m "feat: implement template schema and management

Adds Template struct with Tera template fields. Implements CRUD
operations: load_template, list_templates, save_template, delete_template.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 4.2: Implement Template Rendering

**Files:**
- Create: `src/template/render.rs`
- Modify: `src/template/mod.rs`

**Step 1: Write failing test for template rendering**

Add to `tests/integration/template_test.rs`:

```rust
#[test]
fn test_render_template_simple() {
    let template_json = r#"{
        "name": "test",
        "title": "Hello {{ name }}",
        "defaults": {}
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let mut vars = std::collections::HashMap::new();
    vars.insert("name".to_string(), "World".to_string());

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "Hello World");
}

#[test]
fn test_render_template_with_defaults() {
    let template_json = r#"{
        "name": "test",
        "title": "Status: {{ status }}",
        "defaults": {
            "status": "pending"
        }
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let vars = std::collections::HashMap::new();

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "Status: pending");
}

#[test]
fn test_render_template_with_filter() {
    let template_json = r#"{
        "name": "test",
        "title": "{{ name | upper }}",
        "defaults": {}
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let mut vars = std::collections::HashMap::new();
    vars.insert("name".to_string(), "hello".to_string());

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "HELLO");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test template_test`
Expected: FAIL (render_template doesn't exist)

**Step 3: Create src/template/render.rs**

```rust
use super::Template;
use crate::error::AppError;
use std::collections::HashMap;
use tera::{Context, Tera};

/// Rendered template with all fields resolved
#[derive(Debug, Clone)]
pub struct RenderedTemplate {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub sound: Option<String>,
    pub actions: Option<Vec<String>>,
    pub reply: Option<String>,
    pub url: Option<String>,
    pub persistent: Option<bool>,
}

/// Render a template with provided variables
pub fn render_template(
    template: &Template,
    vars: &HashMap<String, String>,
) -> Result<RenderedTemplate, AppError> {
    let mut tera = Tera::default();

    // Build context with defaults first, then override with provided vars
    let mut context = Context::new();
    for (key, value) in &template.defaults {
        context.insert(key, value);
    }
    for (key, value) in vars {
        context.insert(key, value);
    }

    let render_field = |field: &str| -> Result<String, AppError> {
        tera.render_str(field, &context)
            .map_err(|e| AppError::TemplateError(e.to_string()))
    };

    let render_optional = |field: &Option<String>| -> Result<Option<String>, AppError> {
        match field {
            Some(f) => Ok(Some(render_field(f)?)),
            None => Ok(None),
        }
    };

    Ok(RenderedTemplate {
        title: render_field(&template.title)?,
        subtitle: render_optional(&template.subtitle)?,
        message: render_optional(&template.message)?,
        image: render_optional(&template.image)?,
        icon: template.icon.clone(), // Icons/sounds aren't templates
        sound: template.sound.clone(),
        actions: template.actions.clone(),
        reply: template.reply.clone(),
        url: render_optional(&template.url)?,
        persistent: template.persistent,
    })
}
```

**Step 4: Update src/template/mod.rs**

Add after existing code:

```rust
mod render;

pub use render::{render_template, RenderedTemplate};
```

**Step 5: Run tests**

Run: `cargo test --test template_test`
Expected: PASS

**Step 6: Commit**

```bash
git add src/template/ tests/
git commit -m "feat: implement template rendering with Tera

Renders template fields using Tera engine with variable substitution.
Supports default values that can be overridden by provided vars.
Filters like upper, lower, title work out of the box.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 5: Sound and Icon Alias Systems

### Task 5.1: Implement Sound Alias Management

**Files:**
- Create: `src/alias/sound.rs` (replace placeholder)
- Modify: `src/alias/mod.rs`

**Step 1: Write failing test for sound aliases**

Create `tests/integration/sound_alias_test.rs`:

```rust
use std::collections::HashMap;

#[test]
fn test_parse_sound_aliases() {
    let aliases_json = r#"{
        "bark": "/path/to/bark.aiff",
        "ding": "files/abc123.aiff"
    }"#;

    let aliases: HashMap<String, String> = serde_json::from_str(aliases_json).unwrap();
    assert_eq!(aliases.get("bark"), Some(&"/path/to/bark.aiff".to_string()));
}
```

**Step 2: Run test**

Run: `cargo test --test sound_alias_test`
Expected: PASS (just testing HashMap parsing)

**Step 3: Create src/alias/sound.rs**

```rust
//! Sound alias management

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Sound alias storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SoundAliases {
    /// Map of alias name to path (either absolute or relative to files/)
    #[serde(flatten)]
    pub aliases: HashMap<String, String>,
}

/// Get the sounds directory path
pub fn sounds_dir() -> PathBuf {
    crate::config::config_dir().join("sounds")
}

/// Get the cached files directory path
pub fn sound_files_dir() -> PathBuf {
    sounds_dir().join("files")
}

/// Load sound aliases
pub fn load_sound_aliases() -> Result<SoundAliases, AppError> {
    let aliases_path = sounds_dir().join("aliases.json");

    if !aliases_path.exists() {
        return Ok(SoundAliases::default());
    }

    let content = std::fs::read_to_string(&aliases_path)?;
    let aliases: SoundAliases = serde_json::from_str(&content)?;
    Ok(aliases)
}

/// Save sound aliases
pub fn save_sound_aliases(aliases: &SoundAliases) -> Result<(), AppError> {
    let sounds_path = sounds_dir();
    std::fs::create_dir_all(&sounds_path)?;

    let aliases_path = sounds_path.join("aliases.json");
    let content = serde_json::to_string_pretty(&aliases.aliases)?;
    std::fs::write(&aliases_path, content)?;
    Ok(())
}

/// Add a sound alias
pub fn add_sound_alias(alias: &str, path: &str, cache: bool) -> Result<(), AppError> {
    let mut aliases = load_sound_aliases()?;

    let stored_path = if cache {
        // Copy file to cache with content hash
        let source_path = Path::new(path);
        if !source_path.exists() {
            return Err(AppError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Sound file not found: {}", path),
            )));
        }

        let content = std::fs::read(source_path)?;
        let hash = format!("{:x}", Sha256::digest(&content));
        let hash_short = &hash[..16];

        let extension = source_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("aiff");

        let cached_filename = format!("{}.{}", hash_short, extension);
        let cached_path = sound_files_dir().join(&cached_filename);

        // Create directory and copy file if not already cached
        std::fs::create_dir_all(sound_files_dir())?;
        if !cached_path.exists() {
            std::fs::copy(source_path, &cached_path)?;
        }

        format!("files/{}", cached_filename)
    } else {
        path.to_string()
    };

    aliases.aliases.insert(alias.to_string(), stored_path);
    save_sound_aliases(&aliases)?;
    Ok(())
}

/// Remove a sound alias
pub fn remove_sound_alias(alias: &str, with_file: bool) -> Result<Vec<String>, AppError> {
    let mut aliases = load_sound_aliases()?;
    let mut warnings = vec![];

    let path = aliases.aliases.remove(alias)
        .ok_or_else(|| AppError::AliasNotFound(format!("sound alias: {}", alias)))?;

    if with_file && path.starts_with("files/") {
        // Check if other aliases use this file
        let other_aliases: Vec<_> = aliases.aliases
            .iter()
            .filter(|(_, p)| *p == &path)
            .map(|(a, _)| a.clone())
            .collect();

        if !other_aliases.is_empty() {
            warnings.push(format!(
                "File '{}' is also used by aliases: {}",
                path,
                other_aliases.join(", ")
            ));
        }

        // Delete the file
        let file_path = sounds_dir().join(&path);
        if file_path.exists() {
            std::fs::remove_file(&file_path)?;
        }
    }

    save_sound_aliases(&aliases)?;
    Ok(warnings)
}

/// Resolve a sound reference (system name, path, or @alias)
pub fn resolve_sound(sound: &str) -> Result<String, AppError> {
    if sound.starts_with('@') {
        let alias = &sound[1..];
        let aliases = load_sound_aliases()?;

        let path = aliases.aliases.get(alias)
            .ok_or_else(|| AppError::AliasNotFound(format!("sound alias: {}", alias)))?;

        if path.starts_with("files/") {
            Ok(sounds_dir().join(path).to_string_lossy().to_string())
        } else {
            Ok(path.clone())
        }
    } else {
        // Return as-is (system sound name or direct path)
        Ok(sound.to_string())
    }
}

/// List all sound aliases with their paths
pub fn list_sound_aliases() -> Result<HashMap<String, String>, AppError> {
    let aliases = load_sound_aliases()?;
    Ok(aliases.aliases)
}
```

**Step 4: Add sha2 dependency**

Add to Cargo.toml `[dependencies]`:
```toml
sha2 = "0.10"
```

**Step 5: Update src/alias/mod.rs**

```rust
//! Sound and icon alias management

pub mod sound;
pub mod icon;

pub use sound::{
    add_sound_alias, list_sound_aliases, load_sound_aliases, remove_sound_alias,
    resolve_sound, save_sound_aliases, sound_files_dir, sounds_dir, SoundAliases,
};
```

**Step 6: Run tests**

Run: `cargo test`
Expected: PASS

**Step 7: Commit**

```bash
git add Cargo.toml src/alias/ tests/
git commit -m "feat: implement sound alias management

Adds CRUD operations for sound aliases with content-hash deduplication
for cached files. Supports @alias resolution and warns when removing
files used by multiple aliases.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 5.2: Implement Icon Alias Management

**Files:**
- Create: `src/alias/icon.rs` (replace placeholder)
- Modify: `src/alias/mod.rs`

**Step 1: Create src/alias/icon.rs**

```rust
//! Icon alias management

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Icon alias storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IconAliases {
    /// Map of alias name to bundle name in bundles/
    #[serde(flatten)]
    pub aliases: HashMap<String, String>,
}

/// Bundled icon names (shipped with the binary)
pub const BUNDLED_ICONS: &[&str] = &["claude", "terminal", "success", "warning", "error"];

/// Get the icons directory path
pub fn icons_dir() -> PathBuf {
    crate::config::config_dir().join("icons")
}

/// Get the icon bundles directory path
pub fn icon_bundles_dir() -> PathBuf {
    icons_dir().join("bundles")
}

/// Load icon aliases
pub fn load_icon_aliases() -> Result<IconAliases, AppError> {
    let aliases_path = icons_dir().join("aliases.json");

    if !aliases_path.exists() {
        return Ok(IconAliases::default());
    }

    let content = std::fs::read_to_string(&aliases_path)?;
    let aliases: IconAliases = serde_json::from_str(&content)?;
    Ok(aliases)
}

/// Save icon aliases
pub fn save_icon_aliases(aliases: &IconAliases) -> Result<(), AppError> {
    let icons_path = icons_dir();
    std::fs::create_dir_all(&icons_path)?;

    let aliases_path = icons_path.join("aliases.json");
    let content = serde_json::to_string_pretty(&aliases.aliases)?;
    std::fs::write(&aliases_path, content)?;
    Ok(())
}

/// Add an icon alias (from app or image path)
pub fn add_icon_alias(alias: &str, path: &str) -> Result<(), AppError> {
    let mut aliases = load_icon_aliases()?;
    let source_path = std::path::Path::new(path);

    if !source_path.exists() {
        return Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Icon source not found: {}", path),
        )));
    }

    // For now, just store the path - bundle generation will be implemented
    // when we add macOS notification support
    let bundle_name = format!("{}.app", alias);

    // TODO: Generate app bundle from source (app or image)
    // For now, store a reference
    aliases.aliases.insert(alias.to_string(), bundle_name);
    save_icon_aliases(&aliases)?;
    Ok(())
}

/// Remove an icon alias
pub fn remove_icon_alias(alias: &str, with_bundle: bool) -> Result<Vec<String>, AppError> {
    let mut aliases = load_icon_aliases()?;
    let mut warnings = vec![];

    let bundle_name = aliases.aliases.remove(alias)
        .ok_or_else(|| AppError::AliasNotFound(format!("icon alias: {}", alias)))?;

    if with_bundle {
        // Check if other aliases use this bundle
        let other_aliases: Vec<_> = aliases.aliases
            .iter()
            .filter(|(_, b)| *b == &bundle_name)
            .map(|(a, _)| a.clone())
            .collect();

        if !other_aliases.is_empty() {
            warnings.push(format!(
                "Bundle '{}' is also used by aliases: {}",
                bundle_name,
                other_aliases.join(", ")
            ));
        }

        // Delete the bundle
        let bundle_path = icon_bundles_dir().join(&bundle_name);
        if bundle_path.exists() {
            std::fs::remove_dir_all(&bundle_path)?;
        }
    }

    save_icon_aliases(&aliases)?;
    Ok(warnings)
}

/// Resolve an icon reference (bundled name, path, or @alias)
pub fn resolve_icon(icon: &str) -> Result<Option<String>, AppError> {
    if icon.starts_with('@') {
        let alias = &icon[1..];

        // Check if it's a bundled icon
        if BUNDLED_ICONS.contains(&alias) {
            // TODO: Return path to bundled icon
            return Ok(Some(format!("@{}", alias)));
        }

        // Check user aliases
        let aliases = load_icon_aliases()?;

        let bundle_name = aliases.aliases.get(alias)
            .ok_or_else(|| AppError::AliasNotFound(format!("icon alias: {}", alias)))?;

        Ok(Some(icon_bundles_dir().join(bundle_name).to_string_lossy().to_string()))
    } else {
        // Direct path
        Ok(Some(icon.to_string()))
    }
}

/// List all icon aliases with their bundle names
pub fn list_icon_aliases() -> Result<HashMap<String, String>, AppError> {
    let aliases = load_icon_aliases()?;
    Ok(aliases.aliases)
}
```

**Step 2: Update src/alias/mod.rs**

Add to exports:

```rust
pub use icon::{
    add_icon_alias, icon_bundles_dir, icons_dir, list_icon_aliases, load_icon_aliases,
    remove_icon_alias, resolve_icon, save_icon_aliases, IconAliases, BUNDLED_ICONS,
};
```

**Step 3: Run tests**

Run: `cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add src/alias/
git commit -m "feat: implement icon alias management

Adds CRUD operations for icon aliases. Includes placeholder for
bundle generation (to be implemented with macOS notification support).
Supports @alias resolution with bundled icon fallback.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 6: Doctor/Health Checks

### Task 6.1: Implement Doctor Command

**Files:**
- Create: `src/doctor/checks.rs`
- Modify: `src/doctor/mod.rs`

**Step 1: Create src/doctor/checks.rs**

```rust
//! Health check implementations

use crate::config;
use crate::alias::{sounds_dir, icons_dir, load_sound_aliases, load_icon_aliases};
use crate::error::AppError;
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
        CheckResult::ok("Config directory", &format!("exists at {}", config_dir.display()))
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
```

**Step 2: Update src/doctor/mod.rs**

```rust
//! System health checks and diagnostics

mod checks;

pub use checks::{
    check_config_directory, check_config_file, check_icon_aliases, check_sound_aliases,
    run_all_checks, CheckResult, CheckStatus,
};
```

**Step 3: Run tests**

Run: `cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add src/doctor/
git commit -m "feat: implement doctor health checks

Adds comprehensive health checks for config directory, config file
validity, sound alias integrity, and icon alias integrity. Returns
structured results with hints for fixing issues.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 7: Wire Up CLI Commands

### Task 7.1: Implement Command Dispatch

**Files:**
- Create: `src/commands/mod.rs`
- Create: `src/commands/template.rs`
- Create: `src/commands/sound.rs`
- Create: `src/commands/icon.rs`
- Create: `src/commands/config.rs`
- Create: `src/commands/doctor.rs`
- Modify: `src/lib.rs`

**Step 1: Create src/commands/mod.rs**

```rust
//! Command implementations

pub mod config;
pub mod doctor;
pub mod icon;
pub mod sound;
pub mod template;

use crate::cli::{Cli, Commands};
use crate::error::ExitCode;
use anyhow::Result;

/// Dispatch command based on CLI args
pub fn dispatch(cli: &Cli) -> Result<ExitCode> {
    match &cli.command {
        Some(Commands::Template { action }) => template::handle(action),
        Some(Commands::Sound { action }) => sound::handle(action),
        Some(Commands::Icon { action }) => icon::handle(action),
        Some(Commands::Config { action }) => config::handle(action),
        Some(Commands::Setup) => {
            println!("Running setup wizard...");
            // TODO: Implement setup wizard
            Ok(ExitCode::Success)
        }
        Some(Commands::Doctor) => doctor::handle(),
        None => {
            // No subcommand - send notification
            if cli.title.is_some() {
                // TODO: Send notification
                println!("Would send notification: {:?}", cli.title);
                Ok(ExitCode::Success)
            } else {
                // No title and no command - show help
                println!("Use --help for usage information");
                Ok(ExitCode::Success)
            }
        }
    }
}
```

**Step 2: Create src/commands/doctor.rs**

```rust
use crate::doctor::{run_all_checks, CheckStatus};
use crate::error::ExitCode;
use anyhow::Result;

pub fn handle() -> Result<ExitCode> {
    let results = run_all_checks();

    let mut has_errors = false;
    let mut has_warnings = false;

    for result in &results {
        let status_icon = match result.status {
            CheckStatus::Ok => "✓",
            CheckStatus::Warning => {
                has_warnings = true;
                "⚠"
            }
            CheckStatus::Error => {
                has_errors = true;
                "✗"
            }
        };

        println!("{} {}: {}", status_icon, result.name, result.message);

        if let Some(hint) = &result.hint {
            println!("  Hint: {}", hint);
        }
    }

    if has_errors {
        Ok(ExitCode::SystemError)
    } else if has_warnings {
        Ok(ExitCode::Success) // Warnings don't fail
    } else {
        Ok(ExitCode::Success)
    }
}
```

**Step 3: Create src/commands/config.rs**

```rust
use crate::cli::ConfigCommands;
use crate::config::{load_config, save_config, validate_config, Config};
use crate::error::ExitCode;
use anyhow::Result;

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
        ConfigCommands::Set { key, value } => {
            println!("Setting {} = {}", key, value);
            // TODO: Implement config set
            Ok(ExitCode::Success)
        }
        ConfigCommands::Unset { key } => {
            println!("Unsetting {}", key);
            // TODO: Implement config unset
            Ok(ExitCode::Success)
        }
        ConfigCommands::Reset => {
            let config = Config::default();
            save_config(&config, None)?;
            println!("Configuration reset to defaults");
            Ok(ExitCode::Success)
        }
        ConfigCommands::Validate { path } => {
            let config = load_config(path.as_ref().map(|p| std::path::Path::new(p)))?;
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
```

**Step 4: Create src/commands/template.rs**

```rust
use crate::cli::TemplateCommands;
use crate::template::{list_templates, load_template, delete_template};
use crate::error::ExitCode;
use anyhow::Result;

pub fn handle(action: &TemplateCommands) -> Result<ExitCode> {
    match action {
        TemplateCommands::List => {
            let templates = list_templates()?;
            if templates.is_empty() {
                println!("No templates found");
            } else {
                for name in templates {
                    println!("{}", name);
                }
            }
            Ok(ExitCode::Success)
        }
        TemplateCommands::Show { name } => {
            let template = load_template(name)?;
            let json = serde_json::to_string_pretty(&template)?;
            println!("{}", json);
            Ok(ExitCode::Success)
        }
        TemplateCommands::Create => {
            println!("Interactive template creation not yet implemented");
            // TODO: Implement interactive creation
            Ok(ExitCode::Success)
        }
        TemplateCommands::Update { name } => {
            println!("Updating template: {}", name);
            // TODO: Implement update
            Ok(ExitCode::Success)
        }
        TemplateCommands::Delete { name } => {
            delete_template(name)?;
            println!("Deleted template: {}", name);
            Ok(ExitCode::Success)
        }
        TemplateCommands::Validate { name, all } => {
            if *all {
                println!("Validating all templates...");
                // TODO: Implement validate all
            } else if let Some(name) = name {
                let template = load_template(name)?;
                println!("Template '{}' is valid", template.name);
            } else {
                println!("Specify a template name or use --all");
            }
            Ok(ExitCode::Success)
        }
    }
}
```

**Step 5: Create src/commands/sound.rs**

```rust
use crate::cli::SoundCommands;
use crate::alias::{add_sound_alias, list_sound_aliases, remove_sound_alias};
use crate::error::ExitCode;
use anyhow::Result;

pub fn handle(action: &SoundCommands) -> Result<ExitCode> {
    match action {
        SoundCommands::List => {
            let aliases = list_sound_aliases()?;
            if aliases.is_empty() {
                println!("No sound aliases configured");
            } else {
                for (alias, path) in aliases {
                    println!("@{} -> {}", alias, path);
                }
            }
            Ok(ExitCode::Success)
        }
        SoundCommands::Add { alias, path, cache } => {
            add_sound_alias(alias, path, *cache)?;
            println!("Added sound alias: @{}", alias);
            Ok(ExitCode::Success)
        }
        SoundCommands::Remove { alias, with_file } => {
            let warnings = remove_sound_alias(alias, *with_file)?;
            for warning in warnings {
                eprintln!("Warning: {}", warning);
            }
            println!("Removed sound alias: @{}", alias);
            Ok(ExitCode::Success)
        }
        SoundCommands::Prune { target } => {
            println!("Pruning sounds: {:?}", target);
            // TODO: Implement prune
            Ok(ExitCode::Success)
        }
        SoundCommands::Doctor => {
            let results = crate::doctor::check_sound_aliases();
            for result in results {
                println!("{}: {}", result.name, result.message);
            }
            Ok(ExitCode::Success)
        }
    }
}
```

**Step 6: Create src/commands/icon.rs**

```rust
use crate::cli::IconCommands;
use crate::alias::{add_icon_alias, list_icon_aliases, remove_icon_alias, BUNDLED_ICONS};
use crate::error::ExitCode;
use anyhow::Result;

pub fn handle(action: &IconCommands) -> Result<ExitCode> {
    match action {
        IconCommands::List => {
            println!("Bundled icons: {}", BUNDLED_ICONS.join(", "));

            let aliases = list_icon_aliases()?;
            if aliases.is_empty() {
                println!("No custom icon aliases configured");
            } else {
                println!("\nCustom aliases:");
                for (alias, bundle) in aliases {
                    println!("@{} -> {}", alias, bundle);
                }
            }
            Ok(ExitCode::Success)
        }
        IconCommands::Add { alias, path } => {
            add_icon_alias(alias, path)?;
            println!("Added icon alias: @{}", alias);
            Ok(ExitCode::Success)
        }
        IconCommands::Remove { alias, with_bundle } => {
            let warnings = remove_icon_alias(alias, *with_bundle)?;
            for warning in warnings {
                eprintln!("Warning: {}", warning);
            }
            println!("Removed icon alias: @{}", alias);
            Ok(ExitCode::Success)
        }
        IconCommands::Prune { target } => {
            println!("Pruning icons: {:?}", target);
            // TODO: Implement prune
            Ok(ExitCode::Success)
        }
        IconCommands::Doctor => {
            let results = crate::doctor::check_icon_aliases();
            for result in results {
                println!("{}: {}", result.name, result.message);
            }
            Ok(ExitCode::Success)
        }
    }
}
```

**Step 7: Update src/lib.rs**

```rust
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod notification;
pub mod template;
pub mod alias;
pub mod doctor;

use anyhow::Result;
use clap::Parser;

pub use cli::Cli;
pub use error::{AppError, ExitCode};

pub fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    commands::dispatch(&cli)
}
```

**Step 8: Run tests and verify commands work**

Run: `cargo build && ./target/debug/cb doctor`
Expected: Shows health check results

Run: `cargo build && ./target/debug/cb config show`
Expected: Shows default config JSON

**Step 9: Commit**

```bash
git add src/commands/ src/lib.rs
git commit -m "feat: wire up CLI command dispatch

Implements command handlers for doctor, config, template, sound, and
icon subcommands. Each handler maps CLI args to the appropriate
module functions.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 8: Plugin Packaging

### Task 8.1: Create Marketplace and Plugin Structure

**Files:**
- Create: `.claude-plugin/marketplace.json`
- Create: `plugins/core/skills/claude-bell/SKILL.md`
- Create: `plugins/core/skills/claude-bell/references/cli.md`
- Create: `plugins/core/skills/claude-bell/references/examples.md`
- Create: `plugins/core/skills/claude-bell/references/setup.md`
- Create: `plugins/core/skills/claude-bell/references/troubleshooting.md`
- Create: `plugins/hooks-*/hooks/*.md` (all hook plugins)

This task involves creating all the plugin files from the design document. The content is already specified in the design document appendices.

**Step 1: Create directory structure**

```bash
mkdir -p .claude-plugin
mkdir -p plugins/core/skills/claude-bell/references
mkdir -p plugins/core/bin/macos-arm64
mkdir -p plugins/core/bin/macos-x64
mkdir -p plugins/hooks-dangerous/hooks
mkdir -p plugins/hooks-completion/hooks
mkdir -p plugins/hooks-errors/hooks
mkdir -p plugins/hooks-long-running/hooks
mkdir -p plugins/hooks-idle/hooks
mkdir -p plugins/hooks-session/hooks
```

**Step 2: Create marketplace.json**

Copy content from design document Appendix: Marketplace File

**Step 3: Create SKILL.md**

Copy content from design document Appendix: Skill File

**Step 4: Create reference files**

Copy content from the design's skill reference files (cli.md, examples.md, setup.md, troubleshooting.md)

**Step 5: Create hook files**

Create each hook file with appropriate frontmatter and script.

**Step 6: Commit**

```bash
git add .claude-plugin/ plugins/
git commit -m "feat: create plugin packaging structure

Adds marketplace.json, core plugin with skill and references, and
hook plugin placeholders for dangerous, completion, errors,
long-running, idle, and session hooks.

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Phase 9: CI/CD Setup

### Task 9.1: Create CI Workflow

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `scripts/validate-marketplace.sh`
- Create: `scripts/validate-skills.sh`
- Create: `scripts/validate-hooks.sh`

Content for these files is specified in the design document's CI/CD section.

---

### Task 9.2: Create Release Workflow

**Files:**
- Create: `.github/workflows/release.yml`

Content is specified in the design document's CI/CD section.

---

## Phase 10: macOS Notification Integration

### Task 10.1: Research macOS Notification APIs

This task requires investigating:
1. objc2 crate for UserNotifications framework bindings
2. Persistent notification approach (separate bundle vs runtime)
3. Action button and reply handling

**Files to create:**
- `src/notification/macos.rs`
- `src/notification/response.rs`

This phase requires macOS-specific development and testing that can't be fully specified in advance.

---

## Summary

**Total Phases:** 10
**Estimated Tasks:** ~25 detailed tasks

**Key Milestones:**
1. **Phase 1-2 Complete:** Working CLI with exit codes
2. **Phase 3-4 Complete:** Config and template systems
3. **Phase 5-6 Complete:** Alias management and health checks
4. **Phase 7 Complete:** All commands wired up
5. **Phase 8-9 Complete:** Plugins and CI/CD
6. **Phase 10 Complete:** Full macOS notification support

**Next Steps After Plan:**
- Use superpowers:subagent-driven-development for task execution
- Or open parallel session with superpowers:executing-plans
