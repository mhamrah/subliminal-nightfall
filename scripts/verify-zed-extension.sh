#!/bin/bash

# Verifies that zed/ is a self-contained Zed extension.

set -e

echo "Verifying Subliminal Nightfall Zed extension layout..."
echo ""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

ALL_CHECKS_PASSED=true

check_file() {
    local path="$1"
    local description="$2"

    if [ -f "$path" ]; then
        echo -e "  ${GREEN}OK${NC} $description: $path"
    else
        echo -e "  ${RED}FAIL${NC} Missing $description: $path"
        ALL_CHECKS_PASSED=false
    fi
}

check_absent() {
    local path="$1"
    local description="$2"

    if [ -e "$path" ] || [ -L "$path" ]; then
        echo -e "  ${RED}FAIL${NC} $description should not exist: $path"
        ALL_CHECKS_PASSED=false
    else
        echo -e "  ${GREEN}OK${NC} $description is absent"
    fi
}

check_file "zed/extension.toml" "Zed extension manifest"
check_file "zed/extension/icon.png" "Zed extension icon"
check_file "zed/themes/subliminal-nightfall.json" "Zed theme JSON"
check_absent "extension.toml" "Root Zed manifest"
check_absent "themes" "Root Zed themes symlink"

if grep -q '^icon = "extension/icon.png"$' "zed/extension.toml" 2>/dev/null; then
    echo -e "  ${GREEN}OK${NC} Manifest icon path is relative to zed/"
else
    echo -e "  ${RED}FAIL${NC} zed/extension.toml should use icon = \"extension/icon.png\""
    ALL_CHECKS_PASSED=false
fi

if grep -q '"$schema": "https://zed.dev/schema/themes/v0.2.0.json"' "zed/themes/subliminal-nightfall.json" 2>/dev/null; then
    echo -e "  ${GREEN}OK${NC} Theme references the Zed theme schema"
else
    echo -e "  ${YELLOW}WARN${NC} Theme schema reference not found or incorrect"
fi

echo ""
if [ "$ALL_CHECKS_PASSED" = true ]; then
    echo -e "${GREEN}All critical checks passed.${NC}"
    echo "Install the dev extension from the zed/ directory."
    exit 0
else
    echo -e "${RED}Some checks failed.${NC}"
    exit 1
fi
