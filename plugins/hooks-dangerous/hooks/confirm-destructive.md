---
event: PreToolUse
tools: ["Bash"]
---

# Dangerous Operation Confirmation Hook

This hook prompts for confirmation before destructive bash operations.

## Detection Patterns

Detects commands matching:
- `rm -rf`
- `git reset --hard`
- `git push --force`
- `git push -f`
- `DROP TABLE`
- `DELETE FROM` without WHERE
- `TRUNCATE TABLE`
- `git clean -fd`
- `docker system prune`

## Usage

When detected, sends notification:
```bash
cb -t "Confirm Destructive Operation" \
   -m "Command: $COMMAND" \
   -a "Proceed,Cancel" \
   --default "Cancel" \
   --timeout 5m
```

## Response Handling

- **Proceed**: Allow the command to execute
- **Cancel**: Block the command and inform Claude
- **Timeout**: Treat as Cancel (safe default)

## Configuration

This hook can be customized via `.claude/hooks-dangerous.local.md`:

```yaml
---
# Additional patterns to detect
extra_patterns:
  - "DROP DATABASE"
  - "format c:"

# Patterns to exclude from checks
exclude_patterns:
  - "rm -rf node_modules"  # Common safe operation

# Timeout for confirmation
timeout: "5m"

# Sound for notification
sound: "Sosumi"
---
```
