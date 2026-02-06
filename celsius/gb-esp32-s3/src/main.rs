#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::main; // This provides the #[main] attribute
use esp_println::println;

// #[main] // Replaces #[entry]
// fn main() -> ! {
//     // New way to initialize for v0.23+
//     let _peripherals = esp_hal::init(esp_hal::Config::default());
    
//     println!("Celsius Emulator: ESP32-S3 Target Online");

//     loop {
//         // Emulator logic will go here
//     }
// }


let frame_ready: bool //core 1 needs to wait if display hasn't finished drawing last frame


//need to instantiate all things i.e. catridge bus cpu

//core 1: cpu/ppu
loop {
    //read instructions
    cpu.tick(&mut bus); //cpu owns bus.  //need to keep track of cycles and pass to ppu
    //check video ram update scanlines.  //so it can keep up with cpu !!!!must be in sync!!!
    ppu.tick(&mut bus);
}

//core 0: everything else
loop {
    //do fancy stuff here
}