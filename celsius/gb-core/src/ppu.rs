
// 0. H-BLANK (END OF LINE) 00, 1. V-BLANK (END OF FRAME) 01 2. OAM SEARCH 10, 3. PIXEL TRANSER 11
pub enum Mode {MODE_0, MODE_1, MODE_2, MODE_3}


pub struct Ppu {
    pub screen_data: [u8; 160 * 144],

    //for all register bus will update these when actual register in main memory changes these are to reduce overhead (theoretically)
    pub ppu_mode: Mode, //$FF41 stat register bits 0 and 1
    pub ppu/lcd_is_enabled: Bool, //$FF40 - based off of bit7 at mem addr $FF40 set by cpu 0 or 1 and modified by bus if 0 lv is 0 mode is hblank no rendering many timing rules change
    pub ly: Int, //$FF44 -LCD Y coord r-only value from 0-153 values from 144-153 indicate v-blank peroid
    pub lyc: Int,//compares value of lyc and ly (lyc==ly) when both are idential the STAT register is set and if enable a STAT interrupt is requested

    //STAT LCD status 1byte: $FF41 
    //bit 0/1 PPU mode - read only - reports ppu mode  bit 1: 0 bit 0: 0 = Mode 0 HBlank; bit 1: 0 bit 0: 1 = Mode 1 VBlank; bit 1: 1 bit 0: 0 = Mode 2 OAM Scan; bit 1: 1 bit 0: 1 = Mode 3 Drawing
    //bit 2 LYC=LY - read only - reports 1 if LYC=LY
    //bit 3 Mode 0 - read/write  - if 1 sets mode 0 for stat intterupt
    //bit 4 Mode 2 - read/write  - if 1 sets mode 1 for stat intterupt
    //bit 5 Mode 3 - read/write  - if 1 sets mode 2 for stat intterupt
    //bit 6 LYC INT - read/write - if 1 sets LYC=LY condition for STAT intterupt
0


}

impl Ppu {
    pub fn new() -> Self {
        Self {
            screen_data: [0; 160 * 144],
        }
    }

    pub fn step() -> self {

    }
}