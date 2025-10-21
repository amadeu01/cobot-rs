use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver, config::TimerConfig};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;
// Servo configuration constants
const FREQUENCY: u32 = 50; // 50 Hz for servos

// Standard hobby servo constants
const MIN_PULSE_US: u32 = 500;   // Microseconds for 0 degrees (approx 0.5ms)
const MAX_PULSE_US: u32 = 2500;  // Microseconds for 180 degrees (approx 2.5ms)
const PERIOD_US: u32 = 20000;    // Microseconds for 50Hz (20ms)

/// Maps a servo angle (0-180) to the required duty cycle value.
///
/// For 90 degrees, this should result in a 1500us (1.5ms) pulse.
fn angle_to_duty(angle: u32, max_duty: u32) -> u32 {
    // 1. Calculate the required pulse width in microseconds (us)
    // The pulse width is linearly interpolated between MIN_PULSE_US (0 deg) and MAX_PULSE_US (180 deg).
    let pulse_us = MIN_PULSE_US + (angle * (MAX_PULSE_US - MIN_PULSE_US) / 180);

    // 2. Convert the pulse width (us) to the LEDC duty value
    // Duty Value = (Pulse Width / Period) * Max Duty
    // Note: All calculations must use integer arithmetic, so careful ordering is needed.
    // The division by PERIOD_US (20000) is done last to preserve precision.
    let duty = (pulse_us * max_duty) / PERIOD_US;
    
    // Safety check, although calculation should prevent overflow
    core::cmp::min(duty, max_duty) 
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
