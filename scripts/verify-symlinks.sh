#!/bin/bash

# Subliminal Nightfall - Symlink Verification Script
# Verifies that the themes/ symlink is properly configured

set -e

echo "🔍 Verifying Subliminal Nightfall symlink setup..."
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track if all checks pass
ALL_CHECKS_PASSED=true

# Check 1: Verify symlink exists
echo "1️⃣  Checking if themes/ symlink exists..."
if [ -L "themes" ]; then
    echo -e "   ${GREEN}✓${NC} Symlink exists"
else
    echo -e "   ${RED}✗${NC} Symlink does not exist"
    ALL_CHECKS_PASSED=false
fi
echo ""

# Check 2: Verify symlink target
echo "2️⃣  Checking symlink target..."
if [ -L "themes" ]; then
    TARGET=$(readlink themes)
    if [ "$TARGET" = "zed/themes" ]; then
        echo -e "   ${GREEN}✓${NC} Symlink points to zed/themes"
    else
        echo -e "   ${RED}✗${NC} Symlink points to $TARGET (expected: zed/themes)"
        ALL_CHECKS_PASSED=false
    fi
else
    echo -e "   ${YELLOW}⊘${NC} Skipped (symlink doesn't exist)"
fi
echo ""

# Check 3: Verify source directory exists
echo "3️⃣  Checking if zed/themes/ directory exists..."
if [ -d "zed/themes" ]; then
    echo -e "   ${GREEN}✓${NC} Source directory zed/themes/ exists"
else
    echo -e "   ${RED}✗${NC} Source directory zed/themes/ does not exist"
    ALL_CHECKS_PASSED=false
fi
echo ""

# Check 4: Verify theme file exists
echo "4️⃣  Checking if theme file exists..."
if [ -f "zed/themes/subliminal-nightfall.json" ]; then
    echo -e "   ${GREEN}✓${NC} Theme file zed/themes/subliminal-nightfall.json exists"
else
    echo -e "   ${RED}✗${NC} Theme file zed/themes/subliminal-nightfall.json does not exist"
    ALL_CHECKS_PASSED=false
fi
echo ""

# Check 5: Verify theme file is accessible through symlink
echo "5️⃣  Checking if theme file is accessible through symlink..."
if [ -f "themes/subliminal-nightfall.json" ]; then
    echo -e "   ${GREEN}✓${NC} Theme file accessible via themes/subliminal-nightfall.json"
else
    echo -e "   ${RED}✗${NC} Theme file NOT accessible via symlink"
    ALL_CHECKS_PASSED=false
fi
echo ""

# Check 6: Verify extension.toml exists at root
echo "6️⃣  Checking if extension.toml exists at root..."
if [ -f "extension.toml" ]; then
    echo -e "   ${GREEN}✓${NC} extension.toml exists at repository root"
else
    echo -e "   ${RED}✗${NC} extension.toml does not exist at repository root"
    ALL_CHECKS_PASSED=false
fi
echo ""

# Check 7: Verify theme JSON schema
echo "7️⃣  Checking theme JSON schema reference..."
if grep -q '"$schema": "https://zed.dev/schema/themes/v0.2.0.json"' "zed/themes/subliminal-nightfall.json" 2>/dev/null; then
    echo -e "   ${GREEN}✓${NC} Theme references correct Zed schema"
else
    echo -e "   ${YELLOW}⚠${NC}  Theme schema reference not found or incorrect"
fi
echo ""

# Check 8: Verify Git tracking
echo "8️⃣  Checking Git symlink tracking..."
if git ls-files themes > /dev/null 2>&1; then
    if git ls-files themes | grep -q "themes"; then
        echo -e "   ${GREEN}✓${NC} Symlink is tracked by Git"
    else
        echo -e "   ${YELLOW}⚠${NC}  Symlink exists but may not be tracked by Git yet (run 'git add themes')"
    fi
else
    echo -e "   ${YELLOW}⚠${NC}  Not a Git repository or Git command failed"
fi
echo ""

# Final summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ "$ALL_CHECKS_PASSED" = true ]; then
    echo -e "${GREEN}✓ All critical checks passed!${NC}"
    echo ""
    echo "Your symlink setup is correct. The Zed extension should work properly."
    echo ""
    echo "To test locally:"
    echo "  1. Open Zed"
    echo "  2. Run: 'zed: install dev extension'"
    echo "  3. Select this repository root directory"
    echo "  4. Run: 'theme selector: toggle'"
    echo "  5. Choose 'Subliminal Nightfall'"
    exit 0
else
    echo -e "${RED}✗ Some checks failed${NC}"
    echo ""
    echo "To fix the symlink setup:"
    echo "  cd $(pwd)"
    echo "  rm -f themes"
    echo "  ln -s zed/themes themes"
    echo "  git add themes"
    exit 1
fi
