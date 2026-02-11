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

struct Alu {

}

impl Alu {
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

    fn step(&self, &mut bus) -> u32 {

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
            0x3E => { //LD A, n
                self.a = bus.read(self.pc);
                self.pc +=1;
                8 //2byte instruction
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

}