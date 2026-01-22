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

    let path = aliases
        .aliases
        .remove(alias)
        .ok_or_else(|| AppError::AliasNotFound(format!("sound alias: {}", alias)))?;

    if with_file && path.starts_with("files/") {
        // Check if other aliases use this file
        let other_aliases: Vec<_> = aliases
            .aliases
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
    if let Some(alias) = sound.strip_prefix('@') {
        let aliases = load_sound_aliases()?;

        let path = aliases
            .aliases
            .get(alias)
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
