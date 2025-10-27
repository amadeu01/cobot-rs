# Servo Controller Testing

This document describes the testing strategy and implementation for the Cobot-RS servo controller module.

## Overview

The servo controller uses a two-tier testing approach:

1. **Embedded Unit Tests** - Basic tests within `servo_controller.rs` for hardware-dependent code
2. **Standalone Test Suite** - Comprehensive tests in `test_servo_controller.rs` without ESP32 dependencies

Due to ESP32-IDF dependencies, comprehensive testing requires the standalone test suite that can run on any platform.

## Test Structure

### 1. Embedded Unit Tests (`servo_controller.rs`)

Minimal tests that can run with ESP32 dependencies:
- **Basic Duty Calculation**: Core `angle_to_duty` function validation
- **Duty Bounds Checking**: Ensures duty values stay within hardware limits
- **ServoOperation Structure**: Tests the data structures used for threading

These tests are limited but ensure core mathematical functions work correctly in the ESP32 environment.

### 2. Standalone Test Suite (`test_servo_controller.rs`)

Comprehensive testing without hardware dependencies:
- **Mathematical Function Tests**: Full validation of all conversion functions
- **Mock Hardware Simulation**: Complete servo controller behavior simulation
- **Integration Tests**: Walking patterns and complex robot behaviors
- **Precision & Error Handling**: Boundary conditions and rounding behavior

## Running Tests

### Standalone Test Suite (Recommended)

```bash
# Compile and run comprehensive tests
rustc test_servo_controller.rs && ./test_servo_controller
```

### Embedded Unit Tests

Note: These may not work due to ESP32 dependencies
```bash
# Attempt to run embedded tests (may fail)
cargo test
```

## Test Results

The test suite includes 33+ individual test cases covering:

| Test Category | Test Count | Description | Location |
|---------------|------------|-------------|----------|
| Embedded Basic Tests | 3 | Core function validation | `servo_controller.rs` |
| Pulse Conversion | 5 | Angle to pulse width mapping | `test_servo_controller.rs` |
| Duty Calculation | 6 | PWM duty cycle calculations | `test_servo_controller.rs` |
| Range Validation | 3 | Boundary condition testing | `test_servo_controller.rs` |
| Mock Hardware | 8 | Hardware simulation tests | `test_servo_controller.rs` |
| Robot Behaviors | 11+ | Walking patterns and movements | `test_servo_controller.rs` |

### Expected Output

```
=== Servo Controller Unit Tests ===

Testing angle to pulse conversion...
Testing angle to duty calculations...
Testing duty cycle range...
...

=== Test Results Summary ===
Total Tests: 33
Passed: 33
Failed: 0
Success Rate: 100.0%
```

## Key Test Scenarios

### 1. Servo Angle Accuracy

Tests verify that servo angles are accurately converted to hardware values:

```rust
// 90° should produce 1500µs pulse width
assert_eq!(angle_to_pulse(90), 1500);

// With 10-bit resolution, 90° should produce duty value 76
assert_eq!(angle_to_duty(90, 1024), 76);
```

### 2. Walking Pattern Validation

Simulates complete walking cycles:

```rust
let walking_steps = vec![
    (45, 90, 45, 90),   // Lift right legs
    (135, 90, 135, 90), // Move forward
    (90, 45, 90, 45),   // Lift left legs
    (90, 135, 90, 135), // Move forward
    (90, 90, 90, 90),   // Center position
];
```

### 3. Precision Testing

Accounts for integer arithmetic precision loss:

```rust
// Allow ±2° tolerance due to integer rounding
assert_within_tolerance(actual_angle, expected_angle, 2);
```

## Hardware-Specific Considerations

### ESP32 LEDC Configuration

Tests validate calculations for different LEDC resolutions:

- **10-bit (1024 levels)**: Default ESP32 configuration
- **14-bit (16384 levels)**: Higher precision option
- **16-bit (65536 levels)**: Maximum precision testing

### Servo Specifications

Tests use standard hobby servo parameters:

- **Frequency**: 50 Hz (20ms period)
- **Pulse Range**: 500-2500µs (0-180°)
- **Center Position**: 1500µs (90°)

## Parallel Execution Testing

Tests verify that parallel servo calculations work correctly:

```rust
// Simulate parallel duty calculations
let angles = vec![0, 45, 90, 135, 180];
let duties: Vec<u32> = angles
    .iter()
    .map(|angle| angle_to_duty(*angle, max_duty))
    .collect();

// Verify results are consistent with sequential calculation
```

## Error Conditions

Tests validate error handling for:

- **Invalid Angles**: Values > 180° are clamped
- **Duty Overflow**: Prevents exceeding max_duty values
- **Hardware Failures**: Mock error injection testing

## Performance Metrics

While not measuring actual performance, tests verify:

- **Calculation Accuracy**: All mathematical operations produce expected results
- **Memory Efficiency**: Mock implementations use minimal memory
- **Thread Safety**: Calculations are pure functions safe for parallel execution

## Adding New Tests

### For Embedded Tests (servo_controller.rs)
Limited to tests that don't require mock hardware:
1. Add test function with `#[test]` attribute in the `tests` module
2. Keep tests simple and focused on mathematical functions
3. Avoid complex setup or mock objects due to ESP32 constraints

### For Standalone Tests (test_servo_controller.rs)
Full testing capabilities:
1. **Add to TestRunner**: Implement new test methods
2. **Call from run_all_tests()**: Include in test execution
3. **Update Documentation**: Document new test scenarios
4. **Verify Coverage**: Ensure all code paths are tested

### Example New Test

```rust
fn test_new_behavior(&mut self) {
    println!("Testing new behavior...");
    
    let mut controller = MockServoController::new(1024);
    
    // Test implementation
    controller.some_new_method().unwrap();
    
    // Assertions
    self.assert_eq("New behavior", expected, actual);
}
```

## Limitations

### Embedded Tests
- **ESP32 Dependencies**: May not run with standard `cargo test` due to platform constraints
- **Limited Scope**: Only basic mathematical functions can be tested
- **No Mock Hardware**: Cannot test complex behaviors or hardware simulation

### Standalone Tests  
- **No Real Hardware**: Tests use mocks, may miss hardware-specific issues
- **Integer Precision**: Some precision loss is expected and tolerated
- **Threading Simulation**: Parallel execution benefits are simulated, not measured

## Why Two Test Approaches?

The dual approach exists because:
1. **ESP-IDF Constraints**: Standard Rust testing doesn't work with ESP32 dependencies
2. **Development Flexibility**: Standalone tests can run on any development machine
3. **Hardware Validation**: Embedded tests ensure code works in the actual target environment
4. **Comprehensive Coverage**: Combined approach provides both basic validation and comprehensive testing

## Future Improvements

1. **Hardware-in-Loop Testing**: Integration with actual ESP32 hardware
2. **Performance Benchmarks**: Measure actual execution times
3. **Fuzzing**: Random input testing for edge cases
4. **CI Integration**: Automated testing in development pipeline
5. **Visual Verification**: Servo position plotting and animation

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Ensure Rust is installed and up to date
2. **Precision Failures**: Increase tolerance in assertions if needed
3. **Mock Behavior**: Verify mock implementations match hardware behavior

### Debug Information

Enable detailed logging by modifying test assertions:

```rust
println!("Debug: angle={}, duty={}, recovered={}", angle, duty, recovered_angle);
```

This comprehensive testing approach ensures the servo controller functions correctly across all supported scenarios while maintaining compatibility with the ESP32 embedded environment.