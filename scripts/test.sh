#!/bin/bash

# Cobot-RS Test Runner
# Convenient script for running mathematical function tests

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Cobot-RS Test Runner ===${NC}"



# Default mode is unit tests
MODE="test"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --demo|--visual)
            MODE="demo"
            shift
            ;;
        --unit|--test)
            MODE="test"
            shift
            ;;
        --clean)
            echo -e "${YELLOW}Cleaning build directory...${NC}"
            # Check if we're in project root or scripts directory
            if [ -d "build" ]; then
                rm -rf build/*
            elif [ -d "../build" ]; then
                rm -rf ../build/*
            fi
            echo -e "${GREEN}Build directory cleaned${NC}"
            exit 0
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "OPTIONS:"
            echo "  --test, --unit    Run unit tests (default)"
            echo "  --demo, --visual  Run visual demonstration"
            echo "  --clean          Clean build directory"
            echo "  --help, -h       Show this help message"
            echo ""
            echo "Examples:"
            echo "  ./scripts/test.sh           # Run unit tests"
            echo "  ./scripts/test.sh --demo    # Run visual demo"
            echo "  ./scripts/test.sh --clean   # Clean build files"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if we're in the project root or scripts directory
if [ -f "tests/servo_math.rs" ]; then
    # We're in project root - this is correct
    PROJECT_ROOT="."
elif [ -f "../tests/servo_math.rs" ]; then
    # We're in scripts directory - change to project root
    cd ..
    PROJECT_ROOT="."
else
    echo -e "${RED}Error: tests/servo_math.rs not found${NC}"
    echo "Please run this script from the cobot-rs project root:"
    echo "  ./scripts/test.sh"
    echo "Or from the scripts directory:"
    echo "  cd scripts && ./test.sh"
    exit 1
fi

# Create build directory if it doesn't exist
if [ ! -d "build" ]; then
    echo -e "${YELLOW}Creating build directory...${NC}"
    mkdir -p build
fi

if [ "$MODE" = "test" ]; then
    echo -e "${BLUE}Compiling and running unit tests...${NC}"
    echo -e "${YELLOW}Command: rustc --test tests/servo_math.rs -o build/servo_math${NC}"
    rustc --test tests/servo_math.rs -o build/servo_math
    echo -e "${GREEN}Compilation successful!${NC}"
    echo ""
    echo -e "${BLUE}Running unit tests:${NC}"
    ./build/servo_math
else
    echo -e "${BLUE}Compiling and running visual demonstration...${NC}"
    echo -e "${YELLOW}Command: rustc tests/servo_math.rs -o build/servo_math${NC}"
    rustc tests/servo_math.rs -o build/servo_math
    echo -e "${GREEN}Compilation successful!${NC}"
    echo ""
    echo -e "${BLUE}Running demonstration:${NC}"
    ./build/servo_math
fi

echo ""
echo -e "${GREEN}=== Test execution complete! ===${NC}"
echo -e "${YELLOW}Binary location: build/servo_math${NC}"
