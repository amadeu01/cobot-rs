#!/bin/bash

# Banner Image Diagnostic and Fix Script
# Helps diagnose and fix banner image display issues on GitHub

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Cobot-RS Banner Diagnostic Tool ===${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "readme.md" ] || [ ! -d "assets" ]; then
    echo -e "${RED}Error: This doesn't appear to be the cobot-rs project root${NC}"
    echo "Please run this script from the cobot-rs directory"
    exit 1
fi

echo -e "${YELLOW}Diagnosing banner image issues...${NC}"
echo ""

# Check 1: File existence
echo -e "${BLUE}1. Checking file existence...${NC}"
if [ -f "assets/banner.png" ]; then
    echo -e "${GREEN}✅ banner.png exists in assets/ directory${NC}"

    # Get file size
    FILE_SIZE=$(ls -lh assets/banner.png | awk '{print $5}')
    echo -e "   File size: ${FILE_SIZE}"

    # Check file type
    FILE_TYPE=$(file assets/banner.png 2>/dev/null || echo "unknown")
    echo -e "   File type: ${FILE_TYPE}"

else
    echo -e "${RED}❌ banner.png NOT found in assets/ directory${NC}"
    echo -e "${YELLOW}Creating a placeholder banner...${NC}"

    # Create assets directory if it doesn't exist
    mkdir -p assets

    # Create a simple text-based banner placeholder
    cat > assets/banner.png << 'EOF'
iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==
EOF
    echo -e "${YELLOW}⚠️ Created minimal placeholder. Replace with actual banner image.${NC}"
fi

# Check 2: Git tracking
echo ""
echo -e "${BLUE}2. Checking git tracking...${NC}"
if git ls-files assets/banner.png | grep -q banner.png; then
    echo -e "${GREEN}✅ banner.png is tracked by git${NC}"

    # Check if it's modified
    if git status --porcelain assets/banner.png | grep -q banner.png; then
        echo -e "${YELLOW}⚠️ banner.png has uncommitted changes${NC}"
        git status assets/banner.png
    else
        echo -e "${GREEN}✅ banner.png is up to date${NC}"
    fi
else
    echo -e "${RED}❌ banner.png is NOT tracked by git${NC}"
    echo -e "${YELLOW}Adding to git...${NC}"
    git add assets/banner.png
    echo -e "${GREEN}✅ Added banner.png to git${NC}"
fi

# Check 3: Current README banner reference
echo ""
echo -e "${BLUE}3. Checking README banner reference...${NC}"
if grep -q "banner" readme.md; then
    BANNER_LINE=$(grep "banner" readme.md | head -1)
    echo -e "${GREEN}✅ Banner reference found in readme.md${NC}"
    echo -e "   Current: ${BANNER_LINE}"

    # Check if it's using relative path
    if echo "$BANNER_LINE" | grep -q "assets/banner.png"; then
        echo -e "${YELLOW}⚠️ Using relative path (may not work on GitHub)${NC}"
    fi

    # Check if it's using absolute GitHub URL
    if echo "$BANNER_LINE" | grep -q "raw.githubusercontent.com"; then
        echo -e "${GREEN}✅ Using absolute GitHub URL (recommended)${NC}"
    fi
else
    echo -e "${RED}❌ No banner reference found in readme.md${NC}"
fi

# Check 4: Get GitHub repository info
echo ""
echo -e "${BLUE}4. Checking GitHub repository info...${NC}"
if git remote get-url origin >/dev/null 2>&1; then
    REMOTE_URL=$(git remote get-url origin)
    echo -e "${GREEN}✅ Git remote found: ${REMOTE_URL}${NC}"

    # Extract username and repo from URL
    if [[ $REMOTE_URL =~ github\.com[:/]([^/]+)/([^/.]+) ]]; then
        USERNAME="${BASH_REMATCH[1]}"
        REPO="${BASH_REMATCH[2]}"
        echo -e "   Username: ${USERNAME}"
        echo -e "   Repository: ${REPO}"

        # Generate correct banner URL
        GITHUB_BANNER_URL="https://raw.githubusercontent.com/${USERNAME}/${REPO}/main/assets/banner.png"
        echo -e "   Correct banner URL: ${GITHUB_BANNER_URL}"
    else
        echo -e "${YELLOW}⚠️ Could not parse GitHub username/repo from remote URL${NC}"
        USERNAME="YOUR_USERNAME"
        REPO="cobot-rs"
        GITHUB_BANNER_URL="https://raw.githubusercontent.com/${USERNAME}/${REPO}/main/assets/banner.png"
    fi
else
    echo -e "${RED}❌ No git remote configured${NC}"
    USERNAME="YOUR_USERNAME"
    REPO="cobot-rs"
    GITHUB_BANNER_URL="https://raw.githubusercontent.com/${USERNAME}/${REPO}/main/assets/banner.png"
fi

# Check 5: Test banner accessibility
echo ""
echo -e "${BLUE}5. Testing banner accessibility...${NC}"
if command -v curl >/dev/null 2>&1 && [[ $USERNAME != "YOUR_USERNAME" ]]; then
    echo -e "${YELLOW}Testing GitHub raw URL...${NC}"
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$GITHUB_BANNER_URL" || echo "000")

    if [ "$HTTP_STATUS" = "200" ]; then
        echo -e "${GREEN}✅ Banner accessible via GitHub raw URL${NC}"
    elif [ "$HTTP_STATUS" = "404" ]; then
        echo -e "${RED}❌ Banner not found at GitHub raw URL (404)${NC}"
        echo -e "${YELLOW}   This usually means the file isn't pushed to GitHub yet${NC}"
    else
        echo -e "${YELLOW}⚠️ HTTP Status: ${HTTP_STATUS}${NC}"
    fi
else
    echo -e "${YELLOW}⚠️ Cannot test accessibility (curl not available or username not set)${NC}"
fi

# Solutions section
echo ""
echo -e "${BLUE}=== SOLUTIONS ===${NC}"
echo ""

echo -e "${YELLOW}Option 1: Fix README with absolute GitHub URL (RECOMMENDED)${NC}"
echo "Replace the banner line in readme.md with:"
echo -e "${GREEN}![Cobot-RS banner](${GITHUB_BANNER_URL})${NC}"
echo ""

echo -e "${YELLOW}Option 2: Use relative path (simple but may not always work)${NC}"
echo "Use this format in readme.md:"
echo -e "${GREEN}![Cobot-RS banner](assets/banner.png)${NC}"
echo ""

echo -e "${YELLOW}Option 3: Auto-fix README (run this fix)${NC}"
echo "Would you like to automatically update the README? [y/N]"
read -r RESPONSE
if [[ $RESPONSE =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Updating README with absolute GitHub URL...${NC}"

    # Backup original README
    cp readme.md readme.md.backup

    # Replace banner line with absolute URL
    sed -i.tmp "s|!\[.*banner.*\](.*)|![Cobot-RS banner](${GITHUB_BANNER_URL})|g" readme.md
    rm readme.md.tmp 2>/dev/null || true

    echo -e "${GREEN}✅ README updated with absolute GitHub URL${NC}"
    echo -e "${YELLOW}Backup created as readme.md.backup${NC}"

    # Show the change
    echo ""
    echo -e "${BLUE}Updated banner line:${NC}"
    grep "banner" readme.md | head -1
fi

# Final recommendations
echo ""
echo -e "${BLUE}=== FINAL RECOMMENDATIONS ===${NC}"
echo ""

if [ ! -f "assets/banner.png" ]; then
    echo -e "${RED}🔴 CRITICAL: Create or add a proper banner image${NC}"
    echo "   - Create assets/banner.png with your project banner"
    echo "   - Recommended size: 1200×400 pixels"
    echo "   - Format: PNG or JPG"
    echo "   - Keep file size under 1MB"
    echo ""
fi

if ! git ls-files assets/banner.png | grep -q banner.png; then
    echo -e "${YELLOW}🟡 ACTION NEEDED: Commit banner to git${NC}"
    echo "   Run: git add assets/banner.png && git commit -m 'Add project banner'"
    echo ""
fi

if [[ $USERNAME == "YOUR_USERNAME" ]]; then
    echo -e "${YELLOW}🟡 ACTION NEEDED: Configure git remote${NC}"
    echo "   Run: git remote add origin https://github.com/YOURUSERNAME/cobot-rs.git"
    echo "   Then update banner URL with your actual username"
    echo ""
fi

echo -e "${BLUE}Next steps:${NC}"
echo "1. Ensure banner image exists and is committed"
echo "2. Push changes to GitHub: ${GREEN}git push origin main${NC}"
echo "3. Check GitHub repository page to verify banner displays"
echo "4. Update any forks or templates with correct URLs"
echo ""

echo -e "${GREEN}✨ Banner diagnostic complete!${NC}"
echo ""
echo -e "${YELLOW}💡 Pro tip: Test your README on GitHub to make sure the banner displays correctly${NC}"
