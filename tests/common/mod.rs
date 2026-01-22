//! Test utilities for claude-bell integration tests.
//!
//! This module provides helper functions for setting up temporary directories
//! and fixtures that mimic the ~/.claude-bell/ structure.

use std::path::PathBuf;
use tempfile::TempDir;

/// Create a temporary directory structure mimicking ~/.claude-bell/
///
/// Returns a tuple containing the `TempDir` (which must be kept alive for the
/// duration of the test) and the `PathBuf` to the base directory.
///
/// # Directory Structure
///
/// ```text
/// <temp_dir>/
/// ├── templates/
/// ├── sounds/
/// │   └── files/
/// ├── icons/
/// │   └── bundles/
/// └── apps/
/// ```
///
/// # Example
///
/// ```ignore
/// let (temp_dir, base_path) = setup_test_dir();
/// // temp_dir keeps the directory alive
/// // base_path can be used to access files
/// create_test_config(&base_path, r#"{"version": 1}"#);
/// ```
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

/// Create a test config file at the specified base path.
///
/// # Arguments
///
/// * `base_path` - The base directory path (from `setup_test_dir`)
/// * `content` - The JSON content to write to config.json
///
/// # Example
///
/// ```ignore
/// let (temp_dir, base_path) = setup_test_dir();
/// create_test_config(&base_path, r#"{"version": 1}"#);
/// ```
pub fn create_test_config(base_path: &PathBuf, content: &str) {
    std::fs::write(base_path.join("config.json"), content).unwrap();
}

/// Create a test template file at the specified base path.
///
/// # Arguments
///
/// * `base_path` - The base directory path (from `setup_test_dir`)
/// * `name` - The template name (without .json extension)
/// * `content` - The JSON content to write to the template file
///
/// # Example
///
/// ```ignore
/// let (temp_dir, base_path) = setup_test_dir();
/// create_test_template(&base_path, "success", r#"{"title": "Success!"}"#);
/// // Creates: <base_path>/templates/success.json
/// ```
pub fn create_test_template(base_path: &PathBuf, name: &str, content: &str) {
    let template_path = base_path.join("templates").join(format!("{}.json", name));
    std::fs::write(template_path, content).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_test_dir_creates_structure() {
        let (_temp_dir, base_path) = setup_test_dir();

        assert!(base_path.join("templates").is_dir());
        assert!(base_path.join("sounds/files").is_dir());
        assert!(base_path.join("icons/bundles").is_dir());
        assert!(base_path.join("apps").is_dir());
    }

    #[test]
    fn test_create_test_config() {
        let (_temp_dir, base_path) = setup_test_dir();
        let content = r#"{"version": 1}"#;

        create_test_config(&base_path, content);

        let config_path = base_path.join("config.json");
        assert!(config_path.is_file());
        assert_eq!(std::fs::read_to_string(config_path).unwrap(), content);
    }

    #[test]
    fn test_create_test_template() {
        let (_temp_dir, base_path) = setup_test_dir();
        let content = r#"{"title": "Test"}"#;

        create_test_template(&base_path, "my-template", content);

        let template_path = base_path.join("templates/my-template.json");
        assert!(template_path.is_file());
        assert_eq!(std::fs::read_to_string(template_path).unwrap(), content);
    }
}
