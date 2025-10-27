# Simple and Practical Rust Embedded Architecture

This document outlines a simple, working approach to Rust embedded architecture that avoids the common pitfalls and overcomplications.

## The Problem We Solved

The initial architecture became too complex and had several issues:

1. **Code Duplication**: Tests duplicated production logic
2. **Over-Engineering**: Complex trait hierarchies and conditional compilation
3. **Memory Issues**: ESP32 couldn't run complex tests
4. **Maintenance Burden**: Multiple abstractions layers to maintain
5. **Broken Tests**: Tests didn't actually test the production code

## The Simple Solution

### 1. Single Module Architecture

```
src/
├── main.rs              # Application entry point
└── servo_controller.rs  # All servo logic in one place
```

**Benefits:**
- Easy to understand and navigate
- No complex module dependencies
- Everything servo-related is in one file
- Tests are co-located with the code they test

### 2. Core Functions + Integration

```rust
// servo_controller.rs structure:

// 1. Constants (shared, testable)
pub const MIN_PULSE_US: u32 = 500;
pub const MAX_PULSE_US: u32 = 2500;

// 2. Pure mathematical functions (easily testable)
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 { /* */ }

// 3. Hardware integration (ESP32-specific)
pub struct ServoController<'a> { /* LEDC drivers */ }

// 4. High-level behaviors (robot movements)
impl ServoController {
    pub fn walk_forward(&mut self) -> Result<()> { /* */ }
}

// 5. Setup functions
pub fn setup_servos(peripherals: Peripherals) -> Result<ServoController> { /* */ }

// 6. Tests (test the actual functions above)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_angle_to_duty() {
        assert_eq!(angle_to_duty(90, 1024), 76); // Tests actual function
    }
}
```

### 3. Testing Strategy

#### What Works: Simple Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_functions() {
        // Test the actual production functions
        assert_eq!(angle_to_duty(0, 1024), 25);
        assert_eq!(angle_to_duty(90, 1024), 76);
        assert_eq!(angle_to_duty(180, 1024), 128);
    }
    
    #[test]
    fn test_data_structures() {
        // Test data structure creation and methods
        let op = ServoOperation {
            angle: 90,
            max_duty: 1024,
            servo_name: "test".to_string(),
        };
        assert_eq!(op.angle, 90);
    }
}
```

#### What Doesn't Work: Complex Integration Tests

❌ **Avoid:**
- Mock hardware implementations
- Complex trait hierarchies
- Separate test files that duplicate logic
- Running tests on ESP32 (memory constraints)

### 4. File Organization

```
cobot-rs/
├── src/
│   ├── main.rs                 # Application logic
│   └── servo_controller.rs     # All servo functionality + tests
├── docs/
│   ├── how_to_run.md          # User guide
│   └── architecture/          # Architecture decisions
└── Cargo.toml                 # Simple dependencies
```

**Key Principles:**
- Keep related code together
- Minimize file count for small projects
- Co-locate tests with implementation
- Document architecture decisions

## What We Learned

### 1. ESP32 Constraints Are Real

- **Memory Limits**: ESP32 can't run complex test frameworks
- **std vs no_std**: Standard library features may not work
- **Hardware Dependencies**: Some code only works on actual hardware

### 2. Simple Is Better

- **One Module**: Easier than multiple interconnected modules
- **Direct Functions**: Clearer than abstraction layers
- **Embedded Tests**: More reliable than separate test files

### 3. Testing Strategy

```rust
// ✅ Good: Test mathematical functions directly
#[test]
fn test_angle_calculation() {
    let result = angle_to_duty(90, 1024);  // Test actual function
    assert_eq!(result, 76);
}

// ❌ Bad: Duplicate the logic in tests
#[test]
fn test_angle_calculation_bad() {
    let expected = 500 + ((90 * 2000) / 180); // Duplicated logic!
    assert_eq!(angle_to_duty(90, 1024), expected);
}
```

## Implementation Guidelines

### 1. Keep Functions Pure When Possible

```rust
// Pure function - easy to test
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    let angle = angle.min(180);
    let pulse_us = MIN_PULSE_US + ((angle * (MAX_PULSE_US - MIN_PULSE_US)) / 180);
    (pulse_us * max_duty) / PERIOD_US
}

// Hardware function - harder to test but necessary
pub fn setup_servos(peripherals: Peripherals) -> Result<ServoController> {
    // ESP32-specific setup
}
```

### 2. Use Simple Error Handling

```rust
// Simple - use existing error types
pub fn set_angle(&mut self, angle: u32) -> Result<()> {
    if angle > 180 {
        return Err(anyhow::anyhow!("Invalid angle: {}", angle));
    }
    // ... rest of function
}

// Don't create custom error types unless you really need them
```

### 3. Document Decisions

```rust
/// Convert servo angle (0-180°) to PWM duty cycle value
/// 
/// # Why This Algorithm
/// Uses linear interpolation between MIN_PULSE_US and MAX_PULSE_US
/// to match standard servo control specifications.
/// 
/// # Arguments
/// * `angle` - Servo angle in degrees (0-180, will be clamped)
/// * `max_duty` - Maximum duty cycle value (hardware dependent)
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // Implementation with clear steps
}
```

## Running Tests

### Development Testing

```bash
# Run unit tests (tests mathematical functions)
cargo test

# If ESP32 tests fail due to hardware constraints, that's expected
# The mathematical functions still get tested
```

### Hardware Testing

```bash
# Deploy to ESP32 and test manually
cargo run --release

# Look for log output confirming servo movements
```

## Benefits of This Approach

1. **Maintainable**: Everything in one place, easy to understand
2. **Testable**: Core logic is tested without hardware dependencies  
3. **Practical**: Focuses on what actually needs to be tested
4. **Scalable**: Can grow into multiple modules when actually needed
5. **Reliable**: Less complex abstractions mean fewer bugs

## When to Add Complexity

Add abstractions and separate modules when:

1. **File Size**: Single module becomes > 1000 lines
2. **Multiple Hardware Targets**: Need to support different chips
3. **Team Size**: Multiple people working on different parts
4. **Reuse**: Logic needs to be shared across projects

## Conclusion

The best architecture is the simplest one that meets your current needs. 

For a robotics project like Cobot-RS:
- One module for servo control is sufficient
- Pure mathematical functions can be thoroughly tested
- Hardware integration can be tested manually on the device
- Complex abstractions add more problems than they solve

This approach delivers working code that's easy to understand, test, and maintain.