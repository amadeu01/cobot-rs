use anyhow::{Result};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver, config::TimerConfig};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;
// Servo configuration constants
const FREQUENCY: u32 = 50; // 50 Hz for servos

// Function to calculate duty cycle for a given angle
fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    let pulse_width_us = 500 + (angle as u64 * (2500 - 500) / 180);
    (pulse_width_us * max_duty as u64 / 20000) as u32
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    // Function to set up servo motors
    
    setup_servos(Peripherals::take().unwrap())?;

    loop {
        FreeRtos::delay_ms(1000);
    }
}

fn setup_servos(peripherals: Peripherals) -> Result<()> {
        // LEDC Timer configuration
        let timer_config = TimerConfig::default()
            .frequency(Hertz(FREQUENCY).into())
            .resolution(esp_idf_hal::ledc::Resolution::Bits10);

        let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

        let mut right_back_leg =
            LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio23)?;

        let mut left_back_leg =
            LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio22)?;

        let mut right_front_leg =
            LedcDriver::new(peripherals.ledc.channel2, &timer, peripherals.pins.gpio19)?;

        let mut left_front_leg =
            LedcDriver::new(peripherals.ledc.channel3, &timer, peripherals.pins.gpio18)?;

        right_back_leg.set_duty(angle_to_duty(90, right_back_leg.get_max_duty()))?;
        left_back_leg.set_duty(angle_to_duty(90, left_back_leg.get_max_duty()))?;
        right_front_leg.set_duty(angle_to_duty(90, right_front_leg.get_max_duty()))?;
        left_front_leg.set_duty(angle_to_duty(90, left_front_leg.get_max_duty()))?;

        log::info!("Servos initialized to 90 degrees.");

        Ok(())
    }
