#![no_std]
#![no_main]

// Import the panic handler (required for no_std)
use esp_backtrace as _;
// Import everything we need from esp-hal
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // 1. Initialize peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // 2. Initialize the Delay driver
    let delay = Delay::new();

    // 3. Set up the GPIO pins
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    // Adjust 'gpio8' to match the LED pin on your specific board
    let mut led = Output::new(io.pins.gpio8, Level::Low);

    // Optional: Initialize logger if you have esp-println in Cargo.toml
    // esp_println::logger::init_logger_from_env();

    loop {
        led.set_low();
        delay.delay_millis(1000);
       
        led.set_high();
        delay.delay_millis(500);

        led.set_low();
        delay.delay_millis(1000);
    }
}