#![no_std] // No standard library for ESP32 compatibility

// Use 'alloc' if you need things like Box, but avoid it if possible for speed
// extern crate alloc; 

#[cfg(test)]
extern crate std; // Only use std during 'cargo test' on your Mac

pub mod cpu;
pub mod mmu;
pub mod ppu;
pub mod bus; // Move your busTrait.rs here as bus.rs

pub struct GameBoy<'a> {
    pub cpu: cpu::Cpu,
    pub mmu: mmu::Mmu<'a>,
    pub ppu: ppu::Ppu,
}

impl<'a> GameBoy<'a> {
    pub fn new(rom: &'a [u8]) -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            mmu: mmu::Mmu::new(rom),
            ppu: ppu::Ppu::new(),
        }
    }

    pub fn step(&mut self) {
        // The main emulation heart-beat
        self.cpu.step(&mut self.mmu);
    }
}