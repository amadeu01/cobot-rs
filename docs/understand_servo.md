Great questions! Let me break this down for you and help you understand servo control concepts and testing strategies.

## Understanding Servo Control Terms

### What is PWM Duty Cycle?

**Duty cycle** is the percentage of time a PWM signal is HIGH during one period.

```
Period = 20ms (50Hz for servos)

|----HIGH----|---LOW---|  
0ms         1.5ms    20ms

Duty Cycle = (HIGH time / Period) × 100%
           = (1.5ms / 20ms) × 100% = 7.5%
```

### Key Terms Explained

**MIN_PULSE_WIDTH** (typically 500-1000µs): The shortest pulse that moves the servo to 0°
**MAX_PULSE_WIDTH** (typically 2000-2500µs): The longest pulse that moves the servo to 180°
**PERIOD_US** (20000µs = 20ms): Time for one complete PWM cycle (50Hz)
**max_duty**: The maximum counter value for your PWM timer (e.g., 65536 for 16-bit)

### Understanding `angle_to_duty` Function

Let me annotate your code:

```rust
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // Step 1: Calculate the pulse width range in microseconds
    let rise = MAX_PULSE_US - MIN_PULSE_US;  // e.g., 2500 - 500 = 2000µs
    let run = 180 - 0;                        // Full angle range: 180°
    
    // Step 2: Linear interpolation - convert angle to pulse width
    // If angle = 0°   → pulse_us = MIN_PULSE_US (500µs)
    // If angle = 90°  → pulse_us = 1500µs (middle)
    // If angle = 180° → pulse_us = MAX_PULSE_US (2500µs)
    let pulse_us = MIN_PULSE_US + ((angle * rise) / run);
    
    // Step 3: Convert pulse width to hardware duty value
    // The hardware counts from 0 to max_duty during each period
    // We need to calculate: what count gives us our desired pulse width?
    //
    // Example with 16-bit timer (max_duty = 65536):
    // Period = 20000µs, max_duty = 65536
    // For 1500µs pulse: duty = (1500 * 65536) / 20000 = 4915
    let duty = (pulse_us * max_duty) / PERIOD_US;
    
    // Step 4: Safety clamp to prevent overflow
    core::cmp::min(duty, max_duty)
}
```

**Why this works:**
- Duty value represents how many timer "ticks" the signal stays HIGH
- Timer counts from 0 to `max_duty` in `PERIOD_US` microseconds
- Each tick = `PERIOD_US / max_duty` microseconds
- To get pulse width W: duty = W / (PERIOD_US / max_duty) = (W × max_duty) / PERIOD_US

## Unit Testing Strategy## Key Testing Concepts

### 1. **Unit Tests** (Pure Functions)
Test your calculation logic without hardware:
- `angle_to_duty()` - Does the math work correctly?
- Edge cases (0°, 90°, 180°, >180°)
- Different timer resolutions (14-bit, 16-bit)
- Roundtrip conversions

### 2. **Mock Objects** (Integration Tests)
Simulate servo behavior without hardware:
```rust
struct MockServo {
    current_duty: u32,
    command_history: Vec<u32>,  // Track all commands sent
}
```

This lets you:
- Verify command sequences
- Test movement patterns
- Debug without physical hardware

### 3. **Property-Based Tests**
Verify invariants hold for all inputs:
- Duty always increases with angle (monotonicity)
- Duty never exceeds `max_duty`
- Roundtrip conversions preserve values (within tolerance)

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_angle_to_duty_16bit

# Run with output
cargo test -- --nocapture

# Run only unit tests (exclude integration tests)
cargo test --lib
```

## Visual Understanding

```
Angle:     0°          90°         180°
           |           |            |
Pulse:   500µs      1500µs       2500µs
           |           |            |
Duty:    1638        4915         8192   (for 16-bit timer)
           |           |            |
%Duty:   2.5%        7.5%        12.5%   (of 20ms period)
```

The math essentially creates a linear mapping:
- **Input domain**: 0-180° (angles)
- **Output range**: MIN_PULSE to MAX_PULSE (microseconds)
- **Hardware encoding**: Duty values (timer-specific)

Does this clarify things? Would you like me to explain any specific part in more detail?