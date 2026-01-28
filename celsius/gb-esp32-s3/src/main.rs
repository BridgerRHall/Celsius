#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::main; // This provides the #[main] attribute
use esp_println::println;

#[main] // Replaces #[entry]
fn main() -> ! {
    // New way to initialize for v0.23+
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    
    println!("Celsius Emulator: ESP32-S3 Target Online");

    loop {
        // Emulator logic will go here
    }
}