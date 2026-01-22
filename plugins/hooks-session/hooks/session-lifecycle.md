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
# Notification on session start
on_start:
  enabled: true
  title: "Claude Code"
  message: "Session started in {{ project }}"
  sound: "Pop"

# Notification on session end
on_end:
  enabled: true
  title: "Claude Code"
  message: "Session ended after {{ duration }}"
  sound: "Purr"
  include_summary: true
---
```

## Template Variables

Available variables for start:
- `{{ project }}` - Project directory name
- `{{ path }}` - Full project path
- `{{ time }}` - Session start time

Available variables for end:
- `{{ duration }}` - Session duration
- `{{ messages }}` - Number of messages exchanged
- `{{ tools_used }}` - Number of tool invocations
- `{{ files_changed }}` - Number of files modified

---

# Session End Hook

```yaml
---
event: SessionEnd
---
```

This hook notifies when a Claude Code session ends.

## Usage

When triggered, sends notification:
```bash
cb -t "Claude Code" \
   -m "Session ended after $DURATION" \
   --sound Purr
```

## Summary Information

The session end notification can include a summary:
- Duration of session
- Number of exchanges
- Files modified
- Tools used
