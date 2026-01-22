//! Setup wizard for first-time configuration

use crate::config::{save_config, Config};
use crate::error::ExitCode;
use anyhow::Result;
use dialoguer::{Confirm, Input, Select, MultiSelect};

pub fn run_setup_wizard() -> Result<ExitCode> {
    println!("\n🔔 Welcome to claude-bell setup!\n");
    println!("This wizard will help you configure claude-bell for first use.");
    println!("Data directory: {}\n", crate::config::config_dir().display());

    if !Confirm::new()
        .with_prompt("Continue with setup?")
        .default(true)
        .interact()?
    {
        println!("Setup cancelled. Run `cb setup` anytime to configure.");
        return Ok(ExitCode::Success);
    }

    let mut config = Config::default();

    // Step 1: Default sound
    setup_default_sound(&mut config)?;

    // Step 2: Default timeout
    setup_default_timeout(&mut config)?;

    // Save config
    save_config(&config, None)?;

    // Step 3: Templates
    if Confirm::new()
        .with_prompt("Create common templates?")
        .default(true)
        .interact()?
    {
        setup_templates()?;
    }

    // Step 4: Sound aliases
    if Confirm::new()
        .with_prompt("Configure sound aliases?")
        .default(true)
        .interact()?
    {
        setup_sound_aliases()?;
    }

    // Step 5: Icon aliases
    if Confirm::new()
        .with_prompt("Configure icon aliases?")
        .default(true)
        .interact()?
    {
        setup_icon_aliases()?;
    }

    // Step 6: Test notification
    test_notification(&config)?;

    // Step 7: Summary
    print_summary();

    Ok(ExitCode::Success)
}

fn setup_default_sound(config: &mut Config) -> Result<()> {
    let sounds = vec!["None", "Default", "Ping", "Basso", "Hero", "Funk"];
    let selection = Select::new()
        .with_prompt("Choose default notification sound")
        .items(&sounds)
        .default(0)
        .interact()?;

    if selection > 0 {
        config.defaults.sound = Some(sounds[selection].to_string());

        if Confirm::new()
            .with_prompt("Test this sound?")
            .default(true)
            .interact()?
        {
            // Send test notification with sound
            let test_config = crate::notification::NotificationConfig {
                title: "Sound Test".to_string(),
                subtitle: None,
                message: Some(format!("Testing {} sound", sounds[selection])),
                image: None,
                icon: None,
                sound: Some(sounds[selection].to_string()),
                actions: vec![],
                reply: None,
                url: None,
                persistent: false,
                timeout: None,
                default_value: None,
                on_dismiss: None,
                on_timeout: None,
            };
            let _ = crate::notification::send_notification(test_config);
        }
    }

    Ok(())
}

fn setup_default_timeout(config: &mut Config) -> Result<()> {
    println!("\nDefault timeout controls how long interactive notifications wait for response.");
    println!("Leave empty for no timeout (wait indefinitely).\n");

    let timeout: String = Input::new()
        .with_prompt("Default timeout (e.g., 30s, 5m)")
        .allow_empty(true)
        .interact_text()?;

    config.timeout = if timeout.is_empty() { None } else { Some(timeout) };

    Ok(())
}

fn setup_templates() -> Result<()> {
    use crate::template::Template;
    use std::collections::HashMap;

    let templates = vec![
        ("success", "✓ Success", "Ping", "@success"),
        ("error", "✗ Error", "Basso", "@error"),
        ("warning", "⚠ Warning", "Funk", "@warning"),
        ("info", "ℹ Info", "Default", "@info"),
    ];

    let selections = MultiSelect::new()
        .with_prompt("Which templates do you want to create?")
        .items(&templates.iter().map(|t| t.1).collect::<Vec<_>>())
        .interact()?;

    for idx in selections {
        let (name, title, sound, icon) = templates[idx];
        let template = Template {
            name: name.to_string(),
            description: format!("{} notification template", name),
            title: title.to_string(),
            subtitle: None,
            message: None,
            image: None,
            icon: Some(icon.to_string()),
            sound: Some(sound.to_string()),
            actions: None,
            reply: None,
            url: None,
            persistent: None,
            defaults: HashMap::new(),
        };
        crate::template::save_template(&template)?;
        println!("  ✓ Created template: {}", name);
    }

    Ok(())
}

fn setup_sound_aliases() -> Result<()> {
    println!("\nSound aliases let you use @name instead of full paths.");
    println!("Skipping for now - you can add them later with `cb sound add`.\n");
    Ok(())
}

fn setup_icon_aliases() -> Result<()> {
    println!("\nIcon aliases let you use @name instead of full paths.");
    println!("Skipping for now - you can add them later with `cb icon add`.\n");
    Ok(())
}

fn test_notification(config: &Config) -> Result<()> {
    if !Confirm::new()
        .with_prompt("Send a test notification?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    println!("\n📬 Sending test notification...");

    let test_config = crate::notification::NotificationConfig {
        title: "Test Notification".to_string(),
        subtitle: Some("Claude Bell Setup".to_string()),
        message: Some("If you see this, everything is working!".to_string()),
        image: None,
        icon: None,
        sound: config.defaults.sound.clone(),
        actions: vec!["Looks Good!".to_string()],
        reply: None,
        url: None,
        persistent: false,
        timeout: None,
        default_value: None,
        on_dismiss: None,
        on_timeout: None,
    };

    let _ = crate::notification::send_notification(test_config);
    println!("✓ Test notification sent!\n");

    Ok(())
}

fn print_summary() {
    println!("\n✅ Setup complete!\n");
    println!("Next steps:");
    println!("  • Send a notification: cb -t \"Hello World\"");
    println!("  • Create a template: cb template create");
    println!("  • Check system health: cb doctor");
    println!("  • View configuration: cb config show --pretty\n");
}
