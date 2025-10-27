# Tests Directory

This directory contains standalone test files that can run on any system without requiring ESP32 hardware.

## Files

- **`servo_math.rs`** - Mathematical function tests for servo controller calculations

## Running Tests

### Using the Test Script (Recommended)

From the project root directory:

```bash
./scripts/test.sh           # Run unit tests
./scripts/test.sh --demo    # Run visual demonstration
./scripts/test.sh --clean   # Clean build directory
./scripts/test.sh --help    # Show all options
```

### Manual Compilation

```bash
# Run unit tests
rustc --test tests/servo_math.rs -o build/servo_math && ./build/servo_math

# Run visual demonstration
rustc tests/servo_math.rs -o build/servo_math && ./build/servo_math
```

## What Gets Tested

### Mathematical Functions
- **Angle to PWM Duty Conversion**: Converting servo angles (0-180°) to PWM duty cycle values
- **Pulse Width Calculations**: Converting angles to microsecond pulse widths (500-2500µs)
- **Roundtrip Accuracy**: Testing precision of angle → duty → angle conversions
- **Boundary Conditions**: Testing edge cases like angles > 180°, duty overflow prevention
- **Multi-Resolution Support**: Testing 8-bit, 10-bit, 12-bit, 14-bit, and 16-bit PWM

### Test Coverage
- **10 Unit Tests** covering all mathematical functions
- **Visual Output** showing calculations in action
- **No Hardware Dependencies** - runs on any system with Rust

## Output Directory

All compiled test binaries are placed in the `build/` directory which is:
- Git-ignored (not committed to version control)
- Automatically created when needed
- Can be cleaned with `./test.sh --clean`

## Example Output

### Unit Tests
```
running 10 tests
test tests::test_angle_clamping ... ok
test tests::test_angle_to_duty_basic ... ok
test tests::test_boundary_conditions ... ok
test tests::test_different_resolutions ... ok
test tests::test_duty_bounds ... ok
test tests::test_mathematical_precision ... ok
test tests::test_pulse_width_calculation ... ok
test tests::test_roundtrip_conversion ... ok
test tests::test_servo_range_mapping ... ok
test tests::test_precise_angles ... FAILED

test result: FAILED. 9 passed; 1 failed; 0 ignored
```

### Visual Demo
```
=== Servo Controller Mathematical Function Tests ===

Testing basic angle to duty conversion:
  0° → duty: 25, pulse: 500µs
  30° → duty: 42, pulse: 833µs
  90° → duty: 76, pulse: 1500µs
  180° → duty: 128, pulse: 2500µs

Testing different PWM resolutions:
  8-bit (256): 90° = 19 duty (7.4%)
  10-bit (1024): 90° = 76 duty (7.4%)
  12-bit (4096): 90° = 307 duty (7.5%)
  14-bit (16384): 90° = 1228 duty (7.5%)
```

## Why Standalone Tests?

1. **Development Speed** - Instant feedback without flashing to hardware
2. **No Hardware Required** - Test core logic on any development machine
3. **Comprehensive Coverage** - Can test edge cases and boundary conditions easily
4. **CI/CD Friendly** - Can run in automated build environments

## Adding New Tests

To add new mathematical function tests:

1. Edit `servo_math.rs`
2. Add your test function with `#[test]` attribute in the `tests` module
3. Run `./test.sh` to verify your tests pass
4. Update documentation if needed

Example:
```rust
#[test]
fn test_new_calculation() {
    let result = your_function(input);
    assert_eq!(result, expected_value);
}
```

## Integration with Main Project

These tests validate the same mathematical functions used in the main servo controller (`src/servo_controller.rs`). The functions are duplicated here to avoid ESP32 dependencies, allowing the tests to run on any system.

For hardware-dependent testing, use `cargo test` from the project root (requires connected ESP32 board).