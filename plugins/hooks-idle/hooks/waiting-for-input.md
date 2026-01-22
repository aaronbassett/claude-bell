---
event: Stop
---

# Idle Notification Hook

This hook notifies when Claude is waiting for user input.

## Detection

Triggers when:
- Claude stops and requires user input to continue
- A question was asked that needs an answer
- Clarification is needed before proceeding

## Usage

When detected, sends notification:
```bash
cb -t "Claude Waiting" \
   -m "Input needed to continue" \
   --sound Ping
```

## Configuration

This hook can be customized via `.claude/hooks-idle.local.md`:

```yaml
---
# Delay before showing notification
delay: "5s"

# Only notify if idle for longer than this
min_idle: "10s"

# Sound for notification
sound: "Ping"

# Custom message
message: "Claude is waiting for your response"

# Include the question in the notification
include_question: true

# Maximum question length in notification
max_question_length: 100
---
```

## Template Variables

Available variables:
- `{{ idle_time }}` - How long Claude has been waiting
- `{{ question }}` - The question asked (if any)
- `{{ context }}` - Brief context about what was being done

## Differentiating from Completion

This hook differs from the completion hook:
- **Completion**: Task is done, no further action needed
- **Idle**: Claude is blocked and needs user input

The idle hook typically fires when Claude asks a question and waits for an answer.
