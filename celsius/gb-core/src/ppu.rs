pub struct Ppu {
    pub screen_data: [u8; 160 * 144],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            screen_data: [0; 160 * 144],
        }
    }
}