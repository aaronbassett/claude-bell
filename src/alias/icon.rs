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
            // TODO: Return path to bundled icon
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
