use std::{thread, time::Duration};
use esp_idf_hal::peripherals::Peripherals; // Using esp_idf_hal directly is cleaner
use esp_idf_hal::gpio::PinDriver;

fn main() -> anyhow::Result<()> {
    // REQUIRED: Links the ESP-IDF patches so the system boots correctly
    esp_idf_sys::link_patches();

    // Take hardware peripherals
    let peripherals = Peripherals::take().map_err(|e| anyhow::anyhow!("Failed to take peripherals: {:?}", e))?;
    let pins = peripherals.pins;

    // Configure GPIO5 as an output
    let mut led = PinDriver::output(pins.gpio8)?;

    println!("Blinking started on GPIO 8!");

    loop {
        // Long blink
        led.set_high()?; 
        println!("LED ON (Long)");
        thread::sleep(Duration::from_millis(1000));
        
        led.set_low()?; 
        println!("LED OFF");
        thread::sleep(Duration::from_millis(1000));

        // Short blink
        led.set_high()?; 
        println!("LED ON (Short)");
        thread::sleep(Duration::from_millis(500));
        
        led.set_low()?; 
        println!("LED OFF");
        thread::sleep(Duration::from_millis(500));
    }
}