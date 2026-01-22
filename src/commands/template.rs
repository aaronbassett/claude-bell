//! Template command implementation - notification template management

use crate::cli::args::TemplateCommands;
use crate::error::ExitCode;
use crate::template::{delete_template, list_templates, load_template};
use anyhow::Result;

/// Handle template subcommands
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
                let templates = list_templates()?;
                let mut all_valid = true;
                for template_name in templates {
                    match load_template(&template_name) {
                        Ok(_) => println!("  ✓ {}", template_name),
                        Err(e) => {
                            println!("  ✗ {}: {}", template_name, e);
                            all_valid = false;
                        }
                    }
                }
                if all_valid {
                    println!("All templates are valid");
                    Ok(ExitCode::Success)
                } else {
                    Ok(ExitCode::UserError)
                }
            } else if let Some(name) = name {
                let template = load_template(name)?;
                println!("Template '{}' is valid", template.name);
                Ok(ExitCode::Success)
            } else {
                println!("Specify a template name or use --all");
                Ok(ExitCode::UserError)
            }
        }
    }
}
