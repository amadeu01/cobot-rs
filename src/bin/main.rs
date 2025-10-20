#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::Io;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;
// use esp_hal::ledc::{Ledc, LedcTimer, LedcTimerDriver};
// use esp_hal::prelude::*;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    // let io = Io::new(_peripherals.GPIO, _peripherals.IO_MUX);

    // // Initialize LEDC peripheral for PWM
    // let mut ledc = Ledc::new(_peripherals.LEDC);
    // ledc.set_global_slow_clock(esp_hal::ledc::LSGlobalClkSource::APBClk);

    // let mut lstimer0 = ledc.timer::<esp_hal::ledc::LowSpeed>(esp_hal::ledc::timer::Number::Timer0);
    // lstimer0
    //     .configure(esp_hal::ledc::timer::config::Config {
    //         duty: esp_hal::ledc::timer::config::Duty::Duty14Bit,
    //         clock_source: esp_hal::ledc::LSClockSource::APBClk,
    //         frequency: 50u32.Hz(),
    //     })
    //     .unwrap();

    // // Configure servo pins
    // let right_back_leg_pin = io.pins.gpio23;
    // let left_back_leg_pin = io.pins.gpio22;
    // let right_front_leg_pin = io.pins.gpio19;
    // let left_front_leg_pin = io.pins.gpio18;
    //
    // let mut right_back_servo = ledc
    //     .channel(esp_hal::ledc::channel::Number::Channel0, right_back_leg_pin)
    //     .configure(esp_hal::ledc::channel::config::Config {
    //         timer: &lstimer0,
    //         duty_pct: 7, // ~90 degrees (1.5ms pulse width)
    //         pin_config: esp_hal::ledc::channel::config::PinConfig::PushPull,
    //     })
    //     .unwrap();

    println!("####");
    println!("Hello world!");
    println!("Hello world!");
    println!("Hello world!");
    println!("Hello world!");
    println!("####");

    loop {
        // Create PWM channels for each servo

        // let mut left_back_servo = ledc
        //     .channel(esp_hal::ledc::channel::Number::Channel1, left_back_leg_pin)
        //     .configure(esp_hal::ledc::channel::config::Config {
        //         timer: &lstimer0,
        //         duty_pct: 7, // ~90 degrees (1.5ms pulse width)
        //         pin_config: esp_hal::ledc::channel::config::PinConfig::PushPull,
        //     })
        //     .unwrap();

        // let mut right_front_servo = ledc
        //     .channel(esp_hal::ledc::channel::Number::Channel2, right_front_leg_pin)
        //     .configure(esp_hal::ledc::channel::config::Config {
        //         timer: &lstimer0,
        //         duty_pct: 7, // ~90 degrees (1.5ms pulse width)
        //         pin_config: esp_hal::ledc::channel::config::PinConfig::PushPull,
        //     })
        //     .unwrap();

        // let mut left_front_servo = ledc
        //     .channel(esp_hal::ledc::channel::Number::Channel3, left_front_leg_pin)
        //     .configure(esp_hal::ledc::channel::config::Config {
        //         timer: &lstimer0,
        //         duty_pct: 7, // ~90 degrees (1.5ms pulse width)
        //         pin_config: esp_hal::ledc::channel::config::PinConfig::PushPull,
        //     })
        //     .unwrap();

        println!("Servos set to 90 degrees");

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
