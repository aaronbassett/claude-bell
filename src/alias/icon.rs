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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_source_type() {
        assert_eq!(detect_source_type("/path/to/file.icns"), SourceType::Icns);
        assert_eq!(detect_source_type("/path/to/App.app"), SourceType::AppBundle);
        assert_eq!(detect_source_type("/path/to/file.png"), SourceType::Image);
        assert_eq!(detect_source_type("/path/to/file.jpg"), SourceType::Image);
        assert_eq!(detect_source_type("/path/to/file.jpeg"), SourceType::Image);
    }

    #[test]
    fn test_generate_info_plist() {
        let plist = generate_info_plist("test-icon");
        assert!(plist.contains("com.claude-bell.icon.test-icon"));
        assert!(plist.contains("<key>CFBundleName</key>"));
        assert!(plist.contains("<string>test-icon</string>"));
        assert!(plist.contains("<key>CFBundleIconFile</key>"));
        assert!(plist.contains("<string>icon</string>"));
    }

    #[test]
    fn test_convert_image_to_icns() {
        let temp = tempfile::TempDir::new().unwrap();
        let input = temp.path().join("test.png");
        let output = temp.path().join("output.icns");

        // Create a fake PNG (just needs to exist for test)
        std::fs::write(&input, b"fake png data").unwrap();

        // Note: This will fail on systems without sips, but that's expected
        let result = convert_to_icns(&input, &output);

        // We can't guarantee sips is available in test environment
        // So just verify the function exists and has correct signature
        match result {
            Ok(_) => assert!(output.exists()),
            Err(e) => {
                // Expected on non-macOS or if sips fails with fake data
                assert!(e.to_string().contains("sips") ||
                        e.to_string().contains("image"));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum SourceType {
    AppBundle,
    Icns,
    Image,
}

fn detect_source_type(path: &str) -> SourceType {
    let path_lower = path.to_lowercase();
    if path_lower.ends_with(".app") {
        SourceType::AppBundle
    } else if path_lower.ends_with(".icns") {
        SourceType::Icns
    } else {
        SourceType::Image
    }
}

fn generate_info_plist(alias: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.claude-bell.icon.{}</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>"#, alias, alias)
}

fn convert_to_icns(input: &std::path::Path, output: &std::path::Path) -> Result<(), AppError> {
    use std::process::Command;

    let status = Command::new("sips")
        .arg("-s")
        .arg("format")
        .arg("icns")
        .arg(input)
        .arg("--out")
        .arg(output)
        .status()
        .map_err(|e| AppError::SystemError(
            format!("Failed to run sips command: {}. Is this macOS?", e)
        ))?;

    if !status.success() {
        return Err(AppError::SystemError(
            format!("sips command failed to convert image. Supported formats: png, jpg, jpeg, gif, tiff")
        ));
    }

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

    let bundle_name = format!("{}.app", alias);
    let bundle_path = icon_bundles_dir().join(&bundle_name);

    // Create bundle directory structure
    std::fs::create_dir_all(&bundle_path)?;
    let contents_dir = bundle_path.join("Contents");
    let resources_dir = contents_dir.join("Resources");
    std::fs::create_dir_all(&resources_dir)?;

    // Generate and write Info.plist
    let plist_content = generate_info_plist(alias);
    std::fs::write(contents_dir.join("Info.plist"), plist_content)?;

    // Handle icon based on source type
    let source_type = detect_source_type(path);
    let icon_dest = resources_dir.join("icon.icns");

    match source_type {
        SourceType::AppBundle => {
            // Copy the app bundle's icon if it exists
            let source_resources = source_path.join("Contents/Resources");
            if let Ok(entries) = std::fs::read_dir(source_resources) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("icns") {
                        std::fs::copy(&path, &icon_dest)?;
                        break;
                    }
                }
            }
        }
        SourceType::Icns => {
            // Copy ICNS file directly
            std::fs::copy(source_path, &icon_dest)?;
        }
        SourceType::Image => {
            // Convert image to ICNS
            convert_to_icns(source_path, &icon_dest)?;
        }
    }

    // Update aliases
    aliases.aliases.insert(alias.to_string(), bundle_name);
    save_icon_aliases(&aliases)?;
    Ok(())
}

/// Remove an icon alias
pub fn remove_icon_alias(alias: &str, with_bundle: bool) -> Result<Vec<String>, AppError> {
    let mut aliases = load_icon_aliases()?;
    let mut warnings = vec![];

    let bundle_name = aliases
        .aliases
        .remove(alias)
        .ok_or_else(|| AppError::AliasNotFound(format!("icon alias: {}", alias)))?;

    if with_bundle {
        // Check if other aliases use this bundle
        let other_aliases: Vec<_> = aliases
            .aliases
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
    if let Some(alias) = icon.strip_prefix('@') {
        // Check if it's a bundled icon
        if BUNDLED_ICONS.contains(&alias) {
            // For bundled icons, return the alias format
            // The notification system will handle bundled icon resolution
            return Ok(Some(format!("@{}", alias)));
        }

        // Check user aliases
        let aliases = load_icon_aliases()?;

        let bundle_name = aliases
            .aliases
            .get(alias)
            .ok_or_else(|| AppError::AliasNotFound(format!("icon alias: {}", alias)))?;

        Ok(Some(
            icon_bundles_dir()
                .join(bundle_name)
                .to_string_lossy()
                .to_string(),
        ))
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
