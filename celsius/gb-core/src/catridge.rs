//Need to implement save to file in gb-sdl and gb-esp


//mutliple typesof mbc's
enum Mbc_type { MBC_0, MBC_1, MBC_3}

pub struct Catridge {
    rom_file:     Vec<u8>,
    rom_ext_ram:  Vec<u8>,
    rom_indx:  usize, //the multipler to shift rom banks
    rom_ext_indx:  usize,
    eram_lock:    bool,
    mbc_type:     MbcType,
    rtc_registers: [u8; 5],
    }

impl Catridge {
    pub fn new(data: Vec<u8>) -> Self{

        let mbc_type_selected = match data[0x0147] {
            0x01..=0x03 => MBC_1,
            0x0F..=0x13 => MBC_3,
            _ => MBC_0,
        };

        let ram_size = match data[0x0149] {
           0x00 => 0,
           0x01 => 0
           0x02 => 8 * 1024,
           0x03 => 32 * 1024,
           0x04 => 128 * 1024,
           0x05 => 64 * 1024,
           _ => 0,
        };

        Self {
        rom_file: data,
        rom_ext_ram: vec![0; ram_size],
        rom_indx: 1,
        rom_ext_indx: 0,
        eram_lock: true,
        mbc_type: mbc_type_selected,
        }
    }

    fn latch_clock(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.rtc_registers[0] = (now % 60) as u8;
        self.rtc_registers[1] = ((now /60 ) % 60) as u8;
        self.rtc_registers[2] = ((now / 3600) % 24) as u8;
    }

    pub fn read_rom_bank_00(&self, address: usize) -> u8 { //converted from u16 in bus
        return self.rom_file[address];
    }

    pub fn read_rom_bank_01(&self, address: usize) -> u8 { //converted from u16 in bus
        let actual_address = (self.rom_indx * 0x4000) + address;
        return self.rom_file.get(actual_address).copied().unwrap_or(0xFF)
    }

    pub fn read_rom_ext_ram(&self, address: usize) -> u8 { //converted from u16 in bus
        if !self.eram_lock && !self.rom_ext_ram.isempty() {
            
            if self.rom_ext_indx <= 0x03 {
                let actual_address = (self.rom_ext_indx * 0x2000) + address;
                return self.rom_ext_ram.get(actual_address).copied().unwrap_or(0xFF);
            } else if self.rom_ext_indx >= 0x08 && self.rom_ext_indx <= 0x0C {
                return self.rtc_registers[self.rom_ext_indx -  0x08]
            }
        else {
            return 0xFF
            }
            }
}

    pub fn write(&mut self, address: u16, value: u8){
        match address {
            //lock/unlock ram
            0x0000..=0x1FFF => {
                self.eram_lock = (value & 0x0F) != 0x0A;
            },
            //0 = 1 quirk
            0x2000..=0x3FFF => { //need to update for mbc1 and mbc3
                let mut bank = (value & 0x1F) as usize;
                if bank == 0 { bank = 1; }
                self.rom_indx = bank;
            },
            //ram bank select need to update for mbc3 rtc
            0x4000..=0x5FFF => {
                if self.mbc_type == Mbc_type::MBC_3 {
                    if value <= 0x03 || (value <= 0x0C && value >= 0x08) {
                        self.rom_ext_indx = value as usize;
                    }
                } else {
                self.rom_ext_indx = (value & 0x03) as usize;
                }
            },
            0x6000..=0x7FFF => {
                if self.mbc_type == Mbc_type::MBC_3 {
                    if value == 0x01 {
                        self.latch_clock();
                    }
                }
            },
            //writing to external ram
            0xA000..=0xBFFF => {
                if !self.eram_lock && !self.rom_ext_ram.isempty() {
                    let offset = (address - 0xA000) as usize;
                    let actual_address = (self.rom_ext_indx * 0x2000) + offset
                    if let Some(slot) = self.rom_ext_ram.get_mut(actual_address) {
                        *slot = value;
                    }
                }
            },
            _ => {},

        }
    }


    // pub fn write_save_to_disk - should be called in gb-sdl or gb-esp32 to write to disk
    // pub fn read_save_from_disk - should be called in gb-sdl or gb-esp32 to read from disk

}
