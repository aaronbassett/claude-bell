//! Command implementations for CLI subcommands

pub mod config;
pub mod doctor;
pub mod icon;
pub mod sound;
pub mod template;

use crate::cli::args::{Cli, Commands};
use crate::error::ExitCode;
use crate::notification::{send_notification, NotificationConfig};
use anyhow::Result;
use std::collections::HashMap;

/// Send notification from CLI arguments
fn send_notification_from_cli(cli: &Cli) -> Result<ExitCode> {
    // If template is specified, render it first
    let mut effective_cli = cli.clone();

    if let Some(ref template_name) = cli.template {
        // Load and render template
        let template = crate::template::load_template(template_name)?;

        // Parse variables from --var flags
        let mut vars = HashMap::new();
        if let Some(ref var_list) = cli.var {
            for var_str in var_list {
                if let Some((key, value)) = var_str.split_once(':') {
                    vars.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Render template
        let rendered = crate::template::render_template(&template, &vars)?;

        // Apply rendered values to CLI (CLI values override template)
        effective_cli.title = cli.title.clone().or(Some(rendered.title));
        effective_cli.subtitle = cli.subtitle.clone().or(rendered.subtitle);
        effective_cli.message = cli.message.clone().or(rendered.message);
        effective_cli.image = cli.image.clone().or(rendered.image);
        effective_cli.icon = cli.icon.clone().or(rendered.icon);
        effective_cli.sound = cli.sound.clone().or(rendered.sound);
        effective_cli.actions = cli.actions.clone().or(rendered.actions);
        effective_cli.reply = cli.reply.clone().or(rendered.reply);
        effective_cli.url = cli.url.clone().or(rendered.url);

        if rendered.persistent.unwrap_or(false) && !cli.not_persistent {
            effective_cli.persistent = true;
        }
    }

    // Build notification config
    let mut config = NotificationConfig::from_cli(&effective_cli)?;

    // Resolve sound alias if specified
    if let Some(ref sound) = config.sound {
        if sound.starts_with('@') {
            config.sound = Some(crate::alias::resolve_sound(sound)?);
        }
    }

    // Resolve icon alias if specified
    if let Some(ref icon) = config.icon {
        if icon.starts_with('@') {
            if let Some(resolved) = crate::alias::resolve_icon(icon)? {
                config.icon = Some(resolved);
            }
        }
    }

    // Send the notification
    let exit_code = send_notification(config)?;
    Ok(exit_code)
}

/// Dispatch command based on CLI args
pub fn dispatch(cli: &Cli) -> Result<ExitCode> {
    match &cli.command {
        Some(Commands::Template { action }) => template::handle(action),
        Some(Commands::Sound { action }) => sound::handle(action),
        Some(Commands::Icon { action }) => icon::handle(action),
        Some(Commands::Config { action }) => config::handle(action),
        Some(Commands::Setup) => {
            println!("Running setup wizard...");
            // TODO: Implement setup wizard
            Ok(ExitCode::Success)
        }
        Some(Commands::Doctor) => doctor::handle(),
        None => {
            // No subcommand - send notification or apply template
            if cli.title.is_some() || cli.template.is_some() {
                send_notification_from_cli(cli)
            } else {
                // No title and no command - show help
                println!("Use --help for usage information");
                Ok(ExitCode::Success)
            }
        }
    }
}
