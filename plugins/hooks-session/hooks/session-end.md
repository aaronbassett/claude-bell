---
event: SessionEnd
---

# Session End Hook

This hook notifies when a Claude Code session ends.

## Usage

When triggered, sends notification:
```bash
cb -t "Claude Code" \
   -m "Session ended after $DURATION" \
   --sound Purr
```

## Configuration

This hook can be customized via `.claude/hooks-session.local.md`:

```yaml
---
on_end:
  enabled: true
  title: "Claude Code"
  message: "Session ended after {{ duration }}"
  sound: "Purr"
  include_summary: true
---
```

## Template Variables

Available variables:
- `{{ duration }}` - Session duration
- `{{ messages }}` - Number of messages exchanged
- `{{ tools_used }}` - Number of tool invocations
- `{{ files_changed }}` - Number of files modified

## Summary Information

The session end notification can include a summary:
- Duration of session
- Number of exchanges
- Files modified
- Tools used
