use std::fmt;

#[derive(Copy, Clone)]
enum Key {
    Up,
    Down
}

enum Instruction {
    SYS_addr,
    CLS,
    RET,
    JP_addr,
    CALL_addr,
    SE_Vx,
    SNE_Vx,
    SE_Vx_Vy,
    LD_Vx,
    ADD_Vx,
    LD_Vx_Vy,
    OR_Vx_Vy,
    AND_Vx_Vy,
    XOR_Vx_Vy,
    ADD_Vx_Vy,
    SUB_Vx_Vy,
    SHR_Vx,
    SUBN_Vx_Vy,
    SHL_Vx,
    SNE_Vx_Vy,
    LD_I,
    JP_V0,
    RND_Vx,
    DRW_Vx_Vy,
    SKP_Vx,
    SKNP_Vx,
    LD_Vx_DT,
    LD_Vx_K,
    LD_DT_Vx,
    LD_ST_Vx,
    ADD_I_Vx,
    LD_F_Vx,
    LD_B_Vx,
    LD_I_Vx,
    LD_Vx_I,
    // Super chip 48 instructions
    SCD_nibble,
    SCR,
    SCL,
    EXIT,
    LOW,
    HIGH,
    DRW_Vx_Vy_0,
    LD_HF_Vx,
    LD_R_Vx,
    LD_Vx_R,
}

impl Instruction {
    fn parse_opcode(opcode: u16, high_byte: u8, low_byte: u8) -> Instruction {
        let first_nibble = high_byte & 0xF0; // never a variable
        let last_nibble = low_byte & 0x0F; // sometimes instruction indicator
        let third_nibble = low_byte & 0xF0; // Sometimes instruction indicator.
        if first_nibble == 0x00 && third_nibble == 0xE0 { return Instruction::CLS; }
        else if high_byte == 0x00 && low_byte == 0xEE { return Instruction::RET; }
        else if high_byte == 0x00 && low_byte == 0xFB { return Instruction::SCR; }
        else if high_byte == 0x00 && low_byte == 0xFC { return Instruction::SCL; }
        else if high_byte == 0x00 && low_byte == 0xFD { return Instruction::EXIT; }
        else if high_byte == 0x00 && low_byte == 0xFE { return Instruction::LOW; }
        else if high_byte == 0x00 && low_byte == 0xFF { return Instruction::HIGH; }
        else if first_nibble == 0x00 && third_nibble == 0xC0 { return Instruction::SCD_nibble; }
        else if first_nibble == 0x00 { return Instruction::SYS_addr; }
        else if first_nibble == 0x10 {}
    }
}


pub struct Keyboard {
    keys: [Key; 16]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [Key::Up; 16]
        }
    }
}

 pub struct Display {
     display: [[bool; 64]; 48]
 }

pub struct Emulator {
    registers: [u8; 16],
    flag_register_index: u8,
    pc: u16,
    sp: u8,
    delay_timer_register: u8,
    sound_timer_register: u8,
    address_register: u16,
    memory: [u8; 0x1000],
    program_memory_index: u16,
    display_refresh_memory_index: u16,
    eti_660_memory_index: u16,
    stack: [u16; 16],
}

impl Emulator {
    pub fn new() -> Emulator {
        // TODO need to add spirtes of 0-F in start of memory
        Emulator {
            registers: [0; 16],
            flag_register_index: 0xF,
            pc: 0,
            sp: 0,
            delay_timer_register: 0,
            sound_timer_register: 0,
            address_register: 0,
            memory: [0; 0x1000],
            program_memory_index: 0x200,
            display_refresh_memory_index: 0xF00,
            eti_660_memory_index: 0x600,
            stack: [0; 16],
        }
    }

    fn get_nonzero_memory(&self) -> u16 {
        let mut count = 0;
        for val in self.memory.iter() {
            if *val != 0 {
                count += 1;
            }
        }
        count
    }

    pub fn emulate(&mut self, opcode: u16) {
        let high_byte: u8 = ((opcode >> 8) & 0xFF) as u8;
        let low_byte: u8 = (opcode & 0xFF) as u8;
        println!("high: {}, low: {}", high_byte, low_byte);

        match opcode {
            0x00E0 => self.missing_opcode(opcode),
            _ => panic!("Invalid Opcode! {}", opcode)
        
        }
    }

    fn missing_opcode(&self, opcode: u16) {
        println!("Missing opcode! {} \n{:?}", opcode, *self);
        panic!();
    }


}
impl fmt::Debug for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nonzero_memory = self.get_nonzero_memory();
        write!(f, "Emulator: registers: {:?}, pc: {}, sp: {}, delay_timer_register: {}, sound_timer_register: {}, address_register{}, non-zero memory values: {}, stack: {:?}", 
        self.registers, self.pc, self.sp, self.delay_timer_register, self.sound_timer_register, 
        self.address_register, nonzero_memory, self.stack)
    }
}
