//! Sound command implementation - sound alias management

use crate::alias::{add_sound_alias, list_sound_aliases, remove_sound_alias};
use crate::cli::args::SoundCommands;
use crate::error::ExitCode;
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

fn find_orphaned_sound_files() -> Result<Vec<PathBuf>> {
    use crate::alias::sound::{load_sound_aliases, sound_files_dir};

    let files_dir = sound_files_dir();
    let aliases = load_sound_aliases()?;

    let mut cached_files = HashSet::new();
    if files_dir.exists() {
        for entry in std::fs::read_dir(&files_dir)? {
            let entry = entry?;
            cached_files.insert(entry.path());
        }
    }

    // Remove files that are referenced by aliases
    for path in aliases.aliases.values() {
        cached_files.remove(Path::new(path));
    }

    Ok(cached_files.into_iter().collect())
}

fn find_dangling_sound_aliases() -> Result<Vec<String>> {
    use crate::alias::sound::load_sound_aliases;

    let aliases = load_sound_aliases()?;
    let mut dangling = vec![];

    for (alias, path) in &aliases.aliases {
        if !Path::new(path).exists() {
            dangling.push(alias.clone());
        }
    }

    Ok(dangling)
}

/// Handle sound subcommands
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
        SoundCommands::Prune { target, dry_run, yes } => {
            use crate::alias::sound::{load_sound_aliases, save_sound_aliases};
            use dialoguer::Confirm;

            let target = target.as_deref().unwrap_or("all");

            let (orphaned_files, dangling_aliases) = match target {
                "all" => (
                    find_orphaned_sound_files()?,
                    find_dangling_sound_aliases()?
                ),
                "files" => (find_orphaned_sound_files()?, vec![]),
                "aliases" => (vec![], find_dangling_sound_aliases()?),
                _ => return Err(anyhow::anyhow!("Invalid target: {}. Use: all, files, or aliases", target).into()),
            };

            if orphaned_files.is_empty() && dangling_aliases.is_empty() {
                println!("✓ No cleanup needed - everything looks good!");
                return Ok(ExitCode::Success);
            }

            // Show what will be removed
            if !orphaned_files.is_empty() {
                println!("\nOrphaned files (no aliases point to them):");
                for file in &orphaned_files {
                    println!("  - {}", file.display());
                }
            }

            if !dangling_aliases.is_empty() {
                println!("\nDangling aliases (point to missing files):");
                for alias in &dangling_aliases {
                    println!("  - @{}", alias);
                }
            }

            println!("\nTotal: {} files, {} aliases",
                orphaned_files.len(), dangling_aliases.len());

            if *dry_run {
                println!("\n(dry run - no changes made)");
                return Ok(ExitCode::Success);
            }

            // Confirm
            if !*yes {
                if !Confirm::new()
                    .with_prompt("Remove these items?")
                    .default(false)
                    .interact()?
                {
                    println!("Cancelled.");
                    return Ok(ExitCode::Success);
                }
            }

            // Execute cleanup
            let mut removed_files = 0;
            let mut removed_aliases = 0;

            for file in orphaned_files {
                if let Err(e) = std::fs::remove_file(&file) {
                    eprintln!("Warning: Failed to remove {}: {}", file.display(), e);
                } else {
                    removed_files += 1;
                }
            }

            if !dangling_aliases.is_empty() {
                let mut aliases = load_sound_aliases()?;
                for alias in &dangling_aliases {
                    aliases.aliases.remove(alias);
                    removed_aliases += 1;
                }
                save_sound_aliases(&aliases)?;
            }

            println!("\n✓ Removed {} files, {} aliases", removed_files, removed_aliases);
            Ok(ExitCode::Success)
        }
        SoundCommands::Doctor => {
            let results = crate::doctor::check_sound_aliases();
            for result in results {
                let status_icon = match result.status {
                    crate::doctor::CheckStatus::Ok => "✓",
                    crate::doctor::CheckStatus::Warning => "⚠",
                    crate::doctor::CheckStatus::Error => "✗",
                };
                println!("{} {}: {}", status_icon, result.name, result.message);
                if let Some(hint) = &result.hint {
                    println!("  Hint: {}", hint);
                }
            }
            Ok(ExitCode::Success)
        }
    }
}
