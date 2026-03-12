use esp_hal::ram;

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

    #[ram]
    fn step(&mut self, bus: &mut Bus) -> u32 {

        if self.stopped {
            return 4;
        }

        if self.halted {
            let interrupt_flag = bus.read(0xFF0F);
            let interrupt_enable = bus.read(0xFFFF);

            if (interrupt_flag & interrupt_enable & 0x1F) != 0 {
                self.halted = false;
            } else {
                return 4;
            }
        }

        if self.ime {
            let interrupt_flag = bus.read(0xFF0F);
            let interrupt_enable = bus.read(0xFFFF);
            let pending = interrupt_flag & interrupt_enable & 0x1F;

            if pending != 0 {
                self.execute_interrupt(bus, pending);
                return 20;
            }
        }

        let opcode: u8 = bus.read(self.pc);
        self.pc += 1;

        match opcode {
            0x00 => {
                4 //NOP
            } 
            0x01 => { //LD BC, d16
                self.set_bc(self.read_u16(bus));
                12
            }
            0x03 | 0x13 | 0x23 | 0x33 => {
                match opcode {
                    0x03 => self.set_bc(self.get_bc().wrapping_add(1)), //inc bc
                    0x13 => self.set_de(self.get_de().wrapping_add(1)), //inc de
                    0x23 => self.set_hl(self.get_hl().wrapping_add(1)),
                    0x33 => self.sp = self.sp.wrapping_add(1),
                    _ => unreachable!(),
                }
                8
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
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => {
                let value = bus.read(self.pc);

                match opcode {
                    0x06 => self.b = value,
                    0x0E => self.c = value,
                    0x16 => self.d = value,
                    0x1E => self.e = value,
                    0x26 => self.h = value,
                    0x2E => self.l = value,
                    _ => unreachable!(),
                }
                8
            }
            0x07 | 0x0F | 0x17 | 0x1F => {
                let old_address = self.a;
                let old_carry = (self.f >> 4) & 1;

                match opcode {
                    0x07 => {
                        let bit7 = (old_address >> 7) & 1;
                        self.a = (old_address << 1) | bit7;
                        self.f = if bit7 != 0 { 0x10 } else { 0 };

                    }
                    0x0F => {
                        let bit0 = (old_address & 1);
                        self.a = (old_address >> 1) | (bit0 << 7);
                        self.f = if bit0 == 0 { 0x10 } else { 0 };
                    }
                    0x17 => {
                        let bit7 = (old_address >> 7) & 1;
                        self.a = (old_address << 1) | old_carry;
                        self.f =  if bit7 != 0 { 0x10 } else { 0 };
                    }
                    0x1F => {
                        let bit0 = (old_address & 1);
                        self.a = (old_address >> 1) | (old_carry << 7);
                        self.f = if bit0 != 0 { 0x10 } else { 0 };

                    }
                    _ => unreachable!(),
                }
            }
            0x09 | 0x19 | 0x29 | 0x39 => {
                let hl = self.get_hl();
                let value = match opcode {
                    0x09 => self.get_bc(),
                    0x19 => self.get_de(),
                    0x29 => hl,
                    0x39 => self.sp,
                    _ => unreachable!(),
                };
                let result = hl.wrapping_add(value);

                let n = 0x00;
                let h = if (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF { 0x20 } else { 0 };
                let c = if (hl as u32) + (value as u32) > 0xFFFF { 0x10 } else { 0 }; //bitwise math not if statements to optimize branch prediction?

                self.f = (self.f & 0x80) | n | h | c;

                self.set_hl(result);
                8

            }
            0x0B | 0x1B | 0x2B | 0x3B => {
                match opcode {
                    0x0B => self.set_bc(self.get_bc().wrapping_sub(1)), //sub bc
                    0x1B => self.set_de(self.get_de().wrapping_sub(1)),
                    0x2B => self.set_hl(self.get_hl().wrapping_sub(1)),
                    0x3B => self.sp = self.sp.wrapping_sub(1),
                    _ => unreachable!(),
                }
                8
            }
            0x10 => {
                let unused_byte = bus.read(self.pc);
                self.pc+=1;
                self.stopped = true; //STOP 2bytes 4 cycles
                4
            }
            0x11 => { //LD de, d16
                self.set_de(self.read_u16(bus));
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
                self.set_hl(self.read_u16(bus));
                12
            }
            0x22 => { //ldi hl
                let hl = self.get_hl();
                bus.write(hl, self.a);
                self.set_hl(hl.wrapping_add(1));
                8
            }
            0x27 => {
                let mut a = self.a as u16;
                let n_flag = (self.f & 0x40) != 0;
                let h_flag = (self.f & 0x20) != 0;
                let c_flag = (self.f & 0x10) != 0;

                if !n_flag {
                    if h_flag || a > 0x9F {
                        a += 0x06;
                    }
                    if c_flag  || (a & 0x9F) > 0x99 {
                        a += 0x60;
                        self.f |= 0x10; //set carry

                    }
                } else {
                    if h_flag {
                        a = a.wrapping_sub(0x06);
                    }
                    if c_flag {
                        a = a.wrapping_sub(0x60);
                    }
                }

                self.a = a as u8;
                let z = if self.a == 0 { 0x80 } else { 0 };
                self.f = (self.f & 0x50) | z;
                4
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
            0x2A => { //ldi hl
                let hl = self.get_hl();
                self.a = bus.read(hl);
                self.set_hl(hl.wrapping_add(1));
                8
            }
            0x2F => {
                self.a = !self.a;
                self.f |= 0x60;
                4
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
                self.sp = (self.read_u16(bus));
                12
            }
            0x32 => { //ldi hl
                let hl = self.get_hl();
                bus.write(hl, self.a);
                self.set_hl(hl.wrapping_sub(1));
                8
            }
            0x37 => {
                self.f = (self.f & 0x80) | 0x10;
                4
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
            0x3A => { //ldi hl
                let hl = self.get_hl();
                self.a = bus.read(hl);
                self.set_hl(hl.wrapping_sub(1));
                8
            }
            0x3F => {
                let c = if (self.f & 0x10) != 0 { 0 } else { 0x10 };
                self.f = (self.f & 0x80) | c;
                4
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
            0xC0 | 0xC8 | 0xD0 | 0xD8 => { // cond ret nz z nc c
                let zero_flag = (self.f >> 7) & 1;
                let carry_flag = (self.f >> 4) & 1;
            
                let condition = match opcode {
                    0xC0 => zero_flag == 0,
                    0xC8 => zero_flag != 0,
                    0xD0 => carry_flag == 0,
                    0xD8 => carry_flag != 0,
                    _ => unreachable!(),
                };

                if condition {
                    let low = bus.read(self.sp) as u16;
                    self.sp = self.sp.wrapping_add(1);
                    let high = bus.read(self.sp) as u16;
                    self.sp = self.sp.wrapping_add(1);
                    self.pc = (high << 8) | low;
                    20
                } else {
                    8
                }
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
                let address = self.read_u16(bus);

                let zero_flag = (self.f >> 7) & 1;

                if (zero_flag == 0){
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xC3 => {
                let address = self.read_u16(bus);
                self.pc = address;
                16
            }
            0xC4 | 0xC4 | 0xD4 | 0xD4 => { // cond call nz z nc c
                let target_address = self.read_u16(bus); //reads the two bytes

                let zero_flag = (self.f >> 7) & 1;
                let carry_flag = (self.f >> 4) & 1;
            
                let condition = match opcode {
                    0xC4 => zero_flag == 0,
                    0xCC => zero_flag != 0,
                    0xD4 => carry_flag == 0,
                    0xDC => carry_flag != 0,
                    _ => unreachable!(),
                };

                if condition {
                    self.sp = self.sp.wrapping_sub(1);
                    bus.write(self.sp, (self.pc >> 8) as u8);
                    self.sp = self.sp.wrapping_sub(1);
                    bus.write(self.sp, (self.pc & 0xFF) as u8);

                    self.pc = target_address;
                    24
                } else {
                    12
                }
            }
            0xC5 | 0xD5 | 0xE5 | 0xF5 => { //push
                let value = match opcode {
                    0xC5 => self.get_bc(),
                    0xD5 => self.get_de(),
                    0xE5 => self.get_hl(),
                    0xF5 => ((self.a as u16) << 8) | (self.f as u16),
                    _ => unreachable!(),
                };
                self.sp -= 1;
                bus.write(self.sp, (value >> 8) as u8);
                self.sp -= 1;
                bus.write(self.sp, (value & 0xFF) as u8);
                16
            }
            0xCA => {
                let address = self.read_u16(bus);
                let zero_flag = (self.f >> 7) & 1;
                if (zero_flag != 0){
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
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                //RST 00h 08h 10h 18h 20h 28h 30h 38h
                self.sp = self.sp.wrapping_sub(1);
                bus.write(self.sp, (self.pc >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                bus.write(self.sp, (self.pc & 0xFF) as u8);

                self.pc = (opcode & 0x38) as u16;
                16
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
                let address = read_u16(bus);

                let carry_flag = (self.f >> 4) & 1;

                if (carry_flag == 0){
                    self.pc = address;
                    16
                } else {
                    12
                }
            }
            0xD9 => {
                self.ime = true;
                let low = bus.read(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);
                let high = bus.read(self.sp) as u16;
                self.sp = self.sp.wrapping_add(1);
            
                self.pc = (high << 8) | low;
                16
            }
            0xDA => {
                let address = read_u16(bus);

                let carry_flag = (self.f >> 4) & 1;

                if (carry_flag != 0){
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
            0xE8 => {
                let offset = bus.read(self.pc) as i8 as i16 as u16;
                self.pc += 1;

                let sp = self.sp;
                let result = sp.wrapping_add(offset);

                let h = if (sp & 0x0F) + (offset & 0x0F) > 0x0F { 0x20 } else { 0 };
                let c = if (sp & 0xFF) + (offset & 0xFF) > 0xFF { 0x10 } else { 0 };

                self.f = h | c;
                self.sp = result;
                16
            }
            0xEA => {
                let address = self.read_u16(bus);
                bus.write(address, self.a);
                16
            }
            0xE2 => {
                bus.write(0xFF00 | (self.c as u16), self.a);
                8
            }
            0xE9 => {
                self.pc = self.get_hl();
                4
            }
            0xF0 => {
                let n = bus.read(self.pc) as u16;
                self.pc += 1;
                self.a = bus.read(0xFF00 | n);
                12
            }
            0xF2 => {
                self.a = bus.read(0xFF00 | (self.c as u16));
                8
            }
            0xF3 | 0xFB => {
                match opcode {
                    0xF3 => self.ime = false,
                    0xFB => self.ime = true,
                }
                4
            }
            0xF8 => {
                let offset = bus.read(self.pc) as i8 as i16 as u16;
                self.pc += 1;

                let sp = self.sp;
                let result = sp.wrapping_add(offset);

                let h = if (sp & 0X0F) + (offset & 0x0F) > 0x0F { 0x20 } else { 0 };
                let c = if (sp & 0xFF) + (offset & 0xFF) > 0xFF { 0x10 } else { 0 };

                self.f = h | c;
                self.set_hl(result);
                12

            }
            0xFA => {
                let address = self.read_u16(bus);
                self.a = bus.read(address);
                16
            }
            _ => 4,
        }
    }
    #[inline(always)]
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    #[inline(always)]
    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    #[inline(always)]
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    #[inline(always)]
    fn set_bc(&mut self, value: u16){
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }
    #[inline(always)]
    fn set_de(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }
    #[inline(always)]
    fn set_hl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
    #[inline(always)]
    fn cb_bit_test(&mut self, value: u8, bit: u8){
                let is_set = (value & (1 << bit)) != 0;

                //flags
                let z = if !is_set { 0x80 } else { 0 };
                let n = 0x00;
                let h = 0x20;
                let c = self.f & 0x10;

                self.f = z | n | h | c;
    }
    #[inline(always)]
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
    #[inline(always)]
    fn cb_rlc(&mut self, value: u8) -> u8 {
        let bit7 = (value >> 7) & 1;
        let result = (value << 1) | bit7;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    #[inline(always)]
    fn cb_rrc(&mut self, value: u8) -> u8 {
        let bit0 = (value & 1);
        let result = (value >> 1) | (bit0 << 7);
        self.set_flags(result, 0, 0, bit0);
        result
    }
    #[inline(always)]
    fn cb_rl(&mut self, value: u8) -> u8 {
        let old_carry = (self.f >> 4) & 1;
        let bit7 = (value >> 7) & 1;
        let result = (value << 1) | old_carry;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    #[inline(always)]
    fn cb_rr(&mut self, value: u8) -> u8 {
        let old_carry = (self.f >> 4) & 1;
        let bit0 = value & 1;
        let result = (value >> 1) | (old_carry << 7);
        self.set_flags(result, 0, 0, bit0);
        result
    }
    #[inline(always)]
    fn cb_sla(&mut self, value: u8) -> u8 {
        let bit7 = value >> 7;
        let result = value << 1;
        self.set_flags(result, 0, 0, bit7);
        result
    }
    #[inline(always)]
    fn cb_sra(&mut self, value: u8) -> u8 {
        let bit0 = value & 1;
        let result = (value >> 1) | (value & 0x80);
        self.set_flags(result, 0, 0, bit0);
        result
    }
   #[inline(always)]
   fn cb_swap(&mut self, value: u8) -> u8 {
        let result = (value >> 4) | (value << 4);
        self.f = if result == 0 { 0x80 } else { 0 };
        result
    }
    #[inline(always)]
    fn cb_srl(&mut self, value: u8) -> u8 {
        let bit0 = value & 1;
        let result = value >> 1;
        self.set_flags(result, 0, 0, bit0);
        result
    }
    #[inline(always)]
    fn set_flags(&mut self, res: u8, n: u8, h: u8, c: u8){
        let z_bit = if res == 0 { 0x80 } else { 0 };
        let n_bit = if n != 0 { 0x40 } else { 0 };
        let h_bit = if h != 0 { 0x20 } else { 0 };
        let c_bit = if c != 0 { 0x10 } else { 0 };

        self.f = z_bit | n_bit | h_bit | c_bit;
    }

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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
    #[inline(always)]
    fn inc_8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        let z = if result == 0 { 0x80 } else { 0 };
        let n = 0x00;
        let h = if ( value & 0x0F ) + 1 > 0x0F { 0x20 } else { 0 };
        let c = self.f & 0x10;

        self.f = z | n | h | c;
        result
    }
    #[inline(always)]
    fn dec_8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        let z = if result == 0 { 0x80 } else { 0 };
        let n = 0x40;
        let h = if ( value & 0x0F ) == 0 { 0x20 } else { 0 };
        let c = self.f & 0x10;

        self.f = z | n | h | c;
        result
    }
    #[inline(always)]
    fn read_u16(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let high = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        (high << 8) | low
    }
    #[inline(always)]
    fn execute_interrupt(&mut self, bus: &mut Bus, pending: u8) {
        self.ime = false;

        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, (self.pc >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, (self.pc & 0xFF) as u8);

        if (pending & 0x01) != 0 {
            bus.write(0xFF0F, pending & !0x01);
            self.pc = 0x0040;
        } else if (pending & 0x02) != 0 {
            bus.write(0xFF0F, pending & !0x02);
            self.pc = 0x0048;
        } else if (pending & 0x04) != 0 {
            bus.write(0xFF0F, pending & !0x04);
            self.pc = 0x0050;
        } else if (pending & 0x08) != 0 {
            bus.write(0xFF0F, pending & !0x08);
            self.pc = 0x0058;
        } else if (pending & 0x10) != 0 {
            bus.write(0xFF0F, pending & !0x10);
            self.pc = 0x0060;
        }
    }
}