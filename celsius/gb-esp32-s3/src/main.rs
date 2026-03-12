#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    cpu_control::{CpuControl, Stack},
    main,
    prelude::*,
};
use esp_println::println;

use core::sync::atomic{AtomicBool, Ordering};


static FRAME_READY: AtomicBool = AtomicBool::new(false); //core 1 needs to wait if display hasn't finished drawing last frame

#[main] // Replaces #[entry]
fn main() -> ! {
    // New way to initialize for v0.23+
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut cpu_control = CpuControl::new(peripherals.CPU_CTRL);
    println!("Celsius Emulator: ESP32-S3 Target Online");


    let mut core1_stack = [0u8; 8192];
    let _ = cpu_control.start_app_core(unsafe { &mut core1_stack }, move || {
        println!("core 1: starting loop");
        loop {
            FRAME_READY.store(true, Ordering::Relaxed);
        }
    });

    println!("core 0: starting system");
    loop {
        unsafe {
            if FRAME_READY.load(Ordering::Relaxed) {
                FRAME_READY.store(false, Ordering::Relaxed);
            }
        }
    }

}

