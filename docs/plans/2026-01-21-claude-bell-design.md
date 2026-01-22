# Claude Bell Design Document

**Date:** 2026-01-21
**Status:** Draft
**Author:** Aaron Bassett (with Claude)

## Overview

Claude Bell is a macOS notification CLI tool optimized for Claude Code. It enables fire-and-forget alerts, blocking confirmations with action buttons, and text input collection — all designed to communicate with users outside the terminal.

This is a Rust reimplementation inspired by [NotifiCLI](https://github.com/saihgupr/NotifiCLI), with significant enhancements for Claude Code integration.

## Goals

- **Claude Code optimized** — Designed for use via Bash tool and hooks
- **Robust error handling** — Clear exit codes, structured errors, diagnostic tools
- **Template system** — Reusable notification configurations with Tera templating
- **Alias management** — Sound and icon aliases with CRUD operations
- **Flexible I/O** — Stdin JSON input, configurable output formats
- **Plugin ecosystem** — Core binary + optional hook configurations

## Non-Goals

- Cross-platform support (macOS only)
- GUI configuration interface
- Keyboard Maestro integration

---

## Architecture

### Binary Structure

```
claude-bell (single binary, aliased as 'cb')
├── Main notification logic (macOS UserNotifications via objc bindings)
├── Template engine (Tera)
├── Config manager
├── Alias managers (sounds, icons)
├── Doctor/health checks
└── CLI parser (clap with derive)
```

### Data Directory

```
~/.claude-bell/
├── config.json              # Global defaults
├── templates/               # Notification templates
│   └── *.json
├── sounds/
│   ├── aliases.json         # Sound alias mappings
│   └── files/               # Cached sound files (content-hashed)
├── icons/
│   ├── aliases.json         # Icon alias mappings
│   └── bundles/             # Generated app bundles (self-contained)
├── apps/                    # Persistent notification app bundle
│   └── ClaudeBellPersistent.app
└── state.json               # Internal state
```

---

## CLI Interface

### Commands

```
cb [OPTIONS]                    # Send notification
cb template <SUBCOMMAND>        # Manage templates
cb sound <SUBCOMMAND>           # Manage sound aliases
cb icon <SUBCOMMAND>            # Manage icon aliases/bundles
cb config <SUBCOMMAND>          # Manage global config
cb setup                        # First-run setup wizard
cb doctor                       # Comprehensive health check
```

### Notification Flags

#### Content
| Flag | Description |
|------|-------------|
| `-t, --title <TEXT>` | Notification title (required unless template) |
| `-s, --subtitle <TEXT>` | Secondary text line |
| `-m, --message <TEXT>` | Body text |
| `-i, --image <PATH\|URL>` | Thumbnail image |
| `--icon <PATH\|@alias>` | App icon (app path, image path, or @alias) |
| `--sound <PATH\|@alias>` | Sound (system name, file path, or @alias) |

#### Interaction
| Flag | Description |
|------|-------------|
| `-a, --actions <A,B,C>` | Comma-separated action buttons |
| `-r, --reply <PLACEHOLDER>` | Enable reply text input |
| `--url <URL>` | Open URL when clicked |

#### Behavior
| Flag | Description |
|------|-------------|
| `--persistent` | Stay on screen until dismissed |
| `--not-persistent` | Override implicit persistence |
| `--timeout <DURATION>` | Wait duration (30s, 5m) |
| `--default <VALUE>` | Return value on dismiss/timeout |
| `--on-dismiss <VALUE>` | Return value on dismiss (overrides --default) |
| `--on-timeout <VALUE>` | Return value on timeout (overrides --default) |

#### Input
| Flag | Description |
|------|-------------|
| `--batch` | Process newline-delimited JSON from stdin |

#### Output
| Flag | Description |
|------|-------------|
| `--json [TARGETS]` | JSON output (stdout, stderr, logs, response) |
| `--pretty` | Format and colorize JSON |
| `--quiet` | Suppress stdout |
| `--silent` | Suppress all output |
| `--log-level <LEVEL>` | error, warn, info, debug, trace |

#### Templates
| Flag | Description |
|------|-------------|
| `--template <NAME>` | Use named template |
| `--var <KEY:VALUE,...>` | Template variables (repeatable) |

### Subcommands

#### Templates
```bash
cb template list                              # List all templates
cb template show <NAME>                       # Show template source
cb template show <NAME> --render --var 'k:v'  # Show rendered output
cb template create                            # Interactive creation
cb template create --name <N> --title <T> ... # Non-interactive
cb template update <NAME> --title "New"       # Update fields
cb template delete <NAME>                     # Delete template
cb template validate <NAME>                   # Validate template
cb template validate --all                    # Validate all
```

#### Sounds
```bash
cb sound list                        # List aliases
cb sound add <ALIAS> <PATH>          # Add alias (reference)
cb sound add <ALIAS> <PATH> --cache  # Add alias (copy file, dedupe by hash)
cb sound remove <ALIAS>              # Remove alias only
cb sound remove <ALIAS> --with-file  # Remove alias + cached file (warns if shared)
cb sound prune                       # Fix orphaned files and dangling aliases
cb sound prune files                 # Remove cached files without aliases
cb sound prune aliases               # Remove aliases to missing files
cb sound doctor                      # Check for issues
```

#### Icons
```bash
cb icon list                         # List aliases
cb icon add <ALIAS> <PATH>           # Add alias (app or image path)
cb icon remove <ALIAS>               # Remove alias only
cb icon remove <ALIAS> --with-bundle # Remove alias + bundle (warns if shared)
cb icon prune                        # Fix orphaned bundles and dangling aliases
cb icon prune bundles                # Remove bundles without aliases
cb icon prune aliases                # Remove aliases to missing bundles
cb icon doctor                       # Check for issues
```

#### Config
```bash
cb config show                       # Display current config
cb config show --pretty              # Formatted with colors
cb config set <KEY> <VALUE>          # Set a value
cb config unset <KEY>                # Remove value (use default)
cb config reset                      # Reset to defaults
cb config validate                   # Validate ~/.claude-bell/config.json
cb config validate --path <FILE>     # Validate specific file
echo '{...}' | cb config validate    # Validate from stdin
```

---

## Input/Output

### Input Sources (Precedence)

1. Built-in defaults
2. Config file (`~/.claude-bell/config.json`)
3. Template values
4. Stdin JSON
5. Command-line flags
6. `--no-<option>` explicitly disables

### Stdin JSON Schema

```json
{
  "title": "Deploy Complete",
  "subtitle": "Production",
  "message": "Version 2.1.0 is live",
  "image": "/path/to/image.png",
  "icon": "@claude",
  "sound": "Glass",
  "actions": ["View Logs", "Open Dashboard"],
  "reply": "Add release notes",
  "url": "https://example.com/deploy/123",
  "persistent": true,
  "timeout": "5m",
  "default": "dismissed",
  "on_dismiss": "dismissed",
  "on_timeout": "timeout",
  "template": {
    "name": "deploy",
    "vars": {
      "version": "2.1.0",
      "env": "production"
    }
  }
}
```

### Batch Mode

```bash
echo '{"title": "Step 1", "message": "Starting"}
{"title": "Step 2", "message": "Processing"}
{"title": "Step 3", "message": "Done"}' | cb --batch
```

### Output Destinations

| Stream | Content |
|--------|---------|
| stdout | Response (action clicked, reply text, "dismissed", "timeout") |
| stderr | Logs and errors |

### Verbosity Flags

| Flag | stdout | stderr |
|------|--------|--------|
| (default) | response | logs + errors |
| `--quiet` | suppressed | logs + errors |
| `--silent` | suppressed | suppressed |

### Exit Codes

| Code | Meaning | Example |
|------|---------|---------|
| 0 | Success | Action clicked, reply received, fire-and-forget sent |
| 1 | Timeout | `--timeout` elapsed without response |
| 2 | Dismissed | User dismissed without selecting action |
| 3 | User error | Invalid args, template not found, malformed JSON |
| 4 | System error | Missing permissions, notification service unavailable |
| 5 | Application error | Bug in claude-bell |

### JSON Output Control

```bash
cb --json                    # JSON for all output
cb --json stdout             # JSON for response only
cb --json stderr             # JSON for logs + errors
cb --json logs               # JSON for logs only (errors plain)
cb --json response           # Same as stdout
cb --json response,logs      # JSON for response and logs
```

### Response Formats

**Plain text (default):**
```
Yes
```

**JSON (`--json`):**
```json
{"action":"Yes","type":"action","notification_id":"abc123","timestamp":"2026-01-21T14:30:00Z"}
```

**Response types:**
| type | Meaning | Fields |
|------|---------|--------|
| `action` | User clicked action button | `action` |
| `reply` | User submitted reply | `reply` |
| `default` | User clicked notification body | `url` (if set) |
| `dismissed` | User dismissed | — |
| `timeout` | Timeout elapsed | — |

---

## Templates

### Template File Structure

`~/.claude-bell/templates/<name>.json`:

```json
{
  "name": "build-result",
  "description": "Notify when build finishes",
  "title": "Build {{ status | title }}",
  "message": "{% if duration %}Completed in {{ duration }}{% else %}Completed{% endif %}",
  "sound": "{% if status == 'failed' %}@error{% else %}@success{% endif %}",
  "icon": "@terminal",
  "defaults": {
    "status": "complete"
  }
}
```

### Tera Capabilities

- Variables: `{{ var }}`
- Filters: `{{ name | upper }}`, `{{ name | title }}`
- Conditionals: `{% if x %}...{% elif y %}...{% else %}...{% endif %}`
- Default values: `{{ var | default(value="fallback") }}`
- String concatenation: `{{ a ~ b }}`

---

## Config

### Config File Structure

`~/.claude-bell/config.json`:

```json
{
  "version": 1,
  "defaults": {
    "sound": "Glass",
    "icon": "@claude",
    "json": false,
    "log_level": "warn",
    "persistent": false
  },
  "json_targets": ["stdout"],
  "timeout": null,
  "on_dismiss": null,
  "on_timeout": null
}
```

---

## Bundled Assets

### Icons

| Alias | Description |
|-------|-------------|
| `@claude` | Claude logo |
| `@terminal` | Terminal icon |
| `@success` | Green checkmark |
| `@warning` | Yellow warning |
| `@error` | Red error |

### System Sounds

Basso, Blow, Bottle, Frog, Funk, Glass, Hero, Morse, Ping, Pop, Purr, Sosumi, Submarine, Tink

---

## Persistent Notifications

### Approach

1. Research cleaner macOS APIs for persistence first
2. Fall back to separate app bundle if needed (`~/.claude-bell/apps/ClaudeBellPersistent.app`)
3. `cb setup` guides user through System Settings configuration

### Implicit Persistence

- `--actions` and `--reply` implicitly enable persistent mode
- `--not-persistent` overrides this behavior

---

## Distribution

### Channels

| Channel | Command |
|---------|---------|
| GitHub Releases | Download pre-built binaries |
| Homebrew | `brew install aaronbassett/tap/claude-bell` |
| Cargo | `cargo install claude-bell` |
| Claude Code | `/plugin marketplace add aaronbassett/claude-bell` |

### Claude Code Plugins

**Marketplace:** `aaronbassett/claude-bell`

**Plugin configurations:**

| Name | Contents |
|------|----------|
| `core` | Binary + skill |
| `hooks-dangerous` | Confirmation before destructive operations |
| `hooks-completion` | Notify when Claude finishes |
| `hooks-errors` | Notify on command/build failures |
| `hooks-long-running` | Notify when operations exceed threshold |
| `hooks-idle` | Notify when waiting for user input |
| `hooks-session` | Notify on session start/end |

**Installation:**
```bash
/plugin marketplace add aaronbassett/claude-bell
/plugin install core@claude-bell
/plugin install hooks-dangerous@claude-bell  # Optional
```

---

## Repository Structure

```
claude-bell/
├── .claude-plugin/
│   └── marketplace.json
├── plugins/
│   ├── core/
│   │   ├── skills/
│   │   │   └── claude-bell/
│   │   │       ├── SKILL.md
│   │   │       └── references/
│   │   │           ├── cli.md
│   │   │           ├── examples.md
│   │   │           ├── setup.md
│   │   │           └── troubleshooting.md
│   │   └── bin/
│   │       ├── macos-arm64/cb
│   │       └── macos-x64/cb
│   ├── hooks-dangerous/
│   │   └── hooks/
│   │       └── PreToolUse.md
│   ├── hooks-completion/
│   │   └── hooks/
│   │       └── Stop.md
│   ├── hooks-errors/
│   │   └── hooks/
│   │       └── PostToolUse.md
│   ├── hooks-long-running/
│   │   └── hooks/
│   │       └── PostToolUse.md
│   ├── hooks-idle/
│   │   └── hooks/
│   │       └── Notification.md
│   └── hooks-session/
│       └── hooks/
│           ├── SessionStart.md
│           └── SessionEnd.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli/
│   ├── notification/
│   ├── template/
│   ├── config/
│   ├── alias/
│   └── doctor/
├── assets/
│   ├── icons/
│   └── sounds/
├── tests/
│   ├── integration/
│   ├── fixtures/
│   └── common/
├── scripts/
│   ├── validate-marketplace.sh
│   ├── validate-skills.sh
│   └── validate-hooks.sh
├── homebrew/
│   └── claude-bell.rb
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── Cargo.toml
├── CONSTITUTION.md
└── README.md
```

---

## Testing Strategy

### Philosophy (per Constitution)

- Test critical paths, not coverage metrics
- Integration tests over unit tests for CLI behavior
- Skip testing trivial code the compiler validates

### Critical Paths

| Area | Test Cases |
|------|------------|
| CLI parsing | Flag combinations, stdin detection, `--var` syntax, `--json` targets |
| Input precedence | Config < template < stdin < flags < `--no-*` |
| Template rendering | Tera syntax, variable substitution, conditionals, defaults |
| Exit codes | 0-5 for all scenarios |
| JSON output | Compact default, `--pretty` formatting, selective targets |
| Alias resolution | `@alias` lookup, missing alias errors, prune logic |
| Config validation | Schema validation, type checking, unknown keys |
| Doctor checks | Each health check returns correct status |

### What NOT to Test

- macOS notification delivery (requires GUI interaction)
- Sound playback
- Icon bundle generation internals
- Trivial clap derive parsing

### Mocking Strategy

- Mock the notification system boundary
- Test everything up to "would send notification with these params"
- Integration tests verify params are correctly assembled

---

## CI/CD

### CI Workflow (on PR/push to main)

1. Check formatting (`cargo fmt --check`)
2. Clippy with warnings as errors
3. Run tests
4. Build release
5. Validate marketplace.json schema
6. Validate skill frontmatter
7. Validate hook frontmatter

### Release Workflow (on tag push)

1. Run full CI checks
2. Build binaries (macos-arm64, macos-x64)
3. Create GitHub release with binaries and checksums
4. Copy binaries to plugin directories
5. Update version in marketplace.json
6. Commit and push updated marketplace
7. Publish to crates.io
8. Update aaronbassett/homebrew-tap formula

---

## Tech Stack

| Component | Choice |
|-----------|--------|
| Language | Rust |
| CLI parsing | clap (derive) |
| Template engine | tera |
| JSON output | colored_json (with `--pretty`) |
| macOS notifications | objc bindings to UserNotifications |
| Error handling | thiserror / anyhow |
| CI/CD | GitHub Actions |

---

## Implementation Order

1. **Scaffold project** — Cargo.toml, directory structure, basic clap setup
2. **Core notification** — macOS bindings, basic send functionality
3. **Exit codes** — Response handling, timeout, dismiss
4. **Config system** — Load, validate, precedence
5. **Template system** — CRUD, Tera rendering
6. **Alias systems** — Sound and icon management
7. **Doctor/health** — Validation, diagnostics
8. **Plugin packaging** — Marketplace, skill, hooks
9. **CI/CD** — Workflows, validation scripts
10. **Homebrew formula** — Tap setup

---

## Open Questions

1. **Persistent notifications** — Is there a cleaner macOS API than separate app bundles?
2. **Icon caching** — How long does macOS cache app icons? Document workarounds.
3. **Hook environment variables** — What variables are available in Claude Code hooks?

---

## Appendix: Marketplace File

`.claude-plugin/marketplace.json`:

```json
{
  "name": "claude-bell",
  "owner": {
    "name": "Aaron Bassett"
  },
  "metadata": {
    "description": "macOS notifications for Claude Code",
    "version": "0.1.0",
    "pluginRoot": "./plugins"
  },
  "plugins": [
    {
      "name": "core",
      "source": "./plugins/core",
      "description": "CLI and skill for sending macOS notifications",
      "version": "0.1.0",
      "license": "MIT",
      "keywords": ["notifications", "macos", "alerts"],
      "strict": false
    },
    {
      "name": "hooks-dangerous",
      "source": "./plugins/hooks-dangerous",
      "description": "Prompt for confirmation before destructive operations",
      "version": "0.1.0",
      "strict": false
    },
    {
      "name": "hooks-completion",
      "source": "./plugins/hooks-completion",
      "description": "Notify when Claude finishes working",
      "version": "0.1.0",
      "strict": false
    },
    {
      "name": "hooks-errors",
      "source": "./plugins/hooks-errors",
      "description": "Notify on command failures and build errors",
      "version": "0.1.0",
      "strict": false
    },
    {
      "name": "hooks-long-running",
      "source": "./plugins/hooks-long-running",
      "description": "Notify when operations exceed time threshold",
      "version": "0.1.0",
      "strict": false
    },
    {
      "name": "hooks-idle",
      "source": "./plugins/hooks-idle",
      "description": "Notify when Claude is waiting for user input",
      "version": "0.1.0",
      "strict": false
    },
    {
      "name": "hooks-session",
      "source": "./plugins/hooks-session",
      "description": "Notify on session start and end",
      "version": "0.1.0",
      "strict": false
    }
  ]
}
```

---

## Appendix: Skill File

`plugins/core/skills/claude-bell/SKILL.md`:

```markdown
---
name: claude-bell
description: Use when you need to alert the user outside the terminal, get confirmation before proceeding, or collect text input. Triggers include task completion, errors requiring attention, destructive operations needing approval, or questions requiring user decision.
---

# Claude Bell

Send macOS notifications to communicate with users outside the terminal.

## When to Use

**Use for:**
- Task completion (build done, tests passed, deploy finished)
- Errors requiring attention
- Confirmation before destructive operations
- Collecting text input (release notes, commit messages)
- Status updates during long operations

**Don't use for:**
- Routine progress updates (use terminal)
- Information user is actively watching
- Rapid-fire notifications (spam)

## Quick Reference

| Pattern | Command |
|---------|---------|
| Alert | `cb -t "Done" -m "Build complete"` |
| Alert with sound | `cb -t "Done" -m "Build complete" --sound Glass` |
| Yes/No | `cb -t "Deploy?" -a "Yes,No" --default "No"` |
| Multiple choice | `cb -t "Action" -a "A,B,C,Cancel" --default "Cancel"` |
| Text input | `cb -t "Notes" -r "Enter notes..." --timeout 5m` |
| Template | `cb --template build-done --var 'project:myapp'` |

## Exit Codes

| Code | Meaning | Action |
|------|---------|--------|
| 0 | Success | Proceed with returned value |
| 1 | Timeout | Use `--default` value |
| 2 | Dismissed | Use `--default` value |
| 3 | User error | Check arguments |
| 4 | System error | Check permissions (`cb doctor`) |
| 5 | App error | Report bug |

## Critical Rules

1. **Always provide `--default`** for interactive notifications
2. **Always check exit code** — don't assume success
3. **Use `--timeout`** for non-critical prompts
4. **Don't spam** — one notification per logical event

## References

- `references/cli.md` — Full CLI documentation
- `references/examples.md` — Workflow patterns
- `references/setup.md` — Installation & setup (read only when user requests setup)
- `references/troubleshooting.md` — Diagnosing issues
```
