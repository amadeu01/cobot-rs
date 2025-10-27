# Rust Embedded Architecture Best Practices

This document outlines the architectural best practices implemented in the Cobot-RS project and provides guidance for Rust embedded development.

## Overview

The Cobot-RS project demonstrates proper Rust embedded architecture by solving common challenges:

1. **Hardware Abstraction** - Separating hardware-specific code from business logic
2. **Testability** - Enabling comprehensive testing without hardware dependencies
3. **Code Organization** - Structuring code for maintainability and reusability
4. **Platform Portability** - Supporting multiple targets (ESP32, native, etc.)
5. **Feature Management** - Using Cargo features for conditional compilation

## Architectural Principles

### 1. Layered Architecture

```
┌─────────────────────────────────────┐
│          Application Layer          │  ← main.rs, robot behaviors
│ (Business Logic, Movement Patterns) │
├─────────────────────────────────────┤
│         Abstraction Layer           │  ← Traits, generic controllers
│    (Hardware-agnostic interfaces)   │
├─────────────────────────────────────┤
│        Implementation Layer         │  ← ESP32 drivers, mock drivers
│     (Platform-specific drivers)     │
├─────────────────────────────────────┤
│           Core Math Layer           │  ← Pure mathematical functions
│      (Platform-independent)         │
└─────────────────────────────────────┘
```

**Benefits:**
- Each layer has a single responsibility
- Dependencies flow downward only
- Easy to test each layer independently
- Platform-specific code is isolated

### 2. Dependency Inversion

Instead of depending on concrete implementations:

```rust
// ❌ Bad: Direct dependency on hardware
struct Robot {
    servo1: Esp32Servo,  // Tightly coupled to ESP32
    servo2: Esp32Servo,
}

// ✅ Good: Dependency on abstraction
struct Robot<S: ServoControl> {
    servo1: S,  // Can be any implementation of ServoControl
    servo2: S,
}
```

**Benefits:**
- Testable with mock implementations
- Portable across different hardware platforms
- Easier to extend and modify

### 3. Trait-Based Hardware Abstraction

Define behavior contracts through traits:

```rust
pub trait ServoDriver {
    fn get_max_duty(&self) -> u32;
    fn set_duty(&mut self, duty: u32) -> Result<(), ServoError>;
    fn get_current_duty(&self) -> u32;
}

pub trait ServoControl {
    fn set_angle(&mut self, angle: u32) -> Result<(), ServoError>;
    fn get_angle(&self) -> u32;
    fn get_name(&self) -> &str;
}
```

**Benefits:**
- Clear contracts between components
- Multiple implementations possible (ESP32, mock, future platforms)
- Composable and extensible

## Project Structure Best Practices

### 1. Library-First Design

```
src/
├── lib.rs              # Library root with public API
├── main.rs             # Binary application (thin wrapper)
└── modules/            # Internal modules
```

**Why Library-First?**
- Enables proper testing with `cargo test`
- Reusable across multiple binaries
- Clear separation of library vs application code
- Better for documentation and examples

### 2. Module Organization

```rust
// lib.rs structure
pub mod constants;      // Shared constants
pub mod math;          // Pure mathematical functions  
pub mod hal;           // Hardware abstraction traits
pub mod esp32;         // ESP32-specific implementations
pub mod mock;          // Mock implementations for testing
pub mod robot;         // High-level robot controller
pub mod patterns;      // Movement patterns and behaviors
```

**Benefits:**
- Logical grouping by responsibility
- Easy to navigate and understand
- Clear public API surface
- Scalable as project grows

### 3. Conditional Compilation Strategy

```rust
// Use Cargo features for different compilation targets
#[cfg(all(feature = "esp32", not(test)))]
pub mod esp32;

#[cfg(any(feature = "mock", test))]
pub mod mock;

// Platform-specific code
#[cfg(feature = "esp32")]
use esp_idf_hal::delay::FreeRtos;

#[cfg(feature = "std")]
use std::thread::sleep;
```

**Cargo.toml Features:**
```toml
[features]
default = ["esp32"]
esp32 = ["esp-idf-svc", "esp-idf-hal", "esp-idf-sys"]
mock = ["std"]
std = []
```

**Benefits:**
- Single codebase supports multiple targets
- Optional dependencies reduce bloat
- Clear feature boundaries
- Easy to test without hardware

## Testing Architecture

### 1. Multi-Level Testing Strategy

```
Integration Tests    ← tests/ directory, test actual library API
       ↑
Unit Tests          ← Each module tests its own functionality  
       ↑
Math Tests          ← Core algorithms, platform-independent
```

### 2. Mock-Driven Testing

```rust
// Production code uses traits
fn control_robot<S: ServoControl>(servos: &mut [S]) {
    for servo in servos {
        servo.set_angle(90)?;
    }
}

// Tests use mock implementations
#[test] 
fn test_robot_control() {
    let mut mock_servo = MockServo::new();
    control_robot(&mut [mock_servo]);
    assert_eq!(mock_servo.get_angle(), 90);
}
```

### 3. Integration Test Structure

```rust
// tests/integration_tests.rs
use my_crate::{MockDriver, Servo, Robot};

#[test]
fn test_complete_walking_sequence() {
    let robot = create_mock_robot();
    robot.walk_forward()?;
    // Test actual library code, not duplicated logic
}
```

## Error Handling Patterns

### 1. Custom Error Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ServoError {
    HardwareError(String),
    InvalidAngle(u32),
    InvalidDuty(u32),
}

impl Display for ServoError { /* ... */ }
impl Error for ServoError { /* ... */ }  // Only with std
```

### 2. Result Propagation

```rust
// Consistent error handling throughout the stack
pub fn set_robot_position(robot: &mut Robot, angles: &[u32]) -> Result<(), ServoError> {
    for (servo, &angle) in robot.servos().zip(angles) {
        servo.set_angle(angle)?;  // Propagate errors upward
    }
    Ok(())
}
```

### 3. Error Context

```rust
// Provide meaningful error context
self.ledc.set_duty(duty)
    .map_err(|e| ServoError::HardwareError(
        format!("Failed to set duty {}: {:?}", duty, e)
    ))
```

## Performance Considerations

### 1. Zero-Cost Abstractions

```rust
// Traits compile to direct function calls (no vtables)
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // Pure function, inlines completely
}

// Generic functions monomorphize (no runtime cost)
pub fn control_servo<S: ServoControl>(servo: &mut S, angle: u32) {
    // Compiles to direct calls for each concrete type
}
```

### 2. Compile-Time Optimization

```rust
// Constants are compile-time evaluated
pub const MIN_PULSE_US: u32 = 500;
pub const MAX_PULSE_US: u32 = 2500;
pub const PULSE_RANGE: u32 = MAX_PULSE_US - MIN_PULSE_US;  // = 2000
```

### 3. Memory Efficiency

```rust
// Use appropriate integer sizes
pub struct ServoConfig {
    angle: u8,        // 0-180 fits in u8
    max_duty: u16,    // Most PWM is ≤ 65536
}

// Avoid heap allocation in no_std
pub struct Robot<S, const N: usize> {
    servos: [S; N],   // Fixed-size array, stack allocated
}
```

## Platform Integration Patterns

### 1. ESP32 Integration

```rust
// Wrap ESP-IDF types in our abstraction
pub struct Esp32ServoDriver<'a> {
    ledc: LedcDriver<'a>,
}

impl<'a> ServoDriver for Esp32ServoDriver<'a> {
    fn set_duty(&mut self, duty: u32) -> Result<(), ServoError> {
        self.ledc.set_duty(duty)
            .map_err(|e| ServoError::HardwareError(format!("{:?}", e)))
    }
}
```

### 2. Cross-Platform Compatibility

```rust
// Abstract platform differences
#[cfg(feature = "esp32")]
fn delay_ms(ms: u32) {
    esp_idf_hal::delay::FreeRtos::delay_ms(ms);
}

#[cfg(feature = "std")]
fn delay_ms(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

#[cfg(not(any(feature = "esp32", feature = "std")))]
fn delay_ms(_ms: u32) {
    // No-op for testing or other platforms
}
```

## Documentation Standards

### 1. Module-Level Documentation

```rust
//! # Servo Control Module
//!
//! This module provides hardware abstraction for servo motor control.
//!
//! ## Usage
//!
//! ```rust
//! use cobot_rs::{MockServoDriver, Servo};
//! 
//! let driver = MockServoDriver::new(1024);
//! let mut servo = Servo::new(driver, "test".to_string());
//! servo.set_angle(90)?;
//! ```
```

### 2. Function Documentation

```rust
/// Convert servo angle (0-180°) to PWM duty cycle value
///
/// # Arguments
/// * `angle` - Servo angle in degrees (0-180, will be clamped)
/// * `max_duty` - Maximum duty cycle value (hardware dependent)
///
/// # Returns
/// Duty cycle value to write to PWM hardware
///
/// # Examples
/// ```
/// assert_eq!(angle_to_duty(90, 1024), 76);
/// ```
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // Implementation
}
```

### 3. Architecture Documentation

Document key architectural decisions:

- Why certain abstractions were chosen
- Trade-offs between different approaches  
- How to extend the system
- Platform-specific considerations

## Common Anti-Patterns to Avoid

### 1. ❌ Duplicated Logic in Tests

```rust
// Don't duplicate the production logic in tests
fn test_angle_calculation() {
    let expected = MIN_PULSE + (angle * RANGE) / 180;  // Duplicated!
    assert_eq!(angle_to_duty(angle, max), expected);
}
```

### 2. ❌ Platform-Specific Code in Business Logic

```rust
// Don't mix platform code with business logic
fn walk_forward(robot: &mut Robot) -> Result<(), Error> {
    robot.set_angles([45, 90, 45, 90])?;
    FreeRtos::delay_ms(500);  // ❌ ESP32-specific!
    // ...
}
```

### 3. ❌ Concrete Types in High-Level APIs

```rust
// Don't expose concrete types in public APIs
pub struct RobotController {
    servo1: Esp32Servo,  // ❌ Tied to ESP32
    servo2: Esp32Servo,
}
```

### 4. ❌ Mixed Abstraction Levels

```rust
// Don't mix low-level and high-level operations
pub fn complex_movement(robot: &mut Robot) -> Result<(), Error> {
    robot.walk_forward()?;           // High level
    robot.servo1.set_duty(1234)?;    // ❌ Low level mixed in!
}
```

## Benefits of This Architecture

1. **Testability**: Comprehensive testing without hardware
2. **Maintainability**: Clear separation of concerns
3. **Portability**: Easy to support new platforms  
4. **Reusability**: Library can be used in multiple projects
5. **Performance**: Zero-cost abstractions, compile-time optimization
6. **Safety**: Rust's type system prevents common embedded errors
7. **Documentation**: Self-documenting code through types and traits

## Conclusion

This architecture balances the constraints of embedded development (no_std, resource limits, hardware specificity) with the benefits of modern software engineering (testability, maintainability, portability).

The key insight is that **proper abstraction doesn't cost performance in Rust** - the compiler optimizes away the abstractions, leaving you with efficient code that's also well-organized and testable.

By following these patterns, you can create embedded Rust projects that are both high-performance and maintainable.