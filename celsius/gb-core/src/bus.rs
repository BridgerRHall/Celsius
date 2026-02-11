 //bus owns components
use core::sync::atomic::{AtomicU8, Ordering};

const NINTENDO_BOOT_ROM: [u8, 256] = [0xCE, 0xE, 0xD , 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                                      0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                                      0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];

struct Memory_Map {
    
    //memory map
    // rom_bank_00:  [AtomicU8; 0x4000], //0000 - 3FFF 16kb fixed bank -  from catridge
    // rom_bank_01:  [AtomicU8; 0x4000], //4000 - 7FFF 16kb switchable bank -  via mapper (if any)
    video_ram:    [AtomicU8; 0x2000],   //8000 - 9FFF 8kb Vram -  in CGB mode is switcahble bank
    // rom_ext_ram:  [AtomicU8; 0x2000], //A000 - BFFF 8kb external ram -  from catridege switchable bank if any
    work_ram_00:  [AtomicU8; 0x1000], //C000 - CFFF 4kb work ram
    work_ram_01:  [AtomicU8; 0x1000], //D000 - DFFF 4kb work ram - incb mode switchable bank 1-7
    //echo_ram:     [AtomicU8; 0x1E00],  E000 - FDFF mirror of C000-DDFF (use prohibited)
    oam:          [AtomicU8; 0x00A0],         //FE00 - FE9F Object Attribute Memory (sprites)
    //not_usable:   [AtomicU8; 0x0060],  FEA0 - FEFF not usable (use prohibited)
    io_registers: [AtomicU8; 0x0080], //FF00 - FF7F IO registers - see line
    high_ram:     [AtomicU8; 0x007F],    //FF80 - FFFE High Ram
    intr_enbl_rg: AtomicU8,//FFFF - FFFF Interrupt enable register IE

}

struct Bus {
    memory_map: MemoryMap,
    cpu_lock: bool,
    ppu_lock: bool, //FF40
                    //
    boot_disabled: bool //FF50
    boot_rom: &'static[u8; 256],
    
    catridge: Catridge,

}


public impl Bus(){
    pub fn New() -> Self{
        Self {
            memory_map: MemoryMap::new(),
            cpu_lock: false,
            ppu_lock: false,
            eram_lock: false,
            boot_disabled: false,
            boot_rom: &NINTENDO_BOOT_ROM,
            // would be hardware accurate but adds unneccesary overhead because bool is faster
            // io_registers.store(Ordering::Relaxed, 0x004F) = 0x00,
            intr_e: false,
            intr_f: false,

            catridge: Catridge::new(),
            cpu: Cpu::new(),
            joypad: joypad::new(),
        }

    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8{
        if address < 0x0100 && !self.boot_disabled {
            return self.boot_rom[address as usize];
        }

        match address {
            //beware: self.cartrige and self.cpu need to be objects!
            //also may need to refactor to just cpu.read or ppu.read etc...
            //infact it does because i am already doing offset which
            //would be the purpose of further encapulsation/separation of
            //concerns if we can do it in the cpu to enchanes soc
            //yes put these addresses in the cpu make it care
            //THE ABOVE WOULD BE TRUE IF BUS WAS NOT GLOBAL TRANSLATOR
            //ALONG WITH THIS COMPONENTES SHOULD KNOW ABOUT ANYTHING BESIDES
            //THEMSELVES THIS IS THE BUSES JOB
            //WE ALSO WANT TO HAVE A SINGLE SOURCE OF TRUTH FOR MEMORY MAP
            //AND MEMORY MAPPING IN THE BUS
            0x0000..=0x3FFF => return self.cartridge.read_rom_bank_00(address as usize),
            0x4000..=0x7FFF => return self.catridge.read_rom_bank_01((address - 0X4000) as usize),
            0x8000..=0x9FFF => return self.ppu.read_vram((address - 0x8000) as usize),
            0xA000..=0xBFFF => return self.catridge.read_rom_ext_ram((address  - 0xA000) as usize),
            0xC000..=0xCFFF => return self.cpu.read_work_ram_01((address - 0xC000) as usize),
            0xD000..=0xDFFF => return self.cpu.read_work_ram_02((address - 0xD000) as usize),
            //mirrors function of work ram 0 and 1 (may need to change)
            0xE000..=0xEFFF => return self.cpu.read_work_ram_01((address - 0x2000) as usize),
            0xF000..=0xFDFF => return self.cpu.read_work_ram_02((address - 0x2000) as usize),
            0xFE00..=0xFE9F => return self.ppu.read_oam((address - 0xFE00) as usize),
            0xFEA0..=0xFEFF => return, //error prohibted 
            0xFF00..=0xFF7F => return //self.io.read_io(address - 0xFF00),
            0xFF80..=0xFFFE => return self.cpu.read_high_ram((address - 0xFF80) as usize),
            0xFFFF => return, //io enable registers
            _ => //default,
        }     
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8){
        //0xFF50 to turn off boot flag
        if address == 0xFF50 {
            self.boot_disabled = true;
            // would write to io_registers[0x004F] = 0x01; to turn on boot flag
            // but using bool is faster
            return;
        }
        
        //implement wrigging to other regiestrs

    }
}

