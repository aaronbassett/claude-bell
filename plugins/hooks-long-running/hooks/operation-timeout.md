---
event: PostToolUse
tools: ["Bash", "Task"]
---

# Long-Running Operation Hook

This hook notifies when operations exceed a time threshold.

## Detection

Triggers when tool execution time exceeds configured threshold (default: 60 seconds).

## Usage

When detected, sends notification:
```bash
cb -t "Long Operation Complete" \
   -m "$TOOL_NAME finished after $DURATION" \
   --sound Submarine
```

## Configuration

This hook can be customized via `.claude/hooks-long-running.local.md`:

```yaml
---
# Minimum duration to trigger notification
threshold: "60s"

# Different thresholds per tool
tool_thresholds:
  Bash: "30s"
  Task: "2m"

# Sound for notification
sound: "Submarine"

# Show progress notification at intervals
progress_interval: "30s"

# Commands to always notify regardless of duration
always_notify:
  - "npm install"
  - "cargo build"
  - "docker build"
---
```

## Template Variables

Available variables:
- `{{ tool }}` - Tool name (Bash, Task, etc.)
- `{{ command }}` - Command that was run
- `{{ duration }}` - How long it took
- `{{ threshold }}` - Configured threshold
