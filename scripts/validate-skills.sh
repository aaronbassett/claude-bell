#!/bin/bash
# Validate skill SKILL.md files in plugins/*/skills/*/SKILL.md

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

# Extract YAML frontmatter and check for required fields
validate_skill() {
    local file="$1"
    local relative_path="${file#$PROJECT_ROOT/}"

    ((checked++))

    # Check file exists (should always be true since we found it)
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

    # Check for required 'name' field
    if ! echo "$frontmatter" | grep -qE '^name:\s*.+'; then
        log_error "$relative_path: Missing required field 'name' in frontmatter"
        return 1
    fi

    # Check for required 'description' field
    if ! echo "$frontmatter" | grep -qE '^description:\s*.+'; then
        log_error "$relative_path: Missing required field 'description' in frontmatter"
        return 1
    fi

    # Extract the skill directory to check referenced files
    skill_dir=$(dirname "$file")

    # Check for references directory if mentioned in file
    if grep -q "references/" "$file"; then
        refs_dir="$skill_dir/references"
        if [[ ! -d "$refs_dir" ]]; then
            log_error "$relative_path: References directory mentioned but not found"
            return 1
        fi

        # Extract and verify referenced files
        referenced_files=$(grep -oE 'references/[a-zA-Z0-9_-]+\.md' "$file" | sort -u)
        for ref in $referenced_files; do
            ref_path="$skill_dir/$ref"
            if [[ ! -f "$ref_path" ]]; then
                log_error "$relative_path: Referenced file not found: $ref"
                return 1
            fi
        done
    fi

    log_success "$relative_path"
    return 0
}

echo "========================================="
echo "Validating skill files"
echo "========================================="

# Find all SKILL.md files
skill_files=$(find "$PLUGINS_DIR" -path "*/skills/*/SKILL.md" -type f 2>/dev/null || true)

if [[ -z "$skill_files" ]]; then
    log_info "No skill files found in $PLUGINS_DIR"
    echo ""
    echo "========================================="
    echo -e "${GREEN}Validation passed (no skills to validate)${NC}"
    exit 0
fi

while IFS= read -r file; do
    validate_skill "$file" || true
done <<< "$skill_files"

echo ""
echo "========================================="
echo "Checked $checked skill file(s)"
if [[ $errors -gt 0 ]]; then
    echo -e "${RED}Validation failed with $errors error(s)${NC}"
    exit 1
else
    echo -e "${GREEN}Validation passed${NC}"
    exit 0
fi
