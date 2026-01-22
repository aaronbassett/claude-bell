---
event: PostToolUse
tools: ["Bash"]
---

# Command Failure Hook

This hook notifies when a bash command fails.

## Detection

Triggers when:
- Bash command exits with non-zero status
- Command output contains error patterns

## Error Patterns

Detected patterns include:
- Exit code > 0
- "error:" in output
- "Error:" in output
- "FAILED" in output
- "fatal:" in output (git)
- Build/compile failures

## Usage

When detected, sends notification:
```bash
cb -t "Command Failed" \
   -s "$TOOL_NAME" \
   -m "$ERROR_SUMMARY" \
   --sound Basso
```

## Configuration

This hook can be customized via `.claude/hooks-errors.local.md`:

```yaml
---
# Minimum exit code to trigger notification
min_exit_code: 1

# Additional error patterns
extra_patterns:
  - "ENOENT"
  - "permission denied"

# Ignore certain error patterns
ignore_patterns:
  - "npm WARN"
  - "DeprecationWarning"

# Sound for error notification
sound: "Basso"

# Only notify for these commands
commands:
  - "npm"
  - "cargo"
  - "make"
---
```

## Template Variables

Available variables:
- `{{ command }}` - The failed command
- `{{ exit_code }}` - Exit code
- `{{ error_line }}` - First error line
- `{{ stderr }}` - Full stderr output
