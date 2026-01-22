---
event: SessionStart
---

# Session Start Hook

This hook notifies when a new Claude Code session starts.

## Usage

When triggered, sends notification:
```bash
cb -t "Claude Code" \
   -m "Session started in $PROJECT" \
   --sound Pop
```

## Configuration

This hook can be customized via `.claude/hooks-session.local.md`:

```yaml
---
on_start:
  enabled: true
  title: "Claude Code"
  message: "Session started in {{ project }}"
  sound: "Pop"
---
```

## Template Variables

Available variables:
- `{{ project }}` - Project directory name
- `{{ path }}` - Full project path
- `{{ time }}` - Session start time
