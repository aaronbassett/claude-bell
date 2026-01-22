---
event: Stop
---

# Task Completion Hook

This hook notifies when Claude finishes working on a task.

## Trigger

Fires when Claude's Stop event occurs, indicating it has finished processing and is waiting for user input.

## Usage

When triggered, sends notification:
```bash
cb -t "Claude Code" \
   -m "Task complete - ready for next instruction" \
   --sound Glass
```

## Configuration

This hook can be customized via `.claude/hooks-completion.local.md`:

```yaml
---
# Custom title
title: "Done!"

# Custom message (supports template variables)
message: "Claude has finished working"

# Sound (system name or @alias)
sound: "Glass"

# Show notification only if task took longer than this
min_duration: "30s"

# Include task summary in message
include_summary: true
---
```

## Template Variables

Available variables for message customization:
- `{{ duration }}` - How long the task took
- `{{ tool_count }}` - Number of tools used
- `{{ file_count }}` - Number of files modified
