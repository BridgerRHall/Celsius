//use atomic import

struct Memory_Map {
    
    //memory map
    rom_bank_00: AtomicU8, //0000 - 3FFF 16kb fixed bank -  from catridge
    rom_bank_01: AtomicU8, //4000 - 7FFF 16kb switchable bank -  via mapper (if any)
    video_ram: AtomicU8,   //8000 - 9FFF 8kb Vram -  in CGB mode is switcahble bank
    rom_ext_ram: AtomicU8, //A000 - BFFF 8kb external ram -  from catridege switchable bank if any
    work_ram_00: AtomicU8, //C000 - CFFF 4kb work ram
    work_ram_01: AtomicU8, //D000 - DFFF 4kb work ram - incb mode switchable bank 1-7
    echo_ram: AtomicU8,    //E000 - FDFF mirror of C000-DDFF (use prohibited)
    oam: AtomicU8,         //FE00 - FE9F Object Attribute Memory (sprites)
    not_usable: AtomicU8,  //FEA0 - FEFF not usable (use prohibited)
    io_registers: AtomicU8 //FF00 - FF7F IO registers - see line
    high_ram: AtomicU8,    //FF80 - FFFE High Ram
    intr_enbl_rg: AtomicU8,//FFFF - FFFF Interrupt enable register IE
}

struct Bus {
    memory_map: Memory_Map,
    cpu_lock: Bool,
    ppu_lock: Bool,
}


public impl Bus(){
    fn New() -> Self{
    }



    fn boot_sequence(){
    }


    fn read(){
    }

    fn write(){
    }
}

