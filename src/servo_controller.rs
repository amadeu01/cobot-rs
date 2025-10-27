//! # Servo Controller Module
//!
//! This module provides servo motor control functionality for ESP32-based 4-legged robots.
//! It includes mathematical functions, hardware abstraction, and robot movement patterns.
//!
//! ## Architecture
//!
//! The module is organized into layers:
//! - **Core Math Functions**: Platform-independent servo calculations
//! - **Hardware Integration**: ESP32 LEDC driver integration
//! - **Robot Controller**: High-level movement coordination
//! - **Movement Patterns**: Pre-defined behaviors (walking, waving, etc.)
//!
//! ## Usage
//!
//! ```rust
//! use servo_controller::{setup_servos, demo_servo_movements};
//! use esp_idf_hal::peripherals::Peripherals;
//!
//! let mut servo_controller = setup_servos(Peripherals::take().unwrap())?;
//! servo_controller.set_all_servos_angle(90)?; // Center all servos
//! servo_controller.walk_forward(300)?;        // Execute walking pattern
//! ```

use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver, config::TimerConfig};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;
use std::sync::mpsc;
use std::thread;

// ================================================================================================
// CONSTANTS AND CONFIGURATION
// ================================================================================================

/// Servo configuration constants matching ESP32Servo library standards
pub const FREQUENCY_HZ: u32 = 50; // 50 Hz for servos

/// Standard hobby servo constants
pub const MIN_PULSE_US: u32 = 500; // Microseconds for 0 degrees (approx 0.5ms)
pub const MAX_PULSE_US: u32 = 2500; // Microseconds for 180 degrees (approx 2.5ms)
pub const PERIOD_US: u32 = 20000; // Microseconds for 50Hz (20ms)

// ================================================================================================
// CORE MATHEMATICAL FUNCTIONS
// ================================================================================================

/// Convert servo angle (0-180°) to PWM duty cycle value
///
/// This is the core mathematical function used throughout the library.
/// It's pure and testable without hardware dependencies.
///
/// # Arguments
/// * `angle` - Servo angle in degrees (0-180, will be clamped)
/// * `max_duty` - Maximum duty cycle value (hardware dependent)
///
/// # Returns
/// Duty cycle value to write to PWM hardware
///
/// # Example
/// ```
/// let duty = angle_to_duty(90, 1024); // 90° on 10-bit PWM = 76
/// ```
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // Clamp angle to valid range
    let angle = angle.min(180);

    // Linear interpolation: angle → pulse width
    let pulse_range = MAX_PULSE_US - MIN_PULSE_US;
    let pulse_us = MIN_PULSE_US + ((angle * pulse_range) / 180);

    // Convert pulse width to duty cycle value
    let duty = (pulse_us * max_duty) / PERIOD_US;

    // Safety clamp
    duty.min(max_duty)
}

/// Convert duty cycle value back to angle (for verification/debugging)
pub fn duty_to_angle(duty: u32, max_duty: u32) -> u32 {
    if max_duty == 0 {
        return 0; // Prevent division by zero
    }

    let pulse_us = (duty * PERIOD_US) / max_duty;
    let pulse_range = MAX_PULSE_US - MIN_PULSE_US;

    if pulse_us <= MIN_PULSE_US {
        0
    } else if pulse_us >= MAX_PULSE_US {
        180
    } else {
        ((pulse_us - MIN_PULSE_US) * 180) / pulse_range
    }
}

/// Calculate expected pulse width for a given angle
pub fn angle_to_pulse_width(angle: u32) -> u32 {
    let angle = angle.min(180);
    let pulse_range = MAX_PULSE_US - MIN_PULSE_US;
    MIN_PULSE_US + ((angle * pulse_range) / 180)
}

// ================================================================================================
// SERVO OPERATION DATA STRUCTURE
// ================================================================================================

/// Servo operation for threaded execution
#[derive(Debug, Clone)]
pub struct ServoOperation {
    pub angle: u32,
    pub max_duty: u32,
    pub servo_name: String,
}

// ================================================================================================
// ROBOT CONTROLLER
// ================================================================================================

/// 4-legged robot servo controller with parallel execution capabilities
pub struct ServoController<'a> {
    right_back_leg: LedcDriver<'a>,
    left_back_leg: LedcDriver<'a>,
    right_front_leg: LedcDriver<'a>,
    left_front_leg: LedcDriver<'a>,
}

impl<'a> ServoController<'a> {
    /// Create a new ServoController with the given LEDC drivers
    pub fn new(
        right_back_leg: LedcDriver<'a>,
        left_back_leg: LedcDriver<'a>,
        right_front_leg: LedcDriver<'a>,
        left_front_leg: LedcDriver<'a>,
    ) -> Self {
        Self {
            right_back_leg,
            left_back_leg,
            right_front_leg,
            left_front_leg,
        }
    }

    /// Set all servos to the same angle using parallel calculation
    ///
    /// This function calculates duty values in parallel threads, then applies them
    /// sequentially to avoid hardware conflicts.
    pub fn set_all_servos_angle(&mut self, angle: u32) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        // Prepare servo operations
        let operations = vec![
            ServoOperation {
                angle,
                max_duty: self.right_back_leg.get_max_duty(),
                servo_name: "right_back_leg".to_string(),
            },
            ServoOperation {
                angle,
                max_duty: self.left_back_leg.get_max_duty(),
                servo_name: "left_back_leg".to_string(),
            },
            ServoOperation {
                angle,
                max_duty: self.right_front_leg.get_max_duty(),
                servo_name: "right_front_leg".to_string(),
            },
            ServoOperation {
                angle,
                max_duty: self.left_front_leg.get_max_duty(),
                servo_name: "left_front_leg".to_string(),
            },
        ];

        // Spawn threads to calculate duty values
        for op in operations {
            let tx_clone = tx.clone();
            let handle = thread::spawn(move || {
                let duty = angle_to_duty(op.angle, op.max_duty);
                log::debug!(
                    "Calculated {} duty: {} for angle: {}",
                    op.servo_name,
                    duty,
                    op.angle
                );
                tx_clone.send((op.servo_name, duty)).unwrap();
            });
            handles.push(handle);
        }

        // Drop the original sender
        drop(tx);

        // Collect results from threads
        let mut duties = std::collections::HashMap::new();
        for received in rx {
            duties.insert(received.0, received.1);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Apply calculated duties to servos sequentially (hardware operations)
        self.right_back_leg.set_duty(duties["right_back_leg"])?;
        self.left_back_leg.set_duty(duties["left_back_leg"])?;
        self.right_front_leg.set_duty(duties["right_front_leg"])?;
        self.left_front_leg.set_duty(duties["left_front_leg"])?;

        log::info!(
            "All servos set to {} degrees using parallel calculation",
            angle
        );
        Ok(())
    }

    /// Set individual servo angles using parallel calculation
    pub fn set_servo_angles(
        &mut self,
        right_back: u32,
        left_back: u32,
        right_front: u32,
        left_front: u32,
    ) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        // Prepare servo operations
        let operations = vec![
            ServoOperation {
                angle: right_back,
                max_duty: self.right_back_leg.get_max_duty(),
                servo_name: "right_back_leg".to_string(),
            },
            ServoOperation {
                angle: left_back,
                max_duty: self.left_back_leg.get_max_duty(),
                servo_name: "left_back_leg".to_string(),
            },
            ServoOperation {
                angle: right_front,
                max_duty: self.right_front_leg.get_max_duty(),
                servo_name: "right_front_leg".to_string(),
            },
            ServoOperation {
                angle: left_front,
                max_duty: self.left_front_leg.get_max_duty(),
                servo_name: "left_front_leg".to_string(),
            },
        ];

        // Spawn threads to calculate duty values
        for op in operations {
            let tx_clone = tx.clone();
            let handle = thread::spawn(move || {
                let duty = angle_to_duty(op.angle, op.max_duty);
                tx_clone.send((op.servo_name, duty)).unwrap();
            });
            handles.push(handle);
        }

        drop(tx);

        let mut duties = std::collections::HashMap::new();
        for received in rx {
            duties.insert(received.0, received.1);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Apply calculated duties to servos
        self.right_back_leg.set_duty(duties["right_back_leg"])?;
        self.left_back_leg.set_duty(duties["left_back_leg"])?;
        self.right_front_leg.set_duty(duties["right_front_leg"])?;
        self.left_front_leg.set_duty(duties["left_front_leg"])?;

        log::debug!("Individual servos set using parallel calculation");
        Ok(())
    }

    /// Set right side servos to specific angles
    pub fn set_right_servos(&mut self, back_angle: u32, front_angle: u32) -> Result<()> {
        self.right_back_leg.set_duty(angle_to_duty(
            back_angle,
            self.right_back_leg.get_max_duty(),
        ))?;
        self.right_front_leg.set_duty(angle_to_duty(
            front_angle,
            self.right_front_leg.get_max_duty(),
        ))?;
        Ok(())
    }

    /// Set left side servos to specific angles
    pub fn set_left_servos(&mut self, back_angle: u32, front_angle: u32) -> Result<()> {
        self.left_back_leg
            .set_duty(angle_to_duty(back_angle, self.left_back_leg.get_max_duty()))?;
        self.left_front_leg.set_duty(angle_to_duty(
            front_angle,
            self.left_front_leg.get_max_duty(),
        ))?;
        Ok(())
    }

    /// Center all servos to 90 degrees
    pub fn center_all_servos(&mut self) -> Result<()> {
        self.set_all_servos_angle(90)
    }

    /// Get max duty values for debugging
    pub fn log_max_duties(&self) {
        log::info!(
            "Max duty values - right_back_leg: {}, left_back_leg: {}, right_front_leg: {}, left_front_leg: {}",
            self.right_back_leg.get_max_duty(),
            self.left_back_leg.get_max_duty(),
            self.right_front_leg.get_max_duty(),
            self.left_front_leg.get_max_duty()
        );
    }

    /// Perform a walking motion pattern with parallel servo control
    pub fn walk_forward(&mut self, delay_ms: u32) -> Result<()> {
        log::info!("Starting walk forward pattern with parallel servo control");

        // Step 1: Lift right legs
        self.set_servo_angles(45, 90, 45, 90)?;
        FreeRtos::delay_ms(delay_ms);

        // Step 2: Move right legs forward
        self.set_servo_angles(135, 90, 135, 90)?;
        FreeRtos::delay_ms(delay_ms);

        // Step 3: Put right legs down, lift left legs
        self.set_servo_angles(90, 45, 90, 45)?;
        FreeRtos::delay_ms(delay_ms);

        // Step 4: Move left legs forward
        self.set_servo_angles(90, 135, 90, 135)?;
        FreeRtos::delay_ms(delay_ms);

        // Step 5: Return to center
        self.center_all_servos()?;
        FreeRtos::delay_ms(delay_ms);

        Ok(())
    }

    /// Perform a simple wave motion with threaded calculation
    pub fn wave(&mut self, delay_ms: u32) -> Result<()> {
        log::info!("Starting wave motion with parallel calculation");

        // Wave with front right leg - forward sweep
        for angle in (0..=180).step_by(10) {
            let max_duty = self.right_front_leg.get_max_duty();

            // Calculate duty in a separate thread
            let handle = thread::spawn(move || angle_to_duty(angle, max_duty));
            let duty = handle.join().unwrap();

            self.right_front_leg.set_duty(duty)?;
            FreeRtos::delay_ms(delay_ms);
        }

        // Wave with front right leg - reverse sweep
        for angle in (0..=180).rev().step_by(10) {
            let max_duty = self.right_front_leg.get_max_duty();

            // Calculate duty in a separate thread
            let handle = thread::spawn(move || angle_to_duty(angle, max_duty));
            let duty = handle.join().unwrap();

            self.right_front_leg.set_duty(duty)?;
            FreeRtos::delay_ms(delay_ms);
        }

        // Return to center
        self.center_all_servos()?;
        Ok(())
    }
}

// ================================================================================================
// HARDWARE SETUP FUNCTIONS
// ================================================================================================

/// Set up servo motors and return a ServoController
pub fn setup_servos(peripherals: Peripherals) -> Result<ServoController<'static>> {
    log::info!("Setting up servo motors with parallel control capability");

    // LEDC Timer configuration
    let timer_config = TimerConfig::default()
        .frequency(Hertz(FREQUENCY_HZ).into())
        .resolution(esp_idf_hal::ledc::Resolution::Bits10);

    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

    // Create LEDC drivers for each servo
    let right_back_leg =
        LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio23)?;

    let left_back_leg =
        LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio22)?;

    let right_front_leg =
        LedcDriver::new(peripherals.ledc.channel2, &timer, peripherals.pins.gpio19)?;

    let left_front_leg =
        LedcDriver::new(peripherals.ledc.channel3, &timer, peripherals.pins.gpio18)?;

    let servo_controller = ServoController::new(
        right_back_leg,
        left_back_leg,
        right_front_leg,
        left_front_leg,
    );

    servo_controller.log_max_duties();
    log::info!("Servo controller initialized with parallel execution support");

    Ok(servo_controller)
}

/// Demonstrate servo movements with parallel control
pub fn demo_servo_movements(servo_controller: &mut ServoController) -> Result<()> {
    log::info!("Starting servo demonstration with parallel control...");

    // Set all servos to 180 degrees
    servo_controller.set_all_servos_angle(180)?;
    log::info!("All servos set to 180 degrees (parallel execution)");
    FreeRtos::delay_ms(1000);

    // Set all servos to 90 degrees
    servo_controller.set_all_servos_angle(90)?;
    log::info!("All servos set to 90 degrees (parallel execution)");
    FreeRtos::delay_ms(1000);

    // Set all servos to 0 degrees
    servo_controller.set_all_servos_angle(0)?;
    log::info!("All servos set to 0 degrees (parallel execution)");
    FreeRtos::delay_ms(1000);

    // Test individual leg control
    log::info!("Testing individual leg movements with parallel calculation...");
    servo_controller.set_servo_angles(45, 135, 135, 45)?;
    log::info!("Diagonal movement pattern (parallel execution)");
    FreeRtos::delay_ms(1000);

    // Test side movements
    log::info!("Testing side movements with parallel calculation...");
    servo_controller.set_right_servos(45, 45)?;
    FreeRtos::delay_ms(500);
    servo_controller.set_left_servos(135, 135)?;
    FreeRtos::delay_ms(500);

    // Return to center position
    servo_controller.center_all_servos()?;
    log::info!("Servos centered to 90 degrees (parallel execution)");

    log::info!("Servo demonstration with parallel control complete");
    Ok(())
}

// ================================================================================================
// TESTING MODULE
// ================================================================================================

/// ## Testing Strategy
///
/// This module includes basic embedded tests, but for comprehensive testing
/// without ESP32 dependencies, run the standalone test suite:
///
/// ```bash
/// cargo test --features mock
/// ```
///
/// The tests validate:
/// - Core mathematical functions (angle_to_duty, duty_to_angle, etc.)
/// - Servo operation data structures
/// - Basic functionality that can run in ESP32 environment
///
/// For more comprehensive testing with mock hardware, see the documentation
/// on testing strategies for embedded Rust projects.

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic angle_to_duty calculation for ESP32 10-bit LEDC
    #[test]
    fn test_angle_to_duty_basic() {
        let max_duty = 1024; // ESP32 10-bit default

        // Test key angles
        assert_eq!(angle_to_duty(0, max_duty), 25); // 0°
        assert_eq!(angle_to_duty(90, max_duty), 76); // 90° (center)
        assert_eq!(angle_to_duty(180, max_duty), 128); // 180°
    }

    /// Test that duty values are within valid range
    #[test]
    fn test_duty_bounds() {
        let max_duty = 1024;

        // Test all angles produce valid duty values
        for angle in 0..=180 {
            let duty = angle_to_duty(angle, max_duty);
            assert!(
                duty <= max_duty,
                "Duty {} exceeds max_duty for angle {}",
                duty,
                angle
            );
        }
    }

    /// Test angle clamping behavior
    #[test]
    fn test_angle_clamping() {
        let max_duty = 1024;

        // Angles over 180 should be clamped
        assert_eq!(angle_to_duty(200, max_duty), angle_to_duty(180, max_duty));
        assert_eq!(angle_to_duty(999, max_duty), angle_to_duty(180, max_duty));
    }

    /// Test roundtrip conversion accuracy
    #[test]
    fn test_roundtrip_conversion() {
        let max_duty = 65536; // Higher precision for better roundtrip

        for angle in (0..=180).step_by(10) {
            let duty = angle_to_duty(angle, max_duty);
            let recovered_angle = duty_to_angle(duty, max_duty);
            let diff = (recovered_angle as i32 - angle as i32).abs();

            assert!(
                diff <= 2,
                "Roundtrip failed: {} → {} → {} (diff: {})",
                angle,
                duty,
                recovered_angle,
                diff
            );
        }
    }

    /// Test ServoOperation struct
    #[test]
    fn test_servo_operation() {
        let op = ServoOperation {
            angle: 90,
            max_duty: 1024,
            servo_name: "test_servo".to_string(),
        };

        assert_eq!(op.angle, 90);
        assert_eq!(op.max_duty, 1024);
        assert_eq!(op.servo_name, "test_servo");
    }

    /// Test pulse width calculation
    #[test]
    fn test_pulse_width_calculation() {
        assert_eq!(angle_to_pulse_width(0), MIN_PULSE_US);
        assert_eq!(angle_to_pulse_width(90), 1500);
        assert_eq!(angle_to_pulse_width(180), MAX_PULSE_US);
    }
}
