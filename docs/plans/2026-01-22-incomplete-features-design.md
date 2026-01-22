# Claude Bell - Incomplete Features Implementation Design

**Date:** 2026-01-22
**Status:** Approved
**Author:** Aaron Bassett (with Claude)

## Overview

This document details the implementation design for 8 incomplete features in claude-bell, addressing TODOs across command management, resource cleanup, and core notification functionality.

## Implementation Priority

1. **High Impact**: Icon bundle generation, Interactive notifications
2. **Medium Impact**: Template create/update, Setup wizard
3. **Lower Impact**: Config set/unset, Resource pruning

---

## 1. Icon Bundle Generation

### Current State
- Icon alias system stores paths but doesn't generate `.app` bundles
- `add_icon_alias()` has TODO at src/alias/icon.rs:69
- `resolve_icon()` has TODO at src/alias/icon.rs:119

### Design

Generate proper macOS `.app` bundles with correct structure:

```
~/.claude-bell/icons/bundles/myicon.app/
├── Contents/
    ├── Info.plist
    └── Resources/
        └── icon.icns
```

#### Info.plist Template
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.claude-bell.icon.{alias}</string>
    <key>CFBundleName</key>
    <string>{alias}</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>
```

#### Image Conversion Strategy
- **Source is .app**: Copy bundle identifier, use as-is
- **Source is .icns**: Copy to bundle Resources/
- **Source is image (png/jpg/etc)**: Convert using `sips`:
  ```bash
  sips -s format icns input.png --out icon.icns
  ```

#### API Changes
```rust
// src/alias/icon.rs
pub fn add_icon_alias(alias: &str, path: &str) -> Result<(), AppError> {
    // Validate source exists
    // Determine source type (.app, .icns, image)
    // Generate bundle in ~/.claude-bell/icons/bundles/{alias}.app
    // Update aliases.json with bundle path
}

pub fn resolve_icon(icon: &str) -> Result<Option<String>, AppError> {
    // Return full path to generated .app bundle
    // ~/.claude-bell/icons/bundles/{alias}.app
}
```

#### Error Handling
- Invalid image format: Clear error message with supported formats
- `sips` command fails: Fallback error with instructions
- Bundle generation fails: Clean up partial bundles

---

## 2. Interactive Notifications

### Current State
- Notifications sent as fire-and-forget
- Warnings at src/notification/macos.rs:112-116
- `NotificationResponse` enum exists but unused

### Design

Implement full interactive support with response capture:

#### Response Flow
```
User clicks action → mac-notification-sys returns HashMap
→ parse_notification_response() → NotificationResponse enum
→ Print to stdout + exit with appropriate code
```

#### Output Format
- **Action clicked**: Print button text to stdout, exit 0
- **Reply submitted**: Print reply text to stdout, exit 0
- **Dismissed**: Print `--on-dismiss` value (or `--default`), exit 1
- **Timeout**: Print `--on-timeout` value (or `--default`), exit 2
- **URL click**: Open URL, print "opened", exit 0

#### API Changes
```rust
// src/notification/macos.rs
pub fn send_notification(config: NotificationConfig) -> Result<ExitCode, AppError> {
    // Remove warnings
    // Check if interactive (has actions/reply/url)
    if config.is_interactive() {
        // Wait for response
        let response = wait_for_response(&config)?;
        handle_response(response, &config)
    } else {
        // Fire-and-forget
        sys_send(...)
    }
}

fn handle_response(
    response: NotificationResponse,
    config: &NotificationConfig
) -> Result<ExitCode, AppError> {
    match response {
        NotificationResponse::Action(text) => {
            println!("{}", text);
            Ok(ExitCode::Success)
        }
        NotificationResponse::Reply(text) => {
            println!("{}", text);
            Ok(ExitCode::Success)
        }
        NotificationResponse::Dismissed => {
            if let Some(val) = &config.on_dismiss.or(config.default.as_ref()) {
                println!("{}", val);
            }
            Ok(ExitCode::UserError)
        }
        NotificationResponse::Timeout => {
            if let Some(val) = &config.on_timeout.or(config.default.as_ref()) {
                println!("{}", val);
            }
            Ok(ExitCode::Timeout)
        }
    }
}
```

#### URL Handling
```rust
// Use `open` command on macOS
if let Some(url) = &config.url {
    std::process::Command::new("open")
        .arg(url)
        .spawn()?;
    println!("opened");
}
```

#### Timeout Implementation
- Use `--timeout` flag to set max wait duration
- Default: wait indefinitely (blocking notifications)
- Return timeout response if exceeded

---

## 3. Template Management

### Current State
- `template create`: Placeholder at src/commands/template.rs:30
- `template update`: Placeholder at src/commands/template.rs:35

### Design

Three input modes for maximum flexibility:

#### A) Interactive Mode (Humans)
```bash
cb template create
# Prompts:
# - Template name: [required]
# - Title: [required]
# - Subtitle: [optional, press Enter to skip]
# - Message: [optional]
# - Sound: [optional, @alias or path]
# - Icon: [optional, @alias or path]
# - Actions: [optional, comma-separated]
# - Reply placeholder: [optional]
# - Persistent: [y/N]
# - URL: [optional]
```

#### B) CLI Flags Mode (Scripts)
```bash
cb template create \
  --name build-complete \
  --title "Build Complete" \
  --sound "@success" \
  --actions "View Logs,Dismiss"
```

#### C) JSON stdin Mode (LLMs)
```bash
echo '{
  "name": "build-complete",
  "title": "Build Complete",
  "sound": "@success",
  "actions": ["View Logs", "Dismiss"]
}' | cb template create --json
```

#### Update Command
Same three modes:

**Interactive:**
```bash
cb template update build-complete
# Shows current values, prompts which to change:
# - Title [Build Complete]:
# - Sound [@success]: @ping
# - (press Enter to keep, type new value to change)
```

**CLI Flags:**
```bash
cb template update build-complete --title "Build Done" --sound "@ping"
# Only updates specified fields
```

**JSON stdin:**
```bash
echo '{"title": "Build Done", "sound": "@ping"}' | cb template update build-complete --json
# Merges with existing template
```

#### Implementation
```rust
// src/commands/template.rs

pub fn handle(action: &TemplateCommands) -> Result<ExitCode> {
    match action {
        TemplateCommands::Create {
            name, title, subtitle, // ... all optional fields
            json
        } => {
            if *json {
                create_from_json()?
            } else if name.is_some() {
                create_from_flags(name, title, subtitle, ...)?
            } else {
                create_interactive()?
            }
        }
        TemplateCommands::Update {
            name,
            title, subtitle, // ... all optional fields
            json
        } => {
            let mut template = load_template(name)?;

            if *json {
                update_from_json(&mut template)?
            } else if has_any_flags(title, subtitle, ...) {
                update_from_flags(&mut template, title, subtitle, ...)?
            } else {
                update_interactive(&mut template)?
            }

            save_template(&template)?
        }
    }
}

fn create_interactive() -> Result<Template> {
    use dialoguer::{Input, Confirm, Select};

    let name: String = Input::new()
        .with_prompt("Template name")
        .interact()?;

    let title: String = Input::new()
        .with_prompt("Title")
        .interact()?;

    let subtitle: Option<String> = Input::new()
        .with_prompt("Subtitle (optional)")
        .allow_empty(true)
        .interact_text()
        .ok();

    // ... rest of fields

    Ok(Template { name, title, subtitle, ... })
}

fn update_interactive(template: &mut Template) -> Result<()> {
    println!("Current values shown in [brackets]. Press Enter to keep, or type new value:");

    let new_title: String = Input::new()
        .with_prompt("Title")
        .default(template.title.clone())
        .interact()?;
    template.title = new_title;

    // ... rest of fields

    Ok(())
}
```

#### Dependencies
Add to Cargo.toml:
```toml
dialoguer = "0.11"  # For interactive prompts
```

---

## 4. Setup Wizard

### Current State
- Placeholder at src/commands/mod.rs:86

### Design

Comprehensive first-run configuration wizard.

#### Wizard Flow

```
1. Welcome
   - Explain what claude-bell is
   - Show data directory location
   - Confirm user wants to proceed

2. Default Sound
   - List available system sounds
   - Let user pick default (or none)
   - Test the sound

3. Default Timeout
   - Explain timeout behavior
   - Set default (30s recommended)
   - Can be overridden per-notification

4. Create Initial Templates
   - Offer to create common templates:
     * "Success" - green checkmark, success sound
     * "Error" - red X, error sound
     * "Warning" - yellow !, warning sound
     * "Info" - blue i, default sound
   - User can skip or customize

5. Configure Sound Aliases
   - Offer to set up common aliases:
     * @success -> system sound
     * @error -> system sound
     * @warning -> system sound
   - User can skip

6. Configure Icon Aliases
   - Offer to set up common aliases:
     * @success -> green icon
     * @error -> red icon
     * @warning -> yellow icon
   - User can skip or provide custom images

7. Test Notification
   - Send test notification with chosen defaults
   - Verify it displays correctly
   - Test interactive features (action buttons)

8. Summary
   - Show what was configured
   - Print next steps
   - Suggest running `cb doctor` periodically
```

#### Implementation

```rust
// src/commands/setup.rs (new file)

use dialoguer::{Confirm, Input, Select, MultiSelect};

pub fn run_setup_wizard() -> Result<ExitCode> {
    println!("\n🔔 Welcome to claude-bell setup!\n");
    println!("This wizard will help you configure claude-bell for first use.");
    println!("Data directory: {}\n", data_dir().display());

    if !Confirm::new()
        .with_prompt("Continue with setup?")
        .default(true)
        .interact()?
    {
        println!("Setup cancelled. Run `cb setup` anytime to configure.");
        return Ok(ExitCode::Success);
    }

    let mut config = Config::default();

    // Step 2: Default sound
    setup_default_sound(&mut config)?;

    // Step 3: Default timeout
    setup_default_timeout(&mut config)?;

    // Save config
    save_config(&config, None)?;

    // Step 4: Templates
    if Confirm::new()
        .with_prompt("Create common templates?")
        .default(true)
        .interact()?
    {
        setup_templates()?;
    }

    // Step 5: Sound aliases
    if Confirm::new()
        .with_prompt("Configure sound aliases?")
        .default(true)
        .interact()?
    {
        setup_sound_aliases()?;
    }

    // Step 6: Icon aliases
    if Confirm::new()
        .with_prompt("Configure icon aliases?")
        .default(true)
        .interact()?
    {
        setup_icon_aliases()?;
    }

    // Step 7: Test
    test_notification(&config)?;

    // Step 8: Summary
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

        // Test the sound
        if Confirm::new()
            .with_prompt("Test this sound?")
            .default(true)
            .interact()?
        {
            // Play test notification
            // ...
        }
    }

    Ok(())
}

fn setup_templates() -> Result<()> {
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
            title: title.to_string(),
            sound: Some(sound.to_string()),
            icon: Some(icon.to_string()),
            ..Default::default()
        };
        save_template(&template)?;
        println!("  Created template: {}", name);
    }

    Ok(())
}

fn test_notification(config: &Config) -> Result<()> {
    println!("\n📬 Sending test notification...");

    let test_config = NotificationConfig {
        title: "Test Notification".to_string(),
        subtitle: Some("Claude Bell Setup".to_string()),
        message: Some("If you see this, everything is working!".to_string()),
        sound: config.defaults.sound.clone(),
        actions: Some(vec!["Looks Good!".to_string()]),
        ..Default::default()
    };

    send_notification(test_config)?;
    println!("✓ Test notification sent successfully!\n");

    Ok(())
}
```

#### CLI Args

```rust
// src/cli/args.rs
#[derive(Parser)]
pub enum Commands {
    /// Run first-time setup wizard
    Setup {
        /// Skip confirmation prompts (use defaults)
        #[arg(long)]
        yes: bool,

        /// Minimal setup (skip templates/aliases)
        #[arg(long)]
        minimal: bool,
    },
    // ... rest
}
```

---

## 5. Configuration Management

### Current State
- `config set`: Placeholder at src/commands/config.rs:23
- `config unset`: Placeholder at src/commands/config.rs:28

### Design

Support both dot notation (humans) and JSON stdin (LLMs).

#### Dot Notation Examples
```bash
cb config set defaults.sound "Ping"
cb config set defaults.timeout "30s"
cb config set defaults.persistent true
cb config unset defaults.sound
```

#### JSON stdin Examples
```bash
echo '{"defaults": {"sound": "Ping", "timeout": "30s"}}' | cb config set --json

echo '{"defaults": {"sound": null}}' | cb config set --json  # unset via null
```

#### Implementation

```rust
// src/commands/config.rs

pub fn handle(action: &ConfigCommands) -> Result<ExitCode> {
    match action {
        ConfigCommands::Set { key, value, json } => {
            let mut config = load_config(None)?;

            if *json {
                set_from_json(&mut config)?;
            } else {
                set_from_key_path(&mut config, key, value)?;
            }

            save_config(&config, None)?;
            println!("Configuration updated");
            Ok(ExitCode::Success)
        }
        ConfigCommands::Unset { key } => {
            let mut config = load_config(None)?;
            unset_key_path(&mut config, key)?;
            save_config(&config, None)?;
            println!("Configuration key removed: {}", key);
            Ok(ExitCode::Success)
        }
        // ... rest
    }
}

fn set_from_key_path(config: &mut Config, key: &str, value: &str) -> Result<()> {
    let parts: Vec<&str> = key.split('.').collect();

    match parts.as_slice() {
        ["defaults", "sound"] => {
            config.defaults.sound = Some(value.to_string());
        }
        ["defaults", "timeout"] => {
            // Parse duration
            let duration = parse_duration(value)?;
            config.defaults.timeout = Some(duration);
        }
        ["defaults", "persistent"] => {
            config.defaults.persistent = value.parse()
                .map_err(|_| anyhow!("Invalid boolean value"))?;
        }
        _ => {
            return Err(anyhow!("Unknown config key: {}", key));
        }
    }

    Ok(())
}

fn unset_key_path(config: &mut Config, key: &str) -> Result<()> {
    let parts: Vec<&str> = key.split('.').collect();

    match parts.as_slice() {
        ["defaults", "sound"] => {
            config.defaults.sound = None;
        }
        ["defaults", "timeout"] => {
            config.defaults.timeout = None;
        }
        ["defaults", "persistent"] => {
            config.defaults.persistent = false;
        }
        _ => {
            return Err(anyhow!("Unknown config key: {}", key));
        }
    }

    Ok(())
}

fn set_from_json(config: &mut Config) -> Result<()> {
    use std::io::Read;

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let partial: serde_json::Value = serde_json::from_str(&buffer)?;

    // Merge JSON into config
    merge_json(config, &partial)?;

    Ok(())
}

fn merge_json(config: &mut Config, json: &serde_json::Value) -> Result<()> {
    if let Some(defaults) = json.get("defaults") {
        if let Some(sound) = defaults.get("sound") {
            config.defaults.sound = sound.as_str().map(String::from);
        }
        if let Some(timeout) = defaults.get("timeout") {
            if let Some(s) = timeout.as_str() {
                config.defaults.timeout = Some(parse_duration(s)?);
            }
        }
        if let Some(persistent) = defaults.get("persistent") {
            if let Some(b) = persistent.as_bool() {
                config.defaults.persistent = b;
            }
        }
    }

    Ok(())
}
```

#### CLI Args Update

```rust
// src/cli/args.rs
pub enum ConfigCommands {
    Set {
        /// Key path (e.g., defaults.sound) - not used if --json
        #[arg(required_unless_present = "json")]
        key: Option<String>,

        /// Value to set - not used if --json
        #[arg(required_unless_present = "json")]
        value: Option<String>,

        /// Read JSON from stdin
        #[arg(long)]
        json: bool,
    },
    // ... rest
}
```

---

## 6. Resource Pruning

### Current State
- `sound prune`: Placeholder at src/commands/sound.rs:37
- `icon prune`: Placeholder at src/commands/icon.rs:40

### Design

Smart cleanup with confirmation and dry-run support.

#### Command Structure
```bash
cb sound prune              # Both files and aliases
cb sound prune files        # Only orphaned files
cb sound prune aliases      # Only dangling aliases
cb sound prune --dry-run    # Preview without changes
cb sound prune --yes        # Skip confirmation

cb icon prune               # Both bundles and aliases
cb icon prune bundles       # Only orphaned bundles
cb icon prune aliases       # Only dangling aliases
cb icon prune --dry-run
cb icon prune --yes
```

#### Implementation

```rust
// src/commands/sound.rs

pub fn handle(action: &SoundCommands) -> Result<ExitCode> {
    match action {
        SoundCommands::Prune { target, dry_run, yes } => {
            let target = target.as_deref().unwrap_or("all");

            let (orphaned_files, dangling_aliases) = match target {
                "all" => (
                    find_orphaned_sound_files()?,
                    find_dangling_sound_aliases()?
                ),
                "files" => (find_orphaned_sound_files()?, vec![]),
                "aliases" => (vec![], find_dangling_sound_aliases()?),
                _ => return Err(anyhow!("Invalid target: {}", target)),
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
                use dialoguer::Confirm;
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
        // ... rest
    }
}

fn find_orphaned_sound_files() -> Result<Vec<PathBuf>> {
    let files_dir = sound_files_dir();
    let aliases = load_sound_aliases()?;

    // Get all files in cache
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
    let aliases = load_sound_aliases()?;
    let mut dangling = vec![];

    for (alias, path) in &aliases.aliases {
        if !Path::new(path).exists() {
            dangling.push(alias.clone());
        }
    }

    Ok(dangling)
}
```

#### CLI Args Update

```rust
// src/cli/args.rs
pub enum SoundCommands {
    Prune {
        /// Target: all (default), files, aliases
        target: Option<String>,

        /// Preview changes without executing
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    // ... rest
}

// Same for IconCommands
```

---

## Implementation Order

### Phase 1: Foundation (Days 1-2)
1. Icon bundle generation
2. Interactive notification response handling

### Phase 2: User Experience (Days 3-4)
3. Template create/update (all three modes)
4. Setup wizard

### Phase 3: Maintenance (Day 5)
5. Config set/unset (dot notation + JSON)
6. Resource pruning (sound + icon)

### Phase 4: Testing & Polish (Day 6)
- Comprehensive testing of all features
- Update documentation
- Update tests
- Create example workflows

---

## Testing Strategy

### Unit Tests
- Icon bundle generation (valid/invalid inputs)
- Template CRUD operations
- Config key path parsing
- Prune detection logic

### Integration Tests
- End-to-end notification flows
- Template rendering with new features
- Setup wizard flow
- Prune dry-run vs actual execution

### Manual Testing
- Interactive prompts (dialoguer UI)
- macOS notification center behavior
- Bundle generation on different image formats
- Sound/icon alias resolution

---

## Documentation Updates

### README.md
- Add examples for all new features
- Update CLI reference

### docs/plans/2026-01-21-claude-bell-design.md
- Mark TODOs as implemented
- Add architectural notes for bundle generation

### User Guide (new)
- Setup wizard walkthrough
- Template management guide
- Icon/sound best practices
- Troubleshooting guide

---

## Dependencies to Add

```toml
[dependencies]
dialoguer = "0.11"  # Interactive prompts
```

---

## Success Criteria

- [ ] All 8 TODOs removed from source code
- [ ] No warning messages in notification output
- [ ] `cb doctor` reports all systems operational
- [ ] Setup wizard completes successfully for new users
- [ ] Templates can be created/updated via all three input modes
- [ ] Interactive notifications return responses correctly
- [ ] Icon bundles display in notification center
- [ ] Prune commands clean up orphaned resources
- [ ] All tests pass
- [ ] Documentation updated
