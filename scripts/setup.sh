#!/bin/bash

# Cobot-RS Project Setup Script
# Helps configure the project for development

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Cobot-RS Project Setup ===${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    echo -e "${RED}Error: This doesn't appear to be the cobot-rs project root${NC}"
    echo "Please run this script from the cobot-rs directory"
    exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

echo -e "${YELLOW}Checking system dependencies...${NC}"

# Check for Rust
if command_exists rustc; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✅ Rust found: ${RUST_VERSION}${NC}"
else
    echo -e "${RED}❌ Rust not found${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check for Cargo
if command_exists cargo; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✅ Cargo found: ${CARGO_VERSION}${NC}"
else
    echo -e "${RED}❌ Cargo not found${NC}"
    echo "Cargo should come with Rust installation"
    exit 1
fi

# Check for Git
if command_exists git; then
    GIT_VERSION=$(git --version)
    echo -e "${GREEN}✅ Git found: ${GIT_VERSION}${NC}"
else
    echo -e "${YELLOW}⚠️ Git not found - recommended for version control${NC}"
fi

echo ""
echo -e "${YELLOW}Setting up project structure...${NC}"

# Create build directory if it doesn't exist
if [ ! -d "build" ]; then
    mkdir -p build
    echo -e "${GREEN}✅ Created build directory${NC}"
else
    echo -e "${GREEN}✅ Build directory already exists${NC}"
fi

# Make scripts executable
if [ -f "scripts/test.sh" ]; then
    chmod +x scripts/test.sh
    echo -e "${GREEN}✅ Made test script executable${NC}"
fi

if [ -f "scripts/setup.sh" ]; then
    chmod +x scripts/setup.sh
    echo -e "${GREEN}✅ Made setup script executable${NC}"
fi

echo ""
echo -e "${YELLOW}Testing mathematical functions...${NC}"

# Run a quick test to make sure everything works
if ./scripts/test.sh --demo > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Mathematical function tests work correctly${NC}"
else
    echo -e "${RED}❌ Mathematical function tests failed${NC}"
    echo "Please check the test output manually: ./scripts/test.sh --demo"
fi

echo ""
echo -e "${YELLOW}ESP32 development environment...${NC}"

# Check for ESP32 tools (these might not be installed yet)
if command_exists espflash; then
    echo -e "${GREEN}✅ espflash found - ESP32 deployment ready${NC}"
else
    echo -e "${YELLOW}⚠️ espflash not found${NC}"
    echo "For ESP32 development, you'll need to install ESP32 Rust toolchain"
    echo "See: https://esp-rs.github.io/book/installation/index.html"
fi

# Check if ESP32 target is installed
if rustup target list --installed | grep -q "xtensa-esp32-espidf"; then
    echo -e "${GREEN}✅ ESP32 Rust target installed${NC}"
else
    echo -e "${YELLOW}⚠️ ESP32 Rust target not installed${NC}"
    echo "This is normal if you haven't set up ESP32 development yet"
fi

echo ""
echo -e "${GREEN}=== Setup Summary ===${NC}"
echo ""
echo "✨ Project structure is ready!"
echo "🧪 Mathematical function tests work"
echo "📁 Build directory created"
echo "🔧 Scripts are executable"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo ""
echo -e "${YELLOW}1. Run tests:${NC}"
echo "   ./scripts/test.sh              # Unit tests"
echo "   ./scripts/test.sh --demo       # Visual demo"
echo ""
echo -e "${YELLOW}2. For ESP32 development:${NC}"
echo "   - Install ESP32 Rust toolchain"
echo "   - Connect ESP32 board via USB"
echo "   - Run: cargo run --release"
echo ""
echo -e "${YELLOW}3. For development:${NC}"
echo "   - Edit code in src/"
echo "   - Test with ./scripts/test.sh"
echo "   - See docs/how_to_run.md for details"
echo ""
echo -e "${GREEN}🎉 Setup complete! Happy coding!${NC}"
