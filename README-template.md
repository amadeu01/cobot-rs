![Cobot-RS banner](https://raw.githubusercontent.com/YOUR_USERNAME/cobot-rs/main/assets/banner.png)

# Cobot-RS

<!-- 
🔧 SETUP INSTRUCTIONS: Replace YOUR_USERNAME with your actual GitHub username in all URLs below
Example: Replace "YOUR_USERNAME" with "johndoe" if your GitHub username is johndoe
This includes banner URL above and all badge URLs below
-->

[![Core CI](https://github.com/YOUR_USERNAME/cobot-rs/workflows/Core%20CI/badge.svg)](https://github.com/YOUR_USERNAME/cobot-rs/actions/workflows/core.yml)
[![Mathematical Tests](https://github.com/YOUR_USERNAME/cobot-rs/workflows/Core%20CI/badge.svg?event=push)](https://github.com/YOUR_USERNAME/cobot-rs/actions/workflows/core.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org/)

🤖 Cobot-rs

A cute little robot powered by Rust 🦀⚡

Cobot is a small DIY robot project built in Rust, designed to explore robotics, sensors, and fun interactions. With its friendly face and wiggly legs, Cobot makes tinkering with embedded systems both educational and adorable.

## About

An ESP32-based collaborative robot (cobot) project written in Rust for learning embedded systems programming.

This project controls a 4-legged robot equipped with distance sensors. The cobot can be controlled remotely through multiple connectivity options including WiFi, internet, and potentially Bluetooth.

This project serves as a learning platform for:
- Rust programming language
- Embedded systems development
- ESP32 microcontroller programming

## Features

### Hardware
- [ ] 4-leg locomotion system
- [ ] Distance sensor integration
- [ ] ESP32 microcontroller

### Connectivity
- [ ] WiFi control
- [ ] Internet-based remote control
- [ ] Bluetooth support (planned)

### Control Features
- [ ] Basic movement commands
- [ ] Obstacle detection
- [ ] Autonomous navigation
- [ ] Remote monitoring

### Software
- [ ] Rust embedded environment setup
- [ ] Motor control implementation
- [ ] Sensor data processing
- [ ] Communication protocols

## Quick Start

1. **Prerequisites**: Make sure you have the ESP32 Rust development environment set up
2. **Connect**: Connect your ESP32 board via USB
3. **Deploy**: Run the following command to build and flash the firmware:

```bash
cargo run --release
```

This will compile the project, flash it to your ESP32, and start monitoring the serial output.

## Testing

**Run tests without hardware (recommended for development):**
```bash
# Use the convenient test script
./scripts/test.sh           # Run unit tests
./scripts/test.sh --demo    # Run visual demonstration
./scripts/test.sh --clean   # Clean build directory
./scripts/test.sh --help    # Show all options

# Or run manually
rustc --test tests/servo_math.rs -o build/servo_math && ./build/servo_math
```

**Run tests on ESP32 (requires connected board):**
```bash
cargo test
```

The mathematical function tests can run on any system and validate the core servo control calculations without requiring ESP32 hardware. Tests are organized in the `tests/` directory with binaries compiled to `build/` (git-ignored).

## CI/CD Status

<!-- 
🔧 SETUP INSTRUCTIONS: After pushing to GitHub, update YOUR_USERNAME in the URL below
-->

Our continuous integration automatically tests:
- ✅ **Mathematical Functions** - Core servo calculations
- ✅ **Code Quality** - Formatting and linting  
- ✅ **Scripts** - Test runner functionality
- ✅ **Documentation** - Link validation and structure
- ✅ **Project Structure** - Directory organization

[![View CI Results](https://img.shields.io/github/workflow/status/YOUR_USERNAME/cobot-rs/Core%20CI?label=tests&logo=github)](https://github.com/YOUR_USERNAME/cobot-rs/actions)

## Project Structure

```
cobot-rs/
├── src/                     # ESP32 source code
│   ├── main.rs             # Main application
│   └── servo_controller.rs # Servo control logic + embedded tests
├── tests/                   # Standalone tests (no hardware needed)
│   ├── servo_math.rs       # Mathematical function tests
│   └── README.md           # Testing documentation
├── scripts/                 # Build and test scripts
│   ├── test.sh             # Convenient test runner
│   └── setup.sh            # Project setup script
├── build/                   # Compiled test binaries (git-ignored)
├── docs/                    # Documentation
├── .github/workflows/       # CI/CD automation
└── assets/                  # Images and resources
```

## Getting Started

### Setup Script
Run the automated setup to configure your development environment:

```bash
./scripts/setup.sh
```

This will:
- Check for required dependencies (Rust, Git, etc.)
- Create necessary directories
- Make scripts executable
- Test mathematical functions
- Provide next steps

### Manual Setup
For detailed setup instructions, deployment options, and troubleshooting, see the [How to Run Guide](docs/how_to_run.md).

## Development Workflow

1. **Code** → Make your changes in `src/`
2. **Test Math** → `./scripts/test.sh` (instant feedback)
3. **Visual Check** → `./scripts/test.sh --demo` (see calculations)
4. **Test Hardware** → `cargo run` (requires ESP32 connected)
5. **Deploy** → `cargo run --release` (optimized build)

## Hardware Requirements

- ESP32 development board
- 4-leg chassis with servo motors
- Distance sensors
- Motor drivers
- Power supply

## Contributing

1. Fork the repository
2. Create a feature branch
3. Run tests: `./scripts/test.sh`
4. Commit your changes
5. Push to your branch
6. Create a Pull Request

The CI will automatically test your changes for:
- Mathematical function correctness
- Code formatting and quality
- Documentation completeness
- Script functionality

## Setting Up GitHub Integration

### Step 1: Update URLs
After forking this repository, replace `YOUR_USERNAME` in the following files with your GitHub username:
- This README file (banner URL and badge URLs at the top)
- Any documentation that references the repository

### Step 2: Banner Image
The banner image should automatically work once you replace YOUR_USERNAME in the URL:
```markdown
![Cobot-RS banner](https://raw.githubusercontent.com/YOUR_USERNAME/cobot-rs/main/assets/banner.png)
```

### Step 3: Enable GitHub Actions
1. Go to your repository on GitHub
2. Click on the "Actions" tab
3. GitHub Actions should automatically detect the workflow files in `.github/workflows/`
4. The first push to `main` branch will trigger the CI pipeline

### Step 4: Monitor CI Status
Once set up, you can monitor your CI status:
- Check the badges at the top of this README
- Visit the Actions tab in your GitHub repository
- CI will run automatically on pushes and pull requests

## License

Licensed under the MIT License.