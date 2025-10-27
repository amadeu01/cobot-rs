# How to Run Cobot-RS

This guide explains how to build, deploy, and run the Cobot-RS project on your ESP32 development board.

## Prerequisites

Before running the project, ensure you have the following installed:

1. **Rust with ESP toolchain**: The project uses the ESP-specific Rust toolchain
   - The project is configured to use `channel = "esp"` (see `rust-toolchain.toml`)
   - Make sure you have the ESP32 Rust development environment set up

2. **espflash**: Required for flashing the firmware to the ESP32
   - This should be available if you have the ESP Rust environment properly configured

3. **Hardware**: 
   - ESP32 development board
   - USB cable for connection
   - Any additional hardware components (sensors, motors, etc.) as per your setup

## Project Structure

The project is organized into modular components:

- **`src/main.rs`**: Main application entry point and robot behavior logic
- **`src/servo_controller.rs`**: Servo motor control module with movement patterns
- **Configuration files**: `.cargo/config.toml`, `espflash.toml`, etc.

### Servo Controller Module

The servo controller manages four servo motors representing the robot's legs:
- GPIO 23: Right back leg
- GPIO 22: Left back leg  
- GPIO 19: Right front leg
- GPIO 18: Left front leg

It provides high-level functions for:
- Individual servo control
- Coordinated multi-servo movements
- Pre-defined patterns (walking, waving, etc.)

## Project Configuration

The project is pre-configured for ESP32 development with the following key settings:

### Target Configuration (`.cargo/config.toml`)
- **Target**: `xtensa-esp32-espidf` (ESP32 with ESP-IDF framework)
- **Runner**: `espflash flash --monitor` (automatically flashes and starts monitoring)
- **Linker**: `ldproxy` for ESP32 linking
- **ESP-IDF Version**: v5.3.3

### Flash Configuration (`espflash.toml`)
- **Baud Rate**: 115200 (standard communication speed)

## Running the Project

### Quick Start - Build and Deploy

To build the project and deploy it to your ESP32 board, simply run:

```bash
cargo run --release
```

This single command will:
1. **Build** the project with release optimizations
2. **Flash** the firmware to the connected ESP32 board
3. **Monitor** the serial output automatically

### What Happens Under the Hood

When you run `cargo run --release`, the following process occurs:

1. **Compilation**: Cargo compiles the Rust code for the `xtensa-esp32-espidf` target
2. **Optimization**: Uses release profile optimizations (defined in `Cargo.toml`):
   - Size optimization (`opt-level = "s"`)
   - Link-time optimization (`lto = 'fat'`)
   - Single codegen unit for better optimization
3. **Linking**: Uses `ldproxy` to create ESP32-compatible binary
4. **Flashing**: `espflash` automatically flashes the binary to the board
5. **Monitoring**: Starts serial monitoring to show runtime output

### Alternative Commands

#### Development Build (Debug)
For faster compilation during development (with some optimizations):
```bash
cargo run
```

#### Build Only (No Flashing)
To just build without flashing to the board:
```bash
cargo build --release
```

#### Manual Flashing
If you need more control over the flashing process:
```bash
cargo build --release
espflash flash --monitor target/xtensa-esp32-espidf/release/cobot-rs
```

## Monitoring and Debugging

### Serial Monitor
The project is configured to automatically start monitoring after flashing. You'll see:
- Boot messages from ESP32
- Application logs (using the `log` crate)
- Any debug output from your code

### Log Levels
The project uses Rust's standard `log` crate. You can control log output by setting environment variables or modifying the code.

### Stopping the Monitor
To exit the serial monitor, typically use `Ctrl+C`.

## Troubleshooting

### Common Issues

1. **Board Not Found**
   - Ensure ESP32 is connected via USB
   - Check that the correct driver is installed
   - Verify the board is not being used by another application

2. **Permission Errors** (Linux/macOS)
   - Add your user to the `dialout` group: `sudo usermod -a -G dialout $USER`
   - Or run with `sudo` (not recommended)

3. **Compilation Errors**
   - Ensure you have the ESP Rust toolchain installed
   - Check that ESP-IDF dependencies are properly configured
   - Try cleaning and rebuilding: `cargo clean && cargo build`

4. **Flash Errors**
   - Try holding the BOOT button while flashing
   - Check the baud rate settings in `espflash.toml`
   - Ensure no other program is using the serial port

### Getting Help

If you encounter issues:
1. Check the ESP32 Rust documentation
2. Verify your hardware connections
3. Review the serial output for error messages
4. Ensure all dependencies are properly installed

## Development Workflow

For active development:

1. **Code** → Make your changes
2. **Test** → `cargo run` (for faster debug builds)
3. **Deploy** → `cargo run --release` (for optimized builds)
4. **Monitor** → Watch serial output for debugging

The configuration is optimized for this workflow with automatic building, flashing, and monitoring in a single command.

## Hardware Setup

Make sure your ESP32 board is:
- Connected via USB cable
- Powered appropriately
- Has any required external components connected according to your circuit design

The project targets standard ESP32 boards and should work with most development boards without modification.