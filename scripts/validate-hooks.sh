#!/bin/bash
# Validate hook markdown files in plugins/*/hooks/*.md

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PLUGINS_DIR="$PROJECT_ROOT/plugins"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

errors=0
checked=0

# Valid Claude Code hook event types
VALID_EVENTS=(
    "PreToolUse"
    "PostToolUse"
    "Stop"
    "SubagentStop"
    "SessionStart"
    "SessionEnd"
    "UserPromptSubmit"
    "PreCompact"
    "Notification"
)

log_error() {
    echo -e "${RED}ERROR:${NC} $1"
    ((errors++))
}

log_success() {
    echo -e "${GREEN}OK:${NC} $1"
}

log_info() {
    echo -e "${YELLOW}INFO:${NC} $1"
}

is_valid_event() {
    local event="$1"
    for valid in "${VALID_EVENTS[@]}"; do
        if [[ "$event" == "$valid" ]]; then
            return 0
        fi
    done
    return 1
}

# Extract YAML frontmatter and check for required fields
validate_hook() {
    local file="$1"
    local relative_path="${file#$PROJECT_ROOT/}"

    ((checked++))

    # Check file exists
    if [[ ! -f "$file" ]]; then
        log_error "$relative_path: File not found"
        return 1
    fi

    # Check for YAML frontmatter delimiters
    first_line=$(head -n 1 "$file")
    if [[ "$first_line" != "---" ]]; then
        log_error "$relative_path: Missing YAML frontmatter (no opening ---)"
        return 1
    fi

    # Extract frontmatter content (between first two ---)
    # Find line number of second --- (closing delimiter)
    closing_line=$(awk '/^---$/ { count++; if (count == 2) { print NR; exit } }' "$file")
    if [[ -z "$closing_line" || "$closing_line" -le 1 ]]; then
        log_error "$relative_path: Missing closing --- in frontmatter"
        return 1
    fi
    # Extract lines between first and second ---
    frontmatter=$(sed -n "2,$((closing_line - 1))p" "$file")

    if [[ -z "$frontmatter" ]]; then
        log_error "$relative_path: Empty or invalid frontmatter"
        return 1
    fi

    # Check for required 'event' field
    event_line=$(echo "$frontmatter" | grep -E '^event:\s*' || true)
    if [[ -z "$event_line" ]]; then
        log_error "$relative_path: Missing required field 'event' in frontmatter"
        return 1
    fi

    # Extract the event value (handle with or without quotes)
    event_value=$(echo "$event_line" | sed 's/^event:\s*//' | tr -d '"' | tr -d "'" | xargs)

    if [[ -z "$event_value" ]]; then
        log_error "$relative_path: 'event' field is empty"
        return 1
    fi

    # Validate event type
    if ! is_valid_event "$event_value"; then
        log_error "$relative_path: Invalid event type '$event_value'. Valid types: ${VALID_EVENTS[*]}"
        return 1
    fi

    # For PreToolUse and PostToolUse, 'tools' is optional but if present should be valid
    if [[ "$event_value" == "PreToolUse" || "$event_value" == "PostToolUse" ]]; then
        tools_line=$(echo "$frontmatter" | grep -E '^tools:\s*' || true)
        if [[ -n "$tools_line" ]]; then
            # Check it's a valid YAML array format
            if ! echo "$tools_line" | grep -qE '^\s*tools:\s*\[.*\]'; then
                log_error "$relative_path: 'tools' field should be a YAML array (e.g., tools: [\"Bash\", \"Edit\"])"
                return 1
            fi
        fi
    fi

    log_success "$relative_path (event: $event_value)"
    return 0
}

echo "========================================="
echo "Validating hook files"
echo "========================================="
echo "Valid event types: ${VALID_EVENTS[*]}"
echo ""

# Find all hook markdown files
hook_files=$(find "$PLUGINS_DIR" -path "*/hooks/*.md" -type f 2>/dev/null || true)

if [[ -z "$hook_files" ]]; then
    log_info "No hook files found in $PLUGINS_DIR"
    echo ""
    echo "========================================="
    echo -e "${GREEN}Validation passed (no hooks to validate)${NC}"
    exit 0
fi

while IFS= read -r file; do
    validate_hook "$file" || true
done <<< "$hook_files"

echo ""
echo "========================================="
echo "Checked $checked hook file(s)"
if [[ $errors -gt 0 ]]; then
    echo -e "${RED}Validation failed with $errors error(s)${NC}"
    exit 1
else
    echo -e "${GREEN}Validation passed${NC}"
    exit 0
fi
