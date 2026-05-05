#!/bin/bash
# ============================================================
# iZCore Release Trigger Script
# Usage: ./release.sh v0.1.0
# ============================================================

set -e

VERSION="${1:-v0.1.0}"

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'
BOLD='\033[1m'; NC='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "╔══════════════════════════════════════════╗"
echo "║   iZCore Release Publisher               ║"
echo "╚══════════════════════════════════════════╝"
echo -e "${NC}"
echo -e "  Version: ${BOLD}${VERSION}${NC}"
echo ""

# Validate version format
if ! echo "$VERSION" | grep -qE '^v[0-9]+\.[0-9]+\.[0-9]+'; then
    echo -e "${RED}✗ Invalid version format. Use: v0.1.0${NC}"
    exit 1
fi

# Check we're on main branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
    echo -e "${RED}✗ Must be on 'main' branch (current: $BRANCH)${NC}"
    exit 1
fi

# Check clean working tree
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}✗ Working tree has uncommitted changes. Commit first.${NC}"
    git status --short
    exit 1
fi

echo -e "${GREEN}✓ Branch: main${NC}"
echo -e "${GREEN}✓ Working tree clean${NC}"

# Update version in Cargo.toml workspace
echo -e "\n[1/4] Updating workspace version to ${VERSION}..."
VERSION_NUM="${VERSION#v}"  # strip 'v' prefix
sed -i '' "s/^version = \".*\"/version = \"${VERSION_NUM}\"/" Cargo.toml 2>/dev/null || \
sed -i "s/^version = \".*\"/version = \"${VERSION_NUM}\"/" Cargo.toml

git add Cargo.toml
git commit -m "chore: bump version to ${VERSION}" || true

echo -e "${GREEN}✓ Version bumped to ${VERSION_NUM}${NC}"

# Push latest commits
echo -e "\n[2/4] Pushing commits..."
git push origin main
echo -e "${GREEN}✓ Pushed to main${NC}"

# Create and push tag
echo -e "\n[3/4] Creating tag ${VERSION}..."
git tag -a "${VERSION}" -m "iZCore ${VERSION} — DNA Kernel Release

Platforms: Windows x86_64 | macOS Apple Silicon | macOS Intel
           Linux x86_64 | Linux ARM64 | Android ARM64 | Android ARMv7
           Embedded ARMv7hf

Install: curl -fsSL https://boss.iz.life/install | sh"

git push origin "${VERSION}"
echo -e "${GREEN}✓ Tag ${VERSION} pushed — GitHub Actions build started!${NC}"

# Show status
echo -e "\n[4/4] Build triggered! Monitor progress:"
echo -e "  ${CYAN}https://github.com/iZFxTrade/iZBoss/actions${NC}"
echo -e "\n  Release will be available at:"
echo -e "  ${CYAN}https://github.com/iZFxTrade/iZBoss/releases/tag/${VERSION}${NC}"
echo ""
echo -e "${GREEN}${BOLD}✓ Release ${VERSION} is on its way!${NC}"
