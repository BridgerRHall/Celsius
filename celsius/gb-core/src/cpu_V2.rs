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
    stopped: bool,
    halted: bool,
    ime: bool,


//----------------------------------------


}

impl Cpu {
    pub fn new() ->Self{
        Self {
            a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            sp: 0xFFFE,
            pc: 0x0100,
            stopped: false,
            halted: false,
            ime: false
        }

    }

    fn step(&mut self, bus: &mut Bus) -> u32 {

        if self.stopped {
            return 4;
        }

        let opcode: u8 = bus.read(self.pc);
        self.pc += 1;

        match opcode {
            0x00 => {
                4 //NOP
            } 
            0x01 => { //LD BC, d16
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;
                self.set_bc(((high as u16) << 8) | low as u16);
                12
            }
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                let reg_id = opcode >> 3;
                let value = self.get_register_val(bus, reg_id);
                let new_value = self.inc_8(bus, value);
                self.set_register_val(bus, reg_id, new_value);

                if reg_id == 6 { 12 } else { 4 }
            }
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                let reg_id = opcode >> 3;
                let value = self.get_register_val(bus, reg_id);
                let new_value = self.dec_8(bus, value);
                self.set_register_val(bus, reg_id, new_value);

                if reg_id == 6 { 12 } else { 4 }
            }
            0x10 => {
                let unused_byte = bus.read(self.pc);
                self.pc+=1;
                self.stopped = true; //STOP 2bytes 4 cycles
                4
            }
            0x11 => { //LD de, d16
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;
                self.set_de(((high as u16) << 8) | low as u16);
                12
            }
            0x18 => { //jr e
                let offset = bus.read(self.pc) as i8;
                self.pc +=1;
                self.pc = (self.pc as i16 + offset as i16) as u16;
                12
            }
            0x20 => { //jr == 0 if not 0
                let offset = bus.read(self.pc) as i8;
                self.pc += 1;

                let zero_flag = (self.f >> 7 & 1);

                if zero_flag == 0 {
                    self.pc = (self.pc as i16 + offset as i16) as u16;
                    12
                } else {
                    8
                }
            }
            0x21 => { //LD hl, d16
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;
                self.set_hl(((high as u16) << 8) | low as u16);
                12
            }
            0x28 => { //jr z== 1 if 0
                let offset = bus.read(self.pc) as i8;
                self.pc += 1;

                let zero_flag = (self.f >> 7 & 1);

                if zero_flag != 0 {
                    self.pc = (self.pc as i16 + offset as i16) as u16;
                    12
                } else {
                    8
                }
            }
            0x30 => { //jr c == 0 if no carry
                let offset = bus.read(self.pc) as i8;
                self.pc +=1;

                let carry_flag = (self.f >> 4 & 1);

                if carry_flag == 0 {
                    self.pc = (self.pc as i16 + offset as i16) as u16;
                    12
                } else {
                    8
                }
            }
            0x31 => { //LD sp, d16
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;
                self.sp = (((high as u16) << 8) | low as u16);
                12
            }
            0x38 => { //jr c == 1 if carry
                let offset = bus.read(self.pc) as i8;
                self.pc +=1;

                let carry_flag = (self.f >> 4 & 1);

                if carry_flag != 0 {
                    self.pc = (self.pc as i16 + offset as i16) as u16;
                    12
                } else {
                    8
                }
            }
            0x40..=0x7F => {
                if opcode == 0x76 {
                    self.halted = true;
                    4
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
                    6 => bus.write(self.get_hl(), value),
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
            0xC1 | 0xD1 | 0xE1 | 0xF1 => { //pop
                let low = bus.read(self.sp);
                self.sp += 1;
                let  high = bus.read(self.sp);
                self.sp += 1;

                let value = (((high as u16) << 8) | low as u16);

                match opcode {
                    0xC1 => self.set_bc(value),
                    0xD1 => self.set_de(value),
                    0xE1 => self.set_hl(value),
                    0xF1 => {
                        self.a = (value >> 8) as u8;
                        self.f = (value & 0xF0) as u8;
                    }
                    _ => unreachable!(),
                }
                12
            }
            0xC2 => {
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;

                let zero_flag = (self.f >> 7) & 1;

                if (zero_flag == 0){
                    let address = ((high as u16) << 8) | (low as u16);
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xC3 => {
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;
                let address = ((high as u16) << 8) | (low as u16);
                self.pc = address;
                16
            }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => { //push
                let value = match opcode {
                    0xC5 => self.get_bc(),
                    0xD5 => self.get_de(),
                    0xE5 => self.get_hl(),
                    0xF5 => ((self.a as u16) << 8) | (self.f as u16);
                    _ => unreachable!(),
                };
                self.sp -= 1;
                bus.write(self.sp, (value >> 8) as u8);
                self.sp -= 1;
                bus.write(self.sp, (value & 0xFF) as u8);
                16
            }
            0xCA => {
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;

                let zero_flag = (self.f >> 7) & 1;
                if (zero_flag != 0){
                    let address = ((high as u16) << 8) | (low as u16);
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xCB => {
                let cb_opcode = bus.read(self.pc);
                self.pc +=1;

                let cb_reg = cb_opcode & 0x07; //which reg
                let cb_bit = (cb_opcode >> 3) & 0x07; //which bit 
                let cb_group = cb_opcode >> 6; //which block;

                //get val
                let mut value = self.get_register_val(bus, cb_reg);

                //do op
                match cb_group {
                    0 => value = self.cb_shift_rotate(value , cb_bit),
                    1 => self.cb_bit_test(value , cb_bit), //just changes flag
                    2 => value  &= !(1 << cb_bit),
                    3 => value |= 1 << cb_bit,
                    _ => unreachable!(),

                }

                if cb_group != 1 {
                    self.set_register_val(bus, cb_reg, value);
                }

                if cb_reg == 6 { 16 } else { 8 }
            }
            0xCD => { // call nn
                let target_address = self.read_u16(bus);

                self.sp = self.sp.wrapping_sub(1);
                bus.write(self.sp, (self.pc >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                bus.write(self.sp, (self.pc & 0xFF) as u8);

                self.pc = target_address;
                24
            }
            0xC9 => { //ret unconditional
                let low = bus.read(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);
                let  high = bus.read(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);

                self.pc = (high << 8) | low;
                16
            }
            0xD2 => {
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;

                let carry_flag = (self.f >> 4) & 1;

                if (carry_flag == 0){
                    let address = ((high as u16) << 8) | (low as u16);
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xDA => {
                let low = bus.read(self.pc);
                self.pc += 1;
                let  high = bus.read(self.pc);
                self.pc += 1;

                let carry_flag = (self.f >> 4) & 1;

                if (carry_flag != 0){
                    let address = ((high as u16) << 8) | (low as u16);
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xE0 => {
                let n = bus.read(self.pc) as u16;
                self.pc += 1;
                bus.write(0xFF00 | n, self.a);
                12
            }
            0xF0 => {
                let n = bus.read(self.pc) as u16;
                self.pc += 1;
                self.a = bus.read(0xFF00 | n);
                12
            }
            0xEA => {
                let low = bus.read(self.sp);
                self.sp += 1;
                let  high = bus.read(self.sp);
                self.sp += 1;

                let address = (((high as u16) << 8) | low as u16);
                bus.write(address, self.a);
                16
            }
            0xFA => {
                let low = bus.read(self.sp);
                self.sp += 1;
                let  high = bus.read(self.sp);
                self.sp += 1;

                let address = (((high as u16) << 8) | low as u16);
                self.a = bus.read(address);
                16
            }
            0xE2 => {
                bus.write(0xFF00 | (self.c as u16), self.a);
                8
            }
            0xF2 => {
                self.a = bus.read(0xFF00 | (self.c as u16));
                8
            }
            0xE9 => {
                self.pc = self.get_hl();
                4
            }
            _ => 4,
        }
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_bc(&mut self, value: u16){
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }
    fn set_de(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }
    fn set_hl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
    
    fn cb_bit_test(&mut self, value: u8, bit: u8){
                let is_set = (value & (1 << bit)) != 0;

                //flags
                let z = if !is_set { 0x80 } else { 0 };
                let n = 0x00;
                let h = 0x20;
                let c = self.f & 0x10;

                self.f = z | n | h | c;
    }

    fn cb_shift_rotate(&mut self, value: u8, bit: u8) -> u8 {
        match bit {
            0 => self.cb_rlc(value),
            1 => self.cb_rrc(value),
            2 => self.cb_rl(value),
            3 => self.cb_rr(value),
            4 => self.cb_sla(value),
            5 => self.cb_sra(value),
            6 => self.cb_swap(value),
            7 => self.cb_srl(value),
            _ => unreachable!(),
        }
    }

    fn cb_rlc(&mut self, value) -> u8 {
        let bit7 = (value >> 7) & 1;
        let result = (value << 1) | bit7;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    fn cb_rrc(&mut self, value) -> u8 {
        let bit0 = (value & 1);
        let result = (value >> 1) | (bit0 << 7);
        self.set_flags(result, 0, 0, bit0);
        result
    }
    fn cb_rl(&mut self, value) -> u8 {
        let old_carry = (self.f >> 4) & 1;
        let bit7 = (value >> 7) & 1;
        let result = (value << 1) | old_carry;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    fn cb_rr(&mut self, value) -> u8 {
        let old_carry = (self.f >> 4) & 1;
        let bit0 = value & 1;
        let result = (value >> 1) | (old_carry << 7);
        self.set_flags(result, 0, 0, bit0);
        result
    }
    fn cb_sla(&mut self, value) -> u8 {
        let bit7 = value >> 7;
        let result = value << 1;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    fn cb_sra(&mut self, value) -> u8 {
        let bit0 = value & 1;
        let result = (value >> 1) | (value & 0x80);
        self.set_flags(result, 0, 0, bit0);
        result
    }
    fn cb_swap(&mut self, value) -> u8 {
        let result = (value >> 4) | (value << 4);
        self.f = if result == 0 { 0x80 } else { 0 };
        result
    }
    fn cb_srl(&mut self, value) -> u8 {
        let bit0 = value & 1;
        let result = value >> 1;
        self.set_flags(result, 0, 0, bit0);
        result
    }
    fn set_flags(&mut self, res: u8, n: u8, h: u8, c: u8){
        let z_bit = if res == 0 { 0x80 } else { 0 };
        let n_bit = if n != 0 { 0x40 } else { 0 };
        let h_bit = if h != 0 { 0x20 } else { 0 };
        let c_bit = if c != 0 { 0x10 } else { 0 };

        self.f = z_bit | n_bit | h_bit | c_bit;
    }

    fn get_register_val(&mut self, bus: &mut Bus, id: u8) -> u8 {
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

    fn set_register_val(&mut self, bus: &mut Bus, id: u8, value: u8) {
        match id {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => bus.write(self.get_hl(), value),
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

    fn inc_8(&mut self, bus: &mut Bus, value u8) -> u8 {
        let result = value.wrapping_add(1);

        let z = if result == 0 { 0x80 } else { 0 };
        let n = 0x00;
        let h = if ( value & 0x0F ) + 1 > 0x0F { 0x20 } else { 0 };
        let c = self.f & 0x10;

        self.f = z | n | h | c;
        result
    }

    fn dec_8(&mut self, bus: &mut Bus, value u8) -> u8 {
        let result = value.wrapping_sub(1);

        let z = if result == 0 { 0x80 } else { 0 };
        let n = 0x40;
        let h = if ( value & 0x0F ) == 0 { 0x20 } else { 0 };
        let c = self.f & 0x10;

        self.f = z | n | h | c;
        result
    }

}