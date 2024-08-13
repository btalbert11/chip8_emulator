use std::fmt;
use rand::Rng;




#[allow(non_camel_case_types)]
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
    Invalid_Instruction,
}

#[allow(non_camel_case_types)]
impl Instruction {
    fn parse_opcode(opcode: u16, high_byte: u8, low_byte: u8) -> Instruction {
        let first_nibble = high_byte & 0xF0; // always instruction indicator
        let last_nibble = low_byte & 0x0F; // sometimes instruction indicator
        let third_nibble = low_byte & 0xF0; // Sometimes instruction indicator.
        if first_nibble == 0x00 && third_nibble == 0xE0 { return Instruction::CLS; }
        // Super chip-48 instructions
        else if high_byte == 0x00 && low_byte == 0xEE { return Instruction::RET; }
        else if high_byte == 0x00 && low_byte == 0xFB { return Instruction::SCR; } // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFC { return Instruction::SCL; } // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFD { return Instruction::EXIT; } // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFE { return Instruction::LOW; } // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFF { return Instruction::HIGH; } // Super chip-48 instruction
        else if first_nibble == 0x00 && third_nibble == 0xC0 { return Instruction::SCD_nibble; } // Super chip-48 instruction
        else if first_nibble == 0x00 { return Instruction::SYS_addr; }
        else if first_nibble == 0x10 { return Instruction::JP_addr; }
        else if first_nibble == 0x20 { return Instruction::CALL_addr; }
        else if first_nibble == 0x30 { return Instruction::SE_Vx; }
        else if first_nibble == 0x40 { return Instruction::SNE_Vx; }
        else if first_nibble == 0x50 { return Instruction::SE_Vx_Vy; }
        else if first_nibble == 0x60 { return Instruction::LD_Vx; }
        else if first_nibble == 0x70 { return Instruction::ADD_Vx; }
        else if first_nibble == 0x80 && last_nibble == 0x00 { return Instruction::LD_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x01 { return Instruction::OR_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x02 { return Instruction::AND_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x03 { return Instruction::XOR_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x04 { return Instruction::ADD_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x05 { return Instruction::SUB_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x06 { return Instruction::SHR_Vx; }
        else if first_nibble == 0x80 && last_nibble == 0x07 { return Instruction::SUBN_Vx_Vy; }
        else if first_nibble == 0x80 && last_nibble == 0x0E { return Instruction::SHL_Vx; }
        else if first_nibble == 0x90 && last_nibble == 0x00 { return Instruction::SNE_Vx_Vy; }
        else if first_nibble == 0xA0 { return Instruction::LD_I; }
        else if first_nibble == 0xB0 { return Instruction::JP_V0; }
        else if first_nibble == 0xC0 { return Instruction::RND_Vx; }
        else if first_nibble == 0xD0 && last_nibble == 0x00 { return Instruction::DRW_Vx_Vy_0; } // Super chip-48 instructions
        else if first_nibble == 0xD0 { return Instruction::DRW_Vx_Vy; }
        else if first_nibble == 0xE0 && low_byte == 0x9E { return Instruction::SKP_Vx; }
        else if first_nibble == 0xE0 && low_byte == 0xA1 { return Instruction::SKNP_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x07 { return Instruction::LD_Vx_DT; }
        else if first_nibble == 0xF0 && low_byte == 0x0A { return Instruction::LD_Vx_K; }
        else if first_nibble == 0xF0 && low_byte == 0x15 { return Instruction::LD_DT_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x18 { return Instruction::LD_ST_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x1E { return Instruction::ADD_I_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x29 { return Instruction::LD_F_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x33 { return Instruction::LD_B_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x55 { return Instruction::LD_I_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x65 { return Instruction::LD_Vx_I; }
        // Super chip-48 instructions
        else if first_nibble == 0xF0 && low_byte == 0x30 { return Instruction::LD_HF_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x75 { return Instruction::LD_R_Vx; }
        else if first_nibble == 0xF0 && low_byte == 0x85 { return Instruction::LD_Vx_R; }
        else {
            return Instruction::Invalid_Instruction;
        }

    }
}

#[derive(Copy, Clone)]
enum Key {
    Up,
    Down
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

    pub fn is_key_down(&self, index: u8) -> bool {
        match self.keys[index as usize] {
            Key::Up => false,
            Key::Down => true,
        }
    }

    pub fn get_first_key_down(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            match key {
                Key::Down => return Some(i as u8),
                _ => ()
            }
        }
        None
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

    pub fn emulate(&mut self, opcode: u16, keyboard: &Keyboard) {
        let high_byte: u8 = ((opcode >> 8) & 0xFF) as u8;
        let low_byte: u8 = (opcode & 0xFF) as u8;
        println!("high: {:#04x}, low: {:#04x}", high_byte, low_byte);

        match Instruction::parse_opcode(opcode, high_byte, low_byte) {
            Instruction::SYS_addr => self.sys_addr(),
            Instruction::CLS => self.missing_opcode(opcode),
            Instruction::RET => self.ret(),
            Instruction::JP_addr => self.jp_addr(opcode),
            Instruction::CALL_addr => self.call_addr(opcode),
            Instruction::SE_Vx => self.se_vx(high_byte, low_byte),
            Instruction::SNE_Vx => self.sne_vx(high_byte, low_byte),
            Instruction::SE_Vx_Vy => self.se_vx_vy(high_byte, low_byte),
            Instruction::LD_Vx => self.ld_vx(high_byte, low_byte),
            Instruction::ADD_Vx => self.missing_opcode(opcode),
            Instruction::LD_Vx_Vy => self.ld_vx_vy(high_byte, low_byte),
            Instruction::OR_Vx_Vy => self.or_vx_vy(high_byte, low_byte),
            Instruction::AND_Vx_Vy => self.and_vx_vy(high_byte, low_byte),
            Instruction::XOR_Vx_Vy => self.xor_vx_vy(high_byte, low_byte),
            Instruction::ADD_Vx_Vy => self.add_vx_vy(high_byte, low_byte),
            Instruction::SUB_Vx_Vy => self.sub_vx_vy(high_byte, low_byte),
            Instruction::SHR_Vx => self.shr_vx(high_byte),
            Instruction::SUBN_Vx_Vy => self.subn_vx_vy(high_byte, low_byte),
            Instruction::SHL_Vx => self.shl_vx(high_byte),
            Instruction::SNE_Vx_Vy => self.sne_vx_vy(high_byte, low_byte),
            Instruction::LD_I => self.ld_i(opcode),
            Instruction::JP_V0 => self.jp_v0(opcode),
            Instruction::RND_Vx => self.rnd_vx(high_byte, low_byte),
            Instruction::DRW_Vx_Vy => self.missing_opcode(opcode),
            Instruction::SKP_Vx => self.skp_vx(high_byte, keyboard),
            Instruction::SKNP_Vx => self.sknp_vx(high_byte, keyboard),
            Instruction::LD_Vx_DT => self.ld_vx_dt(high_byte),
            Instruction::LD_Vx_K => self.missing_opcode(opcode),
            Instruction::LD_DT_Vx => self.ld_dt_vx(high_byte),
            Instruction::LD_ST_Vx => self.ld_st_vx(high_byte),
            Instruction::ADD_I_Vx => self.add_i_vx(high_byte),
            Instruction::LD_F_Vx => self.missing_opcode(opcode),
            Instruction::LD_B_Vx => self.missing_opcode(opcode),
            Instruction::LD_I_Vx => self.ld_i_vx(high_byte),
            Instruction::LD_Vx_I => self.ld_vx_i(high_byte),
            Instruction::SCD_nibble => self.missing_opcode(opcode),
            Instruction::SCR => self.missing_opcode(opcode),
            Instruction::SCL => self.missing_opcode(opcode),
            Instruction::EXIT => self.missing_opcode(opcode),
            Instruction::LOW => self.missing_opcode(opcode),
            Instruction::HIGH => self.missing_opcode(opcode),
            Instruction::DRW_Vx_Vy_0 => self.missing_opcode(opcode),
            Instruction::LD_HF_Vx => self.missing_opcode(opcode),
            Instruction::LD_R_Vx => self.missing_opcode(opcode),
            Instruction::LD_Vx_R => self.missing_opcode(opcode),
            Instruction::Invalid_Instruction => panic!("INVALID OP CODE"),
        }
    }

    fn sys_addr(&mut self) {
        // This instruction is ignored in modern emulators
        return
    }

    fn cls (&mut self) {
        // clear the display, set all values to 0
        // TODO
    }

    fn ret (&mut self) {
        // return from a subroutine
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn jp_addr(&mut self, opcode: u16) {
        // jumps to address nnn
        self.pc = opcode & 0x0FFF;
    }

    fn call_addr(&mut self, opcode: u16) {
        // call subroutine
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = opcode & 0x0FFF;
    }

    fn se_vx(&mut self, high_byte: u8, low_byte: u8) {
        // compare Vx to kk, skip next instruction if equal
        let second_nibble = high_byte & 0x0F;
        if self.registers[second_nibble as usize] == low_byte {
            self.pc += 2;
        }
    }

    fn sne_vx(&mut self, high_byte: u8, low_byte: u8) {
        // Compare Vx to kk, skip if not equal
        let second_nibble = high_byte & 0x0F;
        if self.registers[second_nibble as usize] != low_byte {
            self.pc += 2;
        }
    }

    fn se_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // compare Vx to Vy, skip if equal
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        if self.registers[second_nibble as usize] == self.registers[third_nibble as usize] {
            self.pc += 2;
        }
    }

    fn ld_vx(&mut self, high_byte: u8, low_byte: u8) {
        // load kk into Vx
        let second_nibble = high_byte & 0x0F;
        self.registers[second_nibble as usize] = low_byte;
    }

    fn add_vx(&mut self, high_byte: u8, low_byte: u8) {
        // Add kk to Vx and store in Vx
        let second_nibble = high_byte & 0x0F;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize].wrapping_add(low_byte);
    }

    fn ld_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // Set Vx = Vy
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        self.registers[second_nibble as usize] = self.registers[third_nibble as usize];
    }

    fn or_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx to Vx OR Vy
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] | self.registers[third_nibble as usize]; 
    }

    fn and_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx to Vx AND Vy
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] & self.registers[third_nibble as usize]; 
    }

    fn xor_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx to Vx XOR Vy
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] ^ self.registers[third_nibble as usize];
    }

    fn add_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx to Vx + Vy. Only keep lowest 8 bits, set VF = 1 if overflow
        // I can't find what "lowest 8 bits kept" means, so I am assuming its a normal overflow
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        let result: u16 = (self.registers[second_nibble as usize] as u16) + (self.registers[third_nibble as usize] as u16);
        // set carry flag
        if result > 255 { 
            self.registers[self.flag_register_index as usize] = 1;
        } else {
            self.registers[self.flag_register_index as usize] = 0;
        }
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize].wrapping_add(self.registers[third_nibble as usize]);
        // TODO CHECK THIS FUNCTION
    }

    fn sub_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx = Vx - Vy. If underflow (Vy > Vx), set Vf to 0 else 1.
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte &0xF0) >> 4;
        // underflow
        if third_nibble > second_nibble {
            self.registers[self.flag_register_index as usize] = 0;
        } else {
            self.registers[self.flag_register_index as usize] = 1;
        }
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize].wrapping_sub(self.registers[third_nibble as usize]);
    }

    fn shr_vx(&mut self, high_byte: u8) {
        // store least significant bit of Vx into Vf, then Vx >>= 1
        let second_nibble = high_byte & 0x0F;
        self.registers[self.flag_register_index as usize] = second_nibble & 0x1;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] >> 1;
    }

    fn subn_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx = Vy - Vx, set Vf = 1 if no underflow
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        if second_nibble > third_nibble {
            self.registers[self.flag_register_index as usize] = 0;
        } else {
            self.registers[self.flag_register_index as usize] = 1;
        }
        self.registers[second_nibble as usize] = self.registers[third_nibble as usize].wrapping_sub(self.registers[second_nibble as usize]);
    }

    fn shl_vx(&mut self, high_byte: u8) {
        // set Vf to most significant bit of Vx, then shift Vx left 1
        let second_nibble = high_byte & 0x0F;
        self.registers[self.flag_register_index as usize] = second_nibble & 0x80;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] << 1;
    }

    fn sne_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // skip instruction if Vx != Vy
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        if self.registers[second_nibble as usize] != self.registers[third_nibble as usize] {
            self.pc += 2;
        }
    }

    fn ld_i(&mut self, opcode: u16) {
        // set I to nnn
        let address = opcode & 0x0FFF;
        self.address_register = address;
    }

    fn jp_v0(&mut self, opcode: u16) {
        // set pc to nnn + V0
        let address = opcode & 0x0FFF;
        self.pc = self.registers[0x0] as u16 + address;
    }

    fn rnd_vx(&mut self, high_byte: u8, low_byte: u8) {
        let second_nibble = high_byte & 0x0F;
        let rn: u8 = rand::thread_rng().gen_range(0..=255);
        self.registers[second_nibble as usize] = rn & low_byte;
    }

    fn drw_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // draw a sprite that is n bytes, from memory address I, starting at coordinates (Vx,Vy)
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        let last_nibble = low_byte & 0x0F;
        //TODO implement this with display 
    }

    fn skp_vx(&mut self, high_byte: u8, keyboard: &Keyboard) {
        // if key with value Vx is down, skip instruction
        let second_nibble = high_byte & 0x0F;
        if keyboard.is_key_down(self.registers[second_nibble as usize]) {
            self.pc += 2;
        }
    }

    fn sknp_vx(&mut self, high_byte: u8, keyboard: &Keyboard) {
        // skip instruction if key is up
        let second_nibble = high_byte & 0x0F;
        if !keyboard.is_key_down(self.registers[second_nibble as usize]) {
            self.pc += 2;
        }
    }

    fn ld_vx_dt(&mut self, high_byte: u8) {
        let sn = high_byte & 0x0F;
        self.registers[sn as usize] = self.delay_timer_register;
    }

    fn ld_vx_k(&mut self, high_byte: u8, keyboard: &Keyboard) {
        // This function will halt execution until a key is pressed. Then
        // store that key in Vx. This function assumes that a key has already been pressed.
        //TODO need to stop execution probably in the outer loop
        let sn = high_byte & 0x0F;
        self.registers[sn as usize] = match  keyboard.get_first_key_down() {
            Some(i) => i,
            None => panic!("Keyboard had no keys down, execution should have paused prior to function call.")
        }
    }

    fn ld_dt_vx(&mut self, high_byte: u8) {
        // set DT = Vx
        let sn = high_byte & 0x0F;
        self.delay_timer_register = self.registers[sn as usize];
    }

    fn ld_st_vx(&mut self, high_byte: u8) {
        // set ST = Vx
        let sn = high_byte & 0x0F;
        self.sound_timer_register = self.registers[sn as usize];
    }

    fn add_i_vx(&mut self, high_byte: u8) {
        // I = I + Vx
        let sn = high_byte & 0x0F;
        self.address_register = self.address_register.wrapping_add(self.registers[sn as usize] as u16);
    }

    fn ld_f_xv(&mut self, high_byte: u8) {
        // TODO Not sure what exactly this instruction does
    }

    fn ld_b_vx(&mut self, high_byte: u8) {
        // TODO take the decimal value of Vx, place the hundres digit in memory[I], tens in memory[I + 1], and ones in memory[I+2]
    }

    fn ld_i_vx(&mut self, high_byte: u8) {
        // store registers V0..Vx in memory starting at memory[I]
        let sn = high_byte & 0x0F;
        for i in 0..=sn as usize{
            self.memory[self.address_register as usize + i] = self.registers[i];
        }
    }

    fn ld_vx_i(&mut self, high_byte: u8) {
        // read values from memory[I..I+x] into register V0..Vx
        let sn = high_byte & 0x0F; 
        for i in 0..=sn as usize {
            self.registers[i] = self.memory[self.address_register as usize + i];
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
