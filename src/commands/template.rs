//! Template command implementation - notification template management

use crate::cli::args::TemplateCommands;
use crate::error::ExitCode;
use crate::template::{delete_template, list_templates, load_template};
use anyhow::Result;
use dialoguer::{Confirm, Input};

fn create_interactive() -> Result<crate::template::Template> {
    use crate::template::Template;
    use std::collections::HashMap;

    println!("\n📝 Create New Template\n");

    let name: String = Input::new().with_prompt("Template name").interact()?;

    let title: String = Input::new().with_prompt("Title (required)").interact()?;

    let subtitle: String = Input::new()
        .with_prompt("Subtitle (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;
    let subtitle = if subtitle.is_empty() {
        None
    } else {
        Some(subtitle)
    };

    let message: String = Input::new()
        .with_prompt("Message (optional)")
        .allow_empty(true)
        .interact_text()?;
    let message = if message.is_empty() {
        None
    } else {
        Some(message)
    };

    let sound: String = Input::new()
        .with_prompt("Sound (optional, @alias or path)")
        .allow_empty(true)
        .interact_text()?;
    let sound = if sound.is_empty() { None } else { Some(sound) };

    let icon: String = Input::new()
        .with_prompt("Icon (optional, @alias or path)")
        .allow_empty(true)
        .interact_text()?;
    let icon = if icon.is_empty() { None } else { Some(icon) };

    let actions: String = Input::new()
        .with_prompt("Actions (optional, comma-separated)")
        .allow_empty(true)
        .interact_text()?;
    let actions = if actions.is_empty() {
        None
    } else {
        Some(actions.split(',').map(|s| s.trim().to_string()).collect())
    };

    let reply: String = Input::new()
        .with_prompt("Reply placeholder (optional)")
        .allow_empty(true)
        .interact_text()?;
    let reply = if reply.is_empty() { None } else { Some(reply) };

    let url: String = Input::new()
        .with_prompt("URL (optional)")
        .allow_empty(true)
        .interact_text()?;
    let url = if url.is_empty() { None } else { Some(url) };

    let persistent = Confirm::new()
        .with_prompt("Persistent notification?")
        .default(false)
        .interact()?;
    let persistent = if persistent { Some(true) } else { None };

    Ok(Template {
        name,
        description: String::new(),
        title,
        subtitle,
        message,
        image: None,
        icon,
        sound,
        actions,
        reply,
        url,
        persistent,
        defaults: HashMap::new(),
    })
}

fn update_interactive(template: &mut crate::template::Template) -> Result<()> {
    println!("\n✏️  Update Template: {}\n", template.name);
    println!("Current values shown in brackets. Press Enter to keep, or type new value.\n");

    let title: String = Input::new()
        .with_prompt("Title")
        .default(template.title.clone())
        .interact()?;
    template.title = title;

    let subtitle: String = Input::new()
        .with_prompt("Subtitle")
        .default(template.subtitle.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.subtitle = if subtitle.is_empty() {
        None
    } else {
        Some(subtitle)
    };

    let message: String = Input::new()
        .with_prompt("Message")
        .default(template.message.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.message = if message.is_empty() {
        None
    } else {
        Some(message)
    };

    let sound: String = Input::new()
        .with_prompt("Sound")
        .default(template.sound.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.sound = if sound.is_empty() { None } else { Some(sound) };

    let icon: String = Input::new()
        .with_prompt("Icon")
        .default(template.icon.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.icon = if icon.is_empty() { None } else { Some(icon) };

    let actions_str: String = Input::new()
        .with_prompt("Actions (comma-separated)")
        .default(
            template
                .actions
                .as_ref()
                .map(|a| a.join(", "))
                .unwrap_or_default(),
        )
        .allow_empty(true)
        .interact_text()?;
    template.actions = if actions_str.is_empty() {
        None
    } else {
        Some(
            actions_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        )
    };

    let reply: String = Input::new()
        .with_prompt("Reply placeholder")
        .default(template.reply.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.reply = if reply.is_empty() { None } else { Some(reply) };

    let url: String = Input::new()
        .with_prompt("URL")
        .default(template.url.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;
    template.url = if url.is_empty() { None } else { Some(url) };

    let persistent = Confirm::new()
        .with_prompt("Persistent notification?")
        .default(template.persistent.unwrap_or(false))
        .interact()?;
    template.persistent = if persistent { Some(true) } else { None };

    Ok(())
}

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
        TemplateCommands::Show { name, render } => {
            let template = load_template(name)?;
            if *render {
                // TODO: Implement template rendering with variables
                println!("Template rendering not yet implemented");
                println!("{}", serde_json::to_string_pretty(&template)?);
            } else {
                let json = serde_json::to_string_pretty(&template)?;
                println!("{}", json);
            }
            Ok(ExitCode::Success)
        }
        TemplateCommands::Create {
            name,
            title,
            subtitle,
            message,
            sound,
            icon,
            actions,
            reply,
            url,
            persistent,
            json,
        } => {
            let template = if *json {
                // Read JSON from stdin
                use std::io::Read;
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                serde_json::from_str(&buffer)?
            } else if let (Some(name), Some(title)) = (name, title) {
                // Create from flags
                use crate::template::Template;
                use std::collections::HashMap;

                Template {
                    name: name.clone(),
                    description: String::new(),
                    title: title.clone(),
                    subtitle: subtitle.clone(),
                    message: message.clone(),
                    image: None,
                    icon: icon.clone(),
                    sound: sound.clone(),
                    actions: actions.clone(),
                    reply: reply.clone(),
                    url: url.clone(),
                    persistent: if *persistent { Some(true) } else { None },
                    defaults: HashMap::new(),
                }
            } else {
                // Interactive mode
                create_interactive()?
            };

            // Save template
            crate::template::save_template(&template)?;
            println!("Created template: {}", template.name);
            Ok(ExitCode::Success)
        }
        TemplateCommands::Update {
            name,
            title,
            subtitle,
            message,
            sound,
            icon,
            actions,
            reply,
            url,
            persistent,
            json,
        } => {
            let mut template = load_template(name)?;

            if *json {
                // Merge JSON from stdin
                use std::io::Read;
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                let partial: serde_json::Value = serde_json::from_str(&buffer)?;

                // Merge fields
                if let Some(t) = partial.get("title").and_then(|v| v.as_str()) {
                    template.title = t.to_string();
                }
                if let Some(s) = partial.get("subtitle").and_then(|v| v.as_str()) {
                    template.subtitle = Some(s.to_string());
                }
                if let Some(m) = partial.get("message").and_then(|v| v.as_str()) {
                    template.message = Some(m.to_string());
                }
                if let Some(s) = partial.get("sound").and_then(|v| v.as_str()) {
                    template.sound = Some(s.to_string());
                }
                if let Some(i) = partial.get("icon").and_then(|v| v.as_str()) {
                    template.icon = Some(i.to_string());
                }
                if let Some(a) = partial.get("actions").and_then(|v| v.as_array()) {
                    template.actions = Some(
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect(),
                    );
                }
                if let Some(r) = partial.get("reply").and_then(|v| v.as_str()) {
                    template.reply = Some(r.to_string());
                }
                if let Some(u) = partial.get("url").and_then(|v| v.as_str()) {
                    template.url = Some(u.to_string());
                }
                if let Some(p) = partial.get("persistent").and_then(|v| v.as_bool()) {
                    template.persistent = Some(p);
                }
            } else if title.is_some()
                || subtitle.is_some()
                || message.is_some()
                || sound.is_some()
                || icon.is_some()
                || actions.is_some()
                || reply.is_some()
                || url.is_some()
                || persistent.is_some()
            {
                // Update from flags (only update specified fields)
                if let Some(t) = title {
                    template.title = t.clone();
                }
                if let Some(s) = subtitle {
                    template.subtitle = Some(s.clone());
                }
                if let Some(m) = message {
                    template.message = Some(m.clone());
                }
                if let Some(s) = sound {
                    template.sound = Some(s.clone());
                }
                if let Some(i) = icon {
                    template.icon = Some(i.clone());
                }
                if let Some(a) = actions {
                    template.actions = Some(a.clone());
                }
                if let Some(r) = reply {
                    template.reply = Some(r.clone());
                }
                if let Some(u) = url {
                    template.url = Some(u.clone());
                }
                if let Some(p) = persistent {
                    template.persistent = Some(*p);
                }
            } else {
                // Interactive mode
                update_interactive(&mut template)?;
            }

            crate::template::save_template(&template)?;
            println!("Updated template: {}", name);
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
