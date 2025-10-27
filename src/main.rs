//! Cobot-RS Main Application
//!
//! ESP32-based 4-legged robot controller using the servo_controller module.

use anyhow::Result;
use esp_idf_hal::peripherals::Peripherals;

mod servo_controller;

use servo_controller::{demo_servo_movements, setup_servos};

fn main() -> Result<()> {
    // Initialize ESP-IDF
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting Cobot-RS with servo controller");

    // Set up servo motors
    let mut servo_controller = setup_servos(Peripherals::take().unwrap())?;

    // Run servo demonstration
    // demo_servo_movements(&mut servo_controller)?;

    log::info!("Servo setup complete, entering main loop");

    log::info!("Testing basic servo positions...");

    servo_controller.center_all_servos()?;
    esp_idf_hal::delay::FreeRtos::delay_ms(1000);

    servo_controller.set_all_servos_angle(0)?;
    esp_idf_hal::delay::FreeRtos::delay_ms(1000);

    servo_controller.set_all_servos_angle(180)?;
    esp_idf_hal::delay::FreeRtos::delay_ms(1000);

    // Test walking pattern
    // log::info!("Testing walking pattern...");
    // servo_controller.walk_forward(300)?;

    // // Test wave gesture
    // log::info!("Testing wave gesture...");
    // servo_controller.wave(50)?;

    // log::info!("Cycle complete, repeating...");
    // esp_idf_hal::delay::FreeRtos::delay_ms(3000);
    
    loop {
        // Do nothing in loop
    }
}
