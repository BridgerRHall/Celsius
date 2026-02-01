use crate::bus::Bus; // Only one of these!

pub struct Mmu<'a> {
    pub rom: &'a [u8],
    pub wram: [u8; 8192],
    pub hram: [u8; 127],
}

impl<'a> Mmu<'a> {
    pub fn new(rom: &'a [u8]) -> Self {
        Self {
            rom,
            wram: [0; 8192],
            hram: [0; 127],
        }
    }
}

// Ensure ALL trait functions are inside this block
impl<'a> Bus for Mmu<'a> {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            _ => 0xFF,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            _ => {} 
        }
    }
}