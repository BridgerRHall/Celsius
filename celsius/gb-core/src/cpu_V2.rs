struct Cu {

}

impl Cu {
    pub new() ->Self{

    }
}
struct Rf {

}

impl Rf {
    pub new() ->Self{

    }
}
struct Idu {

}

impl Idu {
    pub new() ->Self{

    }
}



struct Cpu {

//RF--------------------------------------
    a: u8, f: u8, //z = zero flag
    b: u8, c: u8, //c = carry flag
    d: u8, e: u8, //n = subtraction flag
    h: u8, l: u8, //h = half carry flag
    sp: u16,
    pc: u16,

//----------------------------------------


}

impl Cpu {
    pub fn new() ->Self{
        stopped: bool = false;
        halted: bool = false;

    }

    fn step(&mut self, bus: &mut Bus) -> u32 {

        if stopped {
            return 4;
        }

        let opcode: u8 = bus.read(pc)
        pc += 1

        match opcode {
            0x00 => {
                4 //NOP
            } 
            0x10 => {
                let unused_byte = bus.read(pc);
                pc+=1;
                self.stopped = true; //STOP 2bytes 4 cycles
                4
            }
            0x40..=0x7F => {
                if opcode == 0x76 {
                    self.halted = true;
                    return 4;
                }  

                //bitmask and extract values
                let src_id = opcode & 0x07; //00000111
                let dest_id = (opcode >> 3) & 0x7;

                let value = match src_id {
                    0 => self.b,
                    1 => self.c,
                    2 => self.d,
                    3 => self.e,
                    4 => self.h,
                    5 => self.l,
                    6 => bus.read(self.get_hl()),
                    7 => self.a,
                    _ => unreachable!(),
                };

                match dest_id {
                    0 => self.b = value,
                    1 => self.c = value,
                    2 => self.d = value,
                    3 => self.e = value,
                    4 => self.h = value,
                    5 => self.l = value,
                    6 => bus.write(self.get_hl(), val),
                    7 => self.a = value,
                    _ => unreachable!(),
                }

                //return ticks
                if src_id == 6 || dest_id == 6 {
                    8
                } else {
                    4
                }

            }
            0x80..=0xBF => {
                
                let src_id = opcode & 0x7;
                let op_id = (opcode >> 3) & 0x07;
                let value = self.get_register_val(bus, src_id);

                //match op_id to determine operation
                self.alu(value, op_id);

                //return ticks
                if src_id == 6 {
                    8
                } else {
                    4
                }

            }
            0x3E => { //LD A, n
                self.a = bus.read(self.pc);
                self.pc +=1;
                8 //2byte instruction
            }
            0xCB => {
                let cb_opcode = bus.read(self.pc);
                self.pc +=1;

                let cb_reg = cb_opcode & 0x07; //which reg
                let cb_bit = (cb_opcode >> 3) & 0x07; //which bit 
                let cb_group = cb_opcode >> 6; //which block;

                //get val
                let value = self.get_register_val(bus, cb_reg);

                //do op
                match group {
                    0 => val = self.cb_shift_rotate(val, cb_bit),
                    1 => self.cb_bit_test(val, cb_bit), //just changes flag
                    2 => val &= !(1 << bit_index),
                    3 => val |= 1 << bit_index,
                    _ => unreachable!(),

                }

                if group != 1 {
                    self.set_register_val(bus, cb_reg, value);
                }

                if cb_reg == 6 { 16 } else { 8 };
            }
            _ => 4,
            //8bit loads
            //16bit loads
            //8bit alu
            //16bit arithmetic
            //misc
            //rotates & shifts
            //bit opcodes
            //jumps
            //calls
            //restarts
            //returns





            

        }
    }

    fn get_bc(&mut self) -> u16 {
        let bc = ((self.b as u16) << 8) | (self.c as u16);
    }
    fn get_de(&mut self) -> u16 {
        let de = ((self.d as u16) << 8) | (self.e as u16);
    }
    fn get_hl(&mut self) -> u16 {
        let hl = ((self.h as u16) << 8) | (self.l as u16);
    }

    fn set_bc(&mut self, val: u16){
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    fn set_de(&mut self, val: u16){
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    fn set_hl(&mut self, val: u16){
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }
    
    fn get_register_val(&self, bus: &mut Bus, id: u8) -> u8 {
        match id {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => bus.read(self.get_hl()),
            7 => self.a,
            _ => unreachable!(),
        }
    }

    fn set_register_val(&self, bus: &mut Bus, id: u8, value: u8) {
        match id {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => bus.write(self.set_hl(), value),
            7 => self.a = value,
            _ => unreachable!(),
        }
    }

    fn alu(&mut self, value: u8, arithmetic: u8) {
        match arithmetic {
            0x00 => { //add
                let a = self.a;
                let sum = (a as u16) + (value as u16);
                let result = sum as u8;

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x00;
                let h = if (a & 0x0F) + (value & 0x0F) > 0x0F { 0x20 } else { 0 };
                let c = if sum > 0xFF { 0x10 } else { 0 };

                self.f = z | n | h | c;
                self.a = result;

            },
            0x01 => { //adc
                let carry_bit = (self.f >> 4 & 0x01);
                let a = self.a;

                let sum = (a as u16) + (value as u16) + (carry_bit as u16);
                let result = sum as u8;

                //z
                let z = if result == 0 { 0x80 } else { 0 };
                //sub
                let n = 0x00;
                //half carry
                let h = if (((a & 0x0F) + (value & 0x0F) + carry_bit) > 0x0F) { 0x20 } else{ 0 };
                //carry 
                let c = if sum > 0xFF { 0x10 } else { 0 };

                self.f = z | n | h | c;
                self.a = result;
            },
            0x02 => { //sub
                let a = self.a;
                let difference = (a as u16) - (value as u16);
                let result = difference as u8;

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x40;//bit 6
                let h = if (value & 0x0F) > (a & 0x0F) { 0x20 } else { 0 };
                let c = if value > a { 0x10 } else { 0 };

                self.f = z | n | h | c;
                self.a = result;

            },
            0x03 => { //sbc
                let carry_bit = (self.f >> 4 & 0x01);
                let a = self.a;
                let difference = (a as u16) - (value as u16) - (carry_bit as u16);
                let result = difference as u8;

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x40;//bit 6
                let h = if (a & 0x0F) < ((value & 0x0F) + (carry_bit)) { 0x20 } else { 0 };
                //could overflow so set to u16 because need to check u8
                let c = if (value as u16) + (carry_bit as u16) > (a as u16) { 0x10 } else { 0 };

                self.f = z | n | h | c;
                self.a = result;

            },
            0x04 => { //and
                let result = self.a & value;

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x00;//bit 6
                let h = 0x20;
                let c = 0x00;

                self.f = z | n | h | c;
                self.a = result;

            },
            0x05 => { //xor
                let result = self.a ^ value;

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x00;//bit 6
                let h = 0x00;
                let c = 0x00;

                self.f = z | n | h | c;
                self.a = result;

            },
            0x06 => { //or  
                let result = self.a | value; //single pipe = bitwise or double pipe is boolean or

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x00;//bit 6
                let h = 0x00;
                let c = 0x00;

                self.f = z | n | h | c;
                self.a = result;
            },
            0x07 => { //cp
                let a = self.a;
                let result = a.wrapping_sub(value);

                //flags
                let z = if result == 0 { 0x80 } else {0};
                let n = 0x40;//bit 6
                let h = if (value & 0x0F) > (a & 0x0F) { 0x20 } else { 0 };
                let c = if value > a { 0x10 } else { 0 };

                self.f = z | n | h | c;
            },
        }
    }
}