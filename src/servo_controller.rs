//! # Servo Controller Module
//!
//! This module provides servo motor control functionality for the Cobot-RS 4-legged robot.
//! It manages four servo motors representing the robot's legs and provides high-level
//! movement patterns and individual servo control.
//!
//! ## Features
//!
//! - Individual servo angle control
//! - Coordinated multi-servo movements with parallel execution
//! - Pre-defined movement patterns (walking, waving, etc.)
//! - Hardware abstraction for ESP32 LEDC peripheral
//! - Threaded duty cycle calculations for improved performance
//!
//! ## Hardware Configuration
//!
//! The module is configured for standard hobby servos (0-180 degrees) with:
//! - Frequency: 50 Hz
//! - Pulse width: 0.5ms to 2.4ms (corresponding to 0° to 180°)
//! - GPIO pins: 18, 19, 22, 23 for the four servos
//!
//! ## Usage Example
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

// Servo configuration constants
const FREQUENCY: u32 = 50; // 50 Hz for servos

// Standard hobby servo constants
const MIN_PULSE_US: u32 = 500; // Microseconds for 0 degrees (approx 0.5ms)
const MAX_PULSE_US: u32 = 2400; // Microseconds for 180 degrees (approx 2.5ms)
const PERIOD_US: u32 = 20000; // Microseconds for 50Hz (20ms)

/// Struct to hold all servo drivers for a 4-legged robot
///
/// This controller manages four servo motors representing the legs of the robot:
/// - right_back_leg: Back right leg servo
/// - left_back_leg: Back left leg servo
/// - right_front_leg: Front right leg servo
/// - left_front_leg: Front left leg servo
#[allow(dead_code)]
pub struct ServoController<'a> {
    right_back_leg: LedcDriver<'a>,
    left_back_leg: LedcDriver<'a>,
    right_front_leg: LedcDriver<'a>,
    left_front_leg: LedcDriver<'a>,
}

/// Servo operation for threaded execution
#[derive(Debug, Clone)]
struct ServoOperation {
    angle: u32,
    max_duty: u32,
    servo_name: String,
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

    /// Set all servos to the same angle using parallel execution
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

        // Drop the original sender to close the channel when all threads are done
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

        // Apply calculated duties to servos sequentially (but calculations were parallel)
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

    /// Set individual servo angles using parallel execution
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

        // Apply calculated duties to servos
        self.right_back_leg.set_duty(duties["right_back_leg"])?;
        self.left_back_leg.set_duty(duties["left_back_leg"])?;
        self.right_front_leg.set_duty(duties["right_front_leg"])?;
        self.left_front_leg.set_duty(duties["left_front_leg"])?;

        log::info!("Individual servos set using parallel calculation");
        Ok(())
    }

    /// Set right side servos to specific angles using parallel execution
    pub fn set_right_servos(&mut self, back_angle: u32, front_angle: u32) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        let operations = vec![
            ServoOperation {
                angle: back_angle,
                max_duty: self.right_back_leg.get_max_duty(),
                servo_name: "right_back_leg".to_string(),
            },
            ServoOperation {
                angle: front_angle,
                max_duty: self.right_front_leg.get_max_duty(),
                servo_name: "right_front_leg".to_string(),
            },
        ];

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

        self.right_back_leg.set_duty(duties["right_back_leg"])?;
        self.right_front_leg.set_duty(duties["right_front_leg"])?;

        Ok(())
    }

    /// Set left side servos to specific angles using parallel execution
    pub fn set_left_servos(&mut self, back_angle: u32, front_angle: u32) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        let operations = vec![
            ServoOperation {
                angle: back_angle,
                max_duty: self.left_back_leg.get_max_duty(),
                servo_name: "left_back_leg".to_string(),
            },
            ServoOperation {
                angle: front_angle,
                max_duty: self.left_front_leg.get_max_duty(),
                servo_name: "left_front_leg".to_string(),
            },
        ];

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

        self.left_back_leg.set_duty(duties["left_back_leg"])?;
        self.left_front_leg.set_duty(duties["left_front_leg"])?;

        Ok(())
    }

    /// Set front servos to specific angles using parallel execution
    #[allow(dead_code)]
    pub fn set_front_servos(&mut self, right_angle: u32, left_angle: u32) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        let operations = vec![
            ServoOperation {
                angle: right_angle,
                max_duty: self.right_front_leg.get_max_duty(),
                servo_name: "right_front_leg".to_string(),
            },
            ServoOperation {
                angle: left_angle,
                max_duty: self.left_front_leg.get_max_duty(),
                servo_name: "left_front_leg".to_string(),
            },
        ];

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

        self.right_front_leg.set_duty(duties["right_front_leg"])?;
        self.left_front_leg.set_duty(duties["left_front_leg"])?;

        Ok(())
    }

    /// Set back servos to specific angles using parallel execution
    #[allow(dead_code)]
    pub fn set_back_servos(&mut self, right_angle: u32, left_angle: u32) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        let operations = vec![
            ServoOperation {
                angle: right_angle,
                max_duty: self.right_back_leg.get_max_duty(),
                servo_name: "right_back_leg".to_string(),
            },
            ServoOperation {
                angle: left_angle,
                max_duty: self.left_back_leg.get_max_duty(),
                servo_name: "left_back_leg".to_string(),
            },
        ];

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

        self.right_back_leg.set_duty(duties["right_back_leg"])?;
        self.left_back_leg.set_duty(duties["left_back_leg"])?;

        Ok(())
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
        self.set_all_servos_angle(90)?;
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
        self.set_all_servos_angle(90)?;
        Ok(())
    }

    /// Center all servos to 90 degrees
    pub fn center_all_servos(&mut self) -> Result<()> {
        self.set_all_servos_angle(90)
    }
}

/// Maps a servo angle (0-180) to the required duty cycle value.
///
/// For 90 degrees, this should result in a 1500us (1.5ms) pulse.
/// This function is thread-safe and can be called from multiple threads.
pub fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    let rise = MAX_PULSE_US - MIN_PULSE_US;
    let run = 180 - 0;
    let pulse_us = MIN_PULSE_US + ((angle * rise) / run);

    // Convert the pulse width (us) to the LEDC duty value
    // Duty Value = (Pulse Width / Period) * Max Duty
    let duty = (pulse_us * max_duty) / PERIOD_US;

    println!(
        "[Thread {:?}] pulse_us: {}, max_duty: {}, angle: {}, duty: {}",
        thread::current().id(),
        pulse_us,
        max_duty,
        angle,
        duty
    );

    // Safety check, although calculation should prevent overflow
    core::cmp::min(duty, max_duty)
}

/// Set up servo motors and return a ServoController
pub fn setup_servos(peripherals: Peripherals) -> Result<ServoController<'static>> {
    log::info!("Setting up servo motors with parallel control capability");

    // LEDC Timer configuration
    let timer_config = TimerConfig::default()
        .frequency(Hertz(FREQUENCY).into())
        .resolution(esp_idf_hal::ledc::Resolution::Bits10);

    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

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
