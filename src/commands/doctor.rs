//! Doctor command implementation - system health checks

use crate::doctor::{run_all_checks, CheckStatus};
use crate::error::ExitCode;
use anyhow::Result;

/// Handle the doctor command
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
