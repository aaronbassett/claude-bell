//! Icon command implementation - icon alias management

use crate::alias::{add_icon_alias, list_icon_aliases, remove_icon_alias, BUNDLED_ICONS};
use crate::cli::args::IconCommands;
use crate::error::ExitCode;
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

fn find_orphaned_icon_bundles() -> Result<Vec<PathBuf>> {
    use crate::alias::icon::{load_icon_aliases, icon_bundles_dir};

    let bundles_dir = icon_bundles_dir();
    let aliases = load_icon_aliases()?;

    let mut bundle_paths = HashSet::new();
    if bundles_dir.exists() {
        for entry in std::fs::read_dir(&bundles_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("app") {
                bundle_paths.insert(entry.path());
            }
        }
    }

    // Remove bundles that are referenced by aliases
    for bundle_name in aliases.aliases.values() {
        bundle_paths.remove(&bundles_dir.join(bundle_name));
    }

    Ok(bundle_paths.into_iter().collect())
}

fn find_dangling_icon_aliases() -> Result<Vec<String>> {
    use crate::alias::icon::{load_icon_aliases, icon_bundles_dir};

    let aliases = load_icon_aliases()?;
    let bundles_dir = icon_bundles_dir();
    let mut dangling = vec![];

    for (alias, bundle_name) in &aliases.aliases {
        let bundle_path = bundles_dir.join(bundle_name);
        if !bundle_path.exists() {
            dangling.push(alias.clone());
        }
    }

    Ok(dangling)
}

/// Handle icon subcommands
pub fn handle(action: &IconCommands) -> Result<ExitCode> {
    match action {
        IconCommands::List => {
            println!("Bundled icons: {}", BUNDLED_ICONS.join(", "));

            let aliases = list_icon_aliases()?;
            if aliases.is_empty() {
                println!("\nNo custom icon aliases configured");
            } else {
                println!("\nCustom aliases:");
                for (alias, bundle) in aliases {
                    println!("  @{} -> {}", alias, bundle);
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
        IconCommands::Prune { target, dry_run, yes } => {
            use crate::alias::icon::{load_icon_aliases, save_icon_aliases};
            use dialoguer::Confirm;

            let target = target.as_deref().unwrap_or("all");

            let (orphaned_bundles, dangling_aliases) = match target {
                "all" => (
                    find_orphaned_icon_bundles()?,
                    find_dangling_icon_aliases()?
                ),
                "bundles" => (find_orphaned_icon_bundles()?, vec![]),
                "aliases" => (vec![], find_dangling_icon_aliases()?),
                _ => return Err(anyhow::anyhow!("Invalid target: {}. Use: all, bundles, or aliases", target).into()),
            };

            if orphaned_bundles.is_empty() && dangling_aliases.is_empty() {
                println!("✓ No cleanup needed - everything looks good!");
                return Ok(ExitCode::Success);
            }

            // Show what will be removed
            if !orphaned_bundles.is_empty() {
                println!("\nOrphaned bundles (no aliases point to them):");
                for bundle in &orphaned_bundles {
                    println!("  - {}", bundle.display());
                }
            }

            if !dangling_aliases.is_empty() {
                println!("\nDangling aliases (point to missing bundles):");
                for alias in &dangling_aliases {
                    println!("  - @{}", alias);
                }
            }

            println!("\nTotal: {} bundles, {} aliases",
                orphaned_bundles.len(), dangling_aliases.len());

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
            let mut removed_bundles = 0;
            let mut removed_aliases = 0;

            for bundle in orphaned_bundles {
                if let Err(e) = std::fs::remove_dir_all(&bundle) {
                    eprintln!("Warning: Failed to remove {}: {}", bundle.display(), e);
                } else {
                    removed_bundles += 1;
                }
            }

            if !dangling_aliases.is_empty() {
                let mut aliases = load_icon_aliases()?;
                for alias in &dangling_aliases {
                    aliases.aliases.remove(alias);
                    removed_aliases += 1;
                }
                save_icon_aliases(&aliases)?;
            }

            println!("\n✓ Removed {} bundles, {} aliases", removed_bundles, removed_aliases);
            Ok(ExitCode::Success)
        }
        IconCommands::Doctor => {
            let results = crate::doctor::check_icon_aliases();
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
