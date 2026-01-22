//! Sound command implementation - sound alias management

use crate::alias::{add_sound_alias, list_sound_aliases, remove_sound_alias};
use crate::cli::args::SoundCommands;
use crate::error::ExitCode;
use anyhow::Result;

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
        SoundCommands::Prune { target } => {
            println!("Pruning sounds: {:?}", target);
            // TODO: Implement prune logic
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
