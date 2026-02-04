// FF00
// bit 0: A or Right -> 0 is pressed
// bit 1  B or Left -> 0 is pressed
// bit 2  Select or Up -> 0 is pressed
// bit 3  Start or Down -> 0 is pressed
// bit 4  Select D-pad -> 0 is D-pad active
// bit 4  Select Buttons -> 0 is Buttons active
// bit 6 unused
// bit 7 unused

// joypad intterup at $0060 whenever a button bit transitions for 1 to 0
//will use two u8 to reconunstruct a proper u8 so writes don't wipe button presses

//core 0 cpu will read button state core 1 display/input will write
//to do this safely use atomic U

//importent must determine ordering during use to avoid problems
use core::sync::atomic::{AtomicU8, Ordering};


struct Joypad {
    //bit 4 = 0 buttons, bit 5 = 0 dpad
    pub registers: AtomicU8, // stores election bits p14 and p15
    // 7 down, 6 up, 5 left, 4 right, 3 start, 2 select, 1 b, 0 a
    pub state: AtomicU8, //stores acual butotn press bits
                    //we then recreate actual FF00 on the fly using bitmasking
}

impl Joypad {
    pub fn new() -> Self{
       Self {
            registers: AtomicU8::new(255), //11111111
            state: AtomicU8::new(0xFF), //11111111 1 is not pushed
        }
    }

    pub fn read(&self) -> u8{ //& to borrow the instance for the duration of the function
        //self.registers = 0001 0000 or 0010 0000
        //use load to load use store to store ordering relaxed for joypad
        let mskd_rgstrs = self.registers.load(Ordering::Relaxed) & 0b_1111_0000; // clean extra bits
        let shftd_rgstrs = mskd_rgstrs >> 4; //now a nibble with  1110 btns or 1101 dpd
        
        let mut value = 0u8;

        let stt = self.state.load(Ordering::Relaxed);

        match shftd_rgstrs {
            //14 buttons
            0b_1110 => value = (shftd_rgstrs << 4 ) | (stt & 0b_0000_1111),
            //13 dpad
            0b_1101 => value = (shftd_rgstrs << 4 ) | ((stt & 0b_1111_0000) >> 4),
            //12 both
            0b_1100 => {
                let btns = stt & 0b_0000_1111;
                let dpd = (stt & 0b_1111_0000) >> 4;
                value = (shftd_rgstrs << 4) | (btns & dpd);
            },

            //15 or anything else
            _ => value = (shftd_rgstrs << 4) | 0b_0000_1111,
        }

        return (value | 0b_1100_0000);

    }

    pub fn register_write(&self, value: u8) {
        let selection = (value & 0b_0011_0000) | 0b_1100_0000; 
        self.registers.store(selection, Ordering::Relaxed);
        
    }

    pub fn state_write(&self, value: u8){
        self.state.store(value, Ordering::Relaxed);
    }

}


