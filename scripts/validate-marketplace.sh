#!/bin/bash
# Validate .claude-plugin/marketplace.json

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MARKETPLACE_FILE="$PROJECT_ROOT/.claude-plugin/marketplace.json"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

errors=0

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

echo "========================================="
echo "Validating marketplace.json"
echo "========================================="

# Check file exists
if [[ ! -f "$MARKETPLACE_FILE" ]]; then
    log_error "marketplace.json not found at $MARKETPLACE_FILE"
    exit 1
fi
log_success "marketplace.json exists"

# Validate JSON syntax
if ! python3 -m json.tool "$MARKETPLACE_FILE" > /dev/null 2>&1; then
    log_error "marketplace.json is not valid JSON"
    exit 1
fi
log_success "Valid JSON syntax"

# Check required top-level fields
check_field() {
    local field="$1"
    local value
    value=$(python3 -c "import json; data=json.load(open('$MARKETPLACE_FILE')); print(data.get('$field', ''))" 2>/dev/null)
    if [[ -z "$value" ]]; then
        log_error "Missing required field: $field"
        return 1
    fi
    log_success "Has required field: $field"
    return 0
}

check_field "name" || true
check_field "owner" || true
check_field "metadata" || true
check_field "plugins" || true

# Check owner has name
owner_name=$(python3 -c "import json; data=json.load(open('$MARKETPLACE_FILE')); print(data.get('owner', {}).get('name', ''))" 2>/dev/null)
if [[ -z "$owner_name" ]]; then
    log_error "Missing required field: owner.name"
else
    log_success "Has required field: owner.name"
fi

# Check metadata has description
metadata_desc=$(python3 -c "import json; data=json.load(open('$MARKETPLACE_FILE')); print(data.get('metadata', {}).get('description', ''))" 2>/dev/null)
if [[ -z "$metadata_desc" ]]; then
    log_error "Missing required field: metadata.description"
else
    log_success "Has required field: metadata.description"
fi

# Verify plugin sources exist
echo ""
echo "Checking plugin sources..."
plugins=$(python3 -c "
import json
data = json.load(open('$MARKETPLACE_FILE'))
for plugin in data.get('plugins', []):
    print(plugin.get('name', '') + '|' + plugin.get('source', ''))
" 2>/dev/null)

while IFS='|' read -r name source; do
    if [[ -z "$name" || -z "$source" ]]; then
        continue
    fi

    # Resolve relative path
    full_path="$PROJECT_ROOT/$source"

    if [[ ! -d "$full_path" ]]; then
        log_error "Plugin '$name' source directory not found: $source"
    else
        # Check for plugin.json in the source
        if [[ ! -f "$full_path/plugin.json" ]]; then
            log_error "Plugin '$name' missing plugin.json in $source"
        else
            log_success "Plugin '$name' source verified: $source"
        fi
    fi
done <<< "$plugins"

echo ""
echo "========================================="
if [[ $errors -gt 0 ]]; then
    echo -e "${RED}Validation failed with $errors error(s)${NC}"
    exit 1
else
    echo -e "${GREEN}Validation passed${NC}"
    exit 0
fi
