pub struct Registers {
    pub a: u8, pub f: u8, //program status word
    pub b: u8, pub c: u8, //usualy 16-bit memory addresses
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

pub struct Cpu {
    pub regs: Registers,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            halted: false,
        }
    }

    pub fn step(&mut self, bus: &mut dyn crate::bus::Bus) {
        if self.halted { return; }

        let opcode = bus.read(self.regs.pc);
        self.regs.pc += 1;
        // ... rest of your logic

        match opcode {
            0x00 => {},
            0x3E =>{
                let value = bus.read(self.regs.pc);
                self.regs.a = value;
                self.regs.pc += 1;
            },
            // 0xAF => self.xor_a(),
            
            _ => todo!("Opcode {:#04X} not yet implemented", opcode),
        }
    }
    // ... your step and opcode functions ...
}