use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;

mod servo_controller;

use servo_controller::{demo_servo_movements, setup_servos};

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting Cobot-RS servo controller with parallel execution");

    // Set up servo motors
    let mut servo_controller = setup_servos(Peripherals::take().unwrap())?;

    // Run servo demonstration with parallel control
    demo_servo_movements(&mut servo_controller)?;

    log::info!("Servo setup complete, entering main loop with parallel servo control");

    // Main loop - robot behavior demonstration with parallel servo control
    loop {
        // Example 1: Walking pattern with parallel servo calculation
        log::info!("Performing walking motion with parallel servo control...");
        servo_controller.walk_forward(300)?;
        FreeRtos::delay_ms(1000);

        // Example 2: Wave gesture with threaded calculation
        log::info!("Performing wave gesture with parallel calculation...");
        servo_controller.wave(50)?;
        FreeRtos::delay_ms(1000);

        // Example 3: Side-to-side movement with parallel execution
        log::info!("Performing side movements with parallel servo control...");
        servo_controller.set_right_servos(45, 45)?;
        FreeRtos::delay_ms(500);
        servo_controller.set_left_servos(135, 135)?;
        FreeRtos::delay_ms(500);
        servo_controller.center_all_servos()?;
        FreeRtos::delay_ms(500);

        // Example 4: Diagonal stretch with parallel calculation
        log::info!("Performing diagonal stretch with parallel servo control...");
        servo_controller.set_servo_angles(30, 150, 150, 30)?;
        FreeRtos::delay_ms(1000);
        servo_controller.center_all_servos()?;
        FreeRtos::delay_ms(500);

        // Example 5: Simple alternating leg movement with parallel execution
        log::info!("Performing alternating leg movement with parallel servo control...");
        for _ in 0..3 {
            // Lift right legs - calculations done in parallel
            servo_controller.set_servo_angles(45, 90, 45, 90)?;
            FreeRtos::delay_ms(400);

            // Return to center - all servos set in parallel
            servo_controller.set_all_servos_angle(90)?;
            FreeRtos::delay_ms(400);

            // Lift left legs - calculations done in parallel
            servo_controller.set_servo_angles(90, 45, 90, 45)?;
            FreeRtos::delay_ms(400);

            // Return to center - all servos set in parallel
            servo_controller.set_all_servos_angle(90)?;
            FreeRtos::delay_ms(400);
        }

        // Example 6: All servos to different positions simultaneously
        log::info!("Testing all servos to different angles with parallel execution...");
        servo_controller.set_all_servos_angle(0)?;
        FreeRtos::delay_ms(1000);
        servo_controller.set_all_servos_angle(90)?;
        FreeRtos::delay_ms(1000);
        servo_controller.set_all_servos_angle(180)?;
        FreeRtos::delay_ms(1000);
        servo_controller.set_all_servos_angle(90)?;
        FreeRtos::delay_ms(1000);

        log::info!(
            "Behavior cycle complete with parallel servo control, waiting before next cycle..."
        );
        FreeRtos::delay_ms(5000);
    }
}
