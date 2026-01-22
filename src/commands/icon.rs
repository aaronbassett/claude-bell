//! Icon command implementation - icon alias management

use crate::alias::{add_icon_alias, list_icon_aliases, remove_icon_alias, BUNDLED_ICONS};
use crate::cli::args::IconCommands;
use crate::error::ExitCode;
use anyhow::Result;

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
        IconCommands::Prune { target } => {
            println!("Pruning icons: {:?}", target);
            // TODO: Implement prune logic
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
