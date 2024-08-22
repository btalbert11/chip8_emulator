use std::fmt;
use rand::Rng;
use crate::{
    instruction::Instruction, 
    screen::Screen,
    keyboard::{Key, Keyboard},
};




pub struct Emulator {
    registers: [u8; 16],
    flag_register_index: usize,
    pc: u16,
    sp: u8,
    delay_timer_register: u8,
    sound_timer_register: u8,
    address_register: u16,
    memory: [u8; 0x1000],
    program_memory_index: usize,
    display_refresh_memory_index: usize,
    eti_660_memory_index: usize,
    sprite_memory_index: usize,
    stack: [u16; 16],
}


impl Emulator {
    pub fn new() -> Emulator {
        let mut e = Emulator {
            registers: [0; 16],
            flag_register_index: 0xF,
            pc: 0x200,
            sp: 0,
            delay_timer_register: 0,
            sound_timer_register: 0,
            address_register: 0,
            memory: [0; 0x1000],
            program_memory_index: 0x200,
            display_refresh_memory_index: 0xF00,
            eti_660_memory_index: 0x600,
            sprite_memory_index: 0x000,
            stack: [0; 16],
        };
        e.set_character_sprites();
        e
    }

    fn set_character_sprites(&mut self) {
        // sets up the character 0-F in the interpreters memory
        // 0
        self.memory[self.sprite_memory_index + 0] = 0xF0;
        self.memory[self.sprite_memory_index + 1] = 0x90;
        self.memory[self.sprite_memory_index + 2] = 0x90;
        self.memory[self.sprite_memory_index + 3] = 0x90;
        self.memory[self.sprite_memory_index + 4] = 0xF0;
        // 1
        self.memory[self.sprite_memory_index + 5] = 0x20;
        self.memory[self.sprite_memory_index + 6] = 0x60;
        self.memory[self.sprite_memory_index + 7] = 0x20;
        self.memory[self.sprite_memory_index + 8] = 0x20;
        self.memory[self.sprite_memory_index + 9] = 0x70;
        // 2
        self.memory[self.sprite_memory_index + 10] = 0xF0;
        self.memory[self.sprite_memory_index + 11] = 0x10;
        self.memory[self.sprite_memory_index + 12] = 0xF0;
        self.memory[self.sprite_memory_index + 13] = 0x80;
        self.memory[self.sprite_memory_index + 14] = 0xF0;
        // 3
        self.memory[self.sprite_memory_index + 15] = 0xF0;
        self.memory[self.sprite_memory_index + 16] = 0x10;
        self.memory[self.sprite_memory_index + 17] = 0xF0;
        self.memory[self.sprite_memory_index + 18] = 0x10;
        self.memory[self.sprite_memory_index + 19] = 0xF0;
        // 4
        self.memory[self.sprite_memory_index + 20] = 0x90;
        self.memory[self.sprite_memory_index + 21] = 0x90;
        self.memory[self.sprite_memory_index + 22] = 0xF0;
        self.memory[self.sprite_memory_index + 23] = 0x10;
        self.memory[self.sprite_memory_index + 24] = 0x10;
        // 5
        self.memory[self.sprite_memory_index + 25] = 0xF0;
        self.memory[self.sprite_memory_index + 26] = 0x80;
        self.memory[self.sprite_memory_index + 27] = 0xF0;
        self.memory[self.sprite_memory_index + 28] = 0x10;
        self.memory[self.sprite_memory_index + 29] = 0xF0;
        // 6
        self.memory[self.sprite_memory_index + 30] = 0xF0;
        self.memory[self.sprite_memory_index + 31] = 0x80;
        self.memory[self.sprite_memory_index + 32] = 0xF0;
        self.memory[self.sprite_memory_index + 33] = 0x90;
        self.memory[self.sprite_memory_index + 34] = 0xF0;
        // 7 
        self.memory[self.sprite_memory_index + 35] = 0xF0;
        self.memory[self.sprite_memory_index + 36] = 0x10;
        self.memory[self.sprite_memory_index + 37] = 0x20;
        self.memory[self.sprite_memory_index + 38] = 0x40;
        self.memory[self.sprite_memory_index + 39] = 0x40;
        // 8
        self.memory[self.sprite_memory_index + 40] = 0xF0;
        self.memory[self.sprite_memory_index + 41] = 0x90;
        self.memory[self.sprite_memory_index + 42] = 0xF0;
        self.memory[self.sprite_memory_index + 43] = 0x90;
        self.memory[self.sprite_memory_index + 44] = 0xF0;
        // 9
        self.memory[self.sprite_memory_index + 45] = 0xF0;
        self.memory[self.sprite_memory_index + 46] = 0x90;
        self.memory[self.sprite_memory_index + 47] = 0xF0;
        self.memory[self.sprite_memory_index + 48] = 0x10;
        self.memory[self.sprite_memory_index + 49] = 0xF0;
        // A
        self.memory[self.sprite_memory_index + 50] = 0xF0;
        self.memory[self.sprite_memory_index + 51] = 0x90;
        self.memory[self.sprite_memory_index + 52] = 0xF0;
        self.memory[self.sprite_memory_index + 53] = 0x90;
        self.memory[self.sprite_memory_index + 54] = 0x90;
        // B
        self.memory[self.sprite_memory_index + 55] = 0xE0;
        self.memory[self.sprite_memory_index + 56] = 0x90;
        self.memory[self.sprite_memory_index + 57] = 0xE0;
        self.memory[self.sprite_memory_index + 58] = 0x90;
        self.memory[self.sprite_memory_index + 59] = 0xE0;
        // C
        self.memory[self.sprite_memory_index + 60] = 0xF0;
        self.memory[self.sprite_memory_index + 61] = 0x80;
        self.memory[self.sprite_memory_index + 62] = 0x80;
        self.memory[self.sprite_memory_index + 63] = 0x80;
        self.memory[self.sprite_memory_index + 64] = 0xF0;
        // D
        self.memory[self.sprite_memory_index + 65] = 0xE0;
        self.memory[self.sprite_memory_index + 66] = 0x90;
        self.memory[self.sprite_memory_index + 67] = 0x90;
        self.memory[self.sprite_memory_index + 68] = 0x90;
        self.memory[self.sprite_memory_index + 69] = 0xE0;
        // E
        self.memory[self.sprite_memory_index + 70] = 0xF0;
        self.memory[self.sprite_memory_index + 71] = 0x80;
        self.memory[self.sprite_memory_index + 72] = 0xF0;
        self.memory[self.sprite_memory_index + 73] = 0x80;
        self.memory[self.sprite_memory_index + 74] = 0xF0;
        // F
        self.memory[self.sprite_memory_index + 75] = 0xF0;
        self.memory[self.sprite_memory_index + 76] = 0x80;
        self.memory[self.sprite_memory_index + 77] = 0xF0;
        self.memory[self.sprite_memory_index + 78] = 0x80;
        self.memory[self.sprite_memory_index + 79] = 0x80;
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

    pub fn set_memory(&mut self, byte: u8, mem_address: usize) {
        self.memory[mem_address] = byte;
    }

    pub fn print_memory(&self) {
        for i in (0..0xFFF).step_by(16){
            println!("{:#05x}: {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x}, {:#06x},",
                i, self.memory[i], self.memory[i+1], self.memory[i+2], self.memory[i+3], self.memory[i+4], self.memory[i+5], self.memory[i+6], 
                self.memory[i+7], self.memory[i+8], self.memory[i+9], self.memory[i+10], self.memory[i+11], self.memory[i+12], self.memory[i+13], 
                self.memory[i+14], self.memory[i+15]);
        }
    }

    // read PC opcode and run it
    pub fn emulate_step(&mut self, keyboard: &Keyboard, screen: &mut Screen) {
        let first_byte = self.memory[self.pc as usize];
        let second_byte = self.memory[(self.pc + 1) as usize];
        let opcode: u16 = ((first_byte as u16) << 8) | (second_byte as u16);
        // println!("{:#04x}, {:#04x}, {:#04x}, {:#06x}", self.pc, first_byte, second_byte, opcode);
        self.emulate(opcode, &keyboard, screen);
        self.pc += 2;
    }

    pub fn emulate(&mut self, opcode: u16, keyboard: &Keyboard, screen: &mut Screen) {
        let high_byte: u8 = ((opcode >> 8) & 0xFF) as u8;
        let low_byte: u8 = (opcode & 0xFF) as u8;
        // println!("high: {:#04x}, low: {:#04x}", high_byte, low_byte);

        match Instruction::parse_opcode(opcode) {
            Instruction::SYS_addr => self.sys_addr(),
            Instruction::CLS => self.missing_opcode(opcode),
            Instruction::RET => self.ret(),
            Instruction::JP_addr => self.jp_addr(opcode),
            Instruction::CALL_addr => self.call_addr(opcode),
            Instruction::SE_Vx => self.se_vx(high_byte, low_byte),
            Instruction::SNE_Vx => self.sne_vx(high_byte, low_byte),
            Instruction::SE_Vx_Vy => self.se_vx_vy(high_byte, low_byte),
            Instruction::LD_Vx => self.ld_vx(high_byte, low_byte),
            Instruction::ADD_Vx => self.add_vx(high_byte, low_byte),
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
            Instruction::DRW_Vx_Vy => self.drw_vx_vy(high_byte, low_byte, screen),
            Instruction::SKP_Vx => self.skp_vx(high_byte, keyboard),
            Instruction::SKNP_Vx => self.sknp_vx(high_byte, keyboard),
            Instruction::LD_Vx_DT => self.ld_vx_dt(high_byte),
            Instruction::LD_Vx_K => self.ld_vx_k(high_byte, &keyboard),
            Instruction::LD_DT_Vx => self.ld_dt_vx(high_byte),
            Instruction::LD_ST_Vx => self.ld_st_vx(high_byte),
            Instruction::ADD_I_Vx => self.add_i_vx(high_byte),
            Instruction::LD_F_Vx => self.ld_f_vx(high_byte),
            Instruction::LD_B_Vx => self.ld_b_vx(high_byte),
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
    }

    fn sub_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx = Vx - Vy. If underflow (Vy > Vx), set Vf to 0 else 1.
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte &0xF0) >> 4;
        // No underflow
        if self.registers[second_nibble as usize] > self.registers[third_nibble as usize] {
            self.registers[self.flag_register_index as usize] = 1;
        } else {
            self.registers[self.flag_register_index as usize] = 0;
        }
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize].wrapping_sub(self.registers[third_nibble as usize]);
    }

    fn shr_vx(&mut self, high_byte: u8) {
        // store least significant bit of Vx into Vf, then Vx >>= 1
        let second_nibble = high_byte & 0x0F;
        self.registers[self.flag_register_index] = self.registers[second_nibble as usize] & 0x01;
        self.registers[second_nibble as usize] = self.registers[second_nibble as usize] >> 1;
    }

    fn subn_vx_vy(&mut self, high_byte: u8, low_byte: u8) {
        // set Vx = Vy - Vx, set Vf = 1 if no underflow
        let second_nibble = high_byte & 0x0F;
        let third_nibble = (low_byte & 0xF0) >> 4;
        // No underflow, set to 1
        if self.registers[third_nibble as usize] > self.registers[second_nibble as usize] {
            self.registers[self.flag_register_index] = 1;
        } else {
            self.registers[self.flag_register_index] = 0;
        }
        self.registers[second_nibble as usize] = self.registers[third_nibble as usize].wrapping_sub(self.registers[second_nibble as usize]);
    }

    fn shl_vx(&mut self, high_byte: u8) {
        // set Vf to most significant bit of Vx, then shift Vx left 1
        let second_nibble = high_byte & 0x0F;
        self.registers[self.flag_register_index] = (self.registers[second_nibble as usize] & 0x80) >> 7;
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
        println!("{:#04x}, {:#04x}", rn, rn & low_byte);
        self.registers[second_nibble as usize] = rn & low_byte;
    }

    fn drw_vx_vy(&mut self, high_byte: u8, low_byte: u8, screen: &mut Screen) {
        // draw a sprite that is n bytes, from memory address I, starting at coordinates (Vx,Vy).
        // If an on pixel is already set at any point in the sprite, it is set to off and VF is set.
        // From my understanding the only way a pixel is set to off is by this collision.
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
        self.registers[sn as usize] = match keyboard.get_first_key_down() {
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

    fn ld_f_vx(&mut self, high_byte: u8) {
        // TODO This gets the hexidecimal digits that are stored in the 
        // begginning section of memory (first 512 bytes).
        // I think these can be stored in an arbitary location, but I'm not sure.
        // I = sprite_mem_index
        let sn = high_byte & 0x0F;
        self.address_register = self.sprite_memory_index as u16 + (5 * self.registers[sn as usize] as u16);
    }

    fn ld_b_vx(&mut self, high_byte: u8) {
        // take the decimal value of Vx, place the hundres digit in memory[I], tens in memory[I + 1], and ones in memory[I+2]
        let sn = high_byte & 0x0F;
        let value = self.registers[sn as usize];
        self.memory[self.address_register as usize] = (value / 100);
        self.memory[self.address_register as usize + 1] = (value % 100) / 10;
        self.memory[self.address_register as usize + 2] = value % 10;
    }

    fn ld_i_vx(&mut self, high_byte: u8) {
        // store registers V0..Vx in memory starting at memory[I]
        let sn = high_byte & 0x0F;
        println!("sn: {}", sn);
        for i in 0..=sn as usize{
            println!("index: {}", i);
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
        println!("Missing opcode! {:#06x} \n{:?}", opcode, *self);
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



#[cfg(test)]
mod tests {
    use super::*;

    fn set_up(opcode: u16, instruction: Instruction) -> (Emulator, Keyboard) {
        assert_eq!(Instruction::parse_opcode(opcode), instruction);
        let e = Emulator::new();
        let k = Keyboard::new();
        (e, k)
    }


    // This function is just so I dont have to rewrite all my tests. Yes I know its bad
    fn no_screen_test(opcode: u16, e: &mut Emulator, k: &Keyboard) {
        let mut s = Screen::new(1, 1);
        e.emulate(opcode, k, &mut s)
    }

    #[test]
    fn jp_addr() {
        let opcode = 0x12F3;
        let (mut e, k) = set_up(opcode, Instruction::JP_addr);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, 0x2F3);
    }

    #[test]
    fn cls() {
        let opcode = 0x00E0;
        let (mut e, k) = set_up(opcode, Instruction::CLS);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(1, 0);
    }

    #[test]
    fn ret() {
        let opcode = 0x00EE;
        let (mut e, k) = set_up(opcode, Instruction::RET);
        e.stack[1] = 0x123;
        e.sp = 1;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, 0x123);
        assert_eq!(e.sp, 0);
    }

    #[test]
    fn call_addr() {
        let opcode: u16 = 0x23E4;
        let (mut e, k) = set_up(opcode, Instruction::CALL_addr);
        e.pc = 0x123;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.sp, 1);
        assert_eq!(e.stack[1], 0x123);
        assert_eq!(e.pc, 0x3E4);
    }

    #[test]
    fn se_vx() {
        let opcode: u16 = 0x3123;
        let (mut e, k) = set_up(opcode, Instruction::SE_Vx);
        e.registers[1] = 0x23;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index + 2) as u16);
        no_screen_test(0x3111, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index + 2) as u16);
    }

    #[test]
    fn sne_vx() {
        let opcode: u16 = 0x4123;
        let (mut e, k) = set_up(opcode, Instruction::SNE_Vx);
        e.registers[1] = 0x23;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index) as u16);
        no_screen_test(0x4111, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index + 2) as u16);
    }

    #[test]
    fn se_vx_vy() {
        let opcode: u16 = 0x5120;
        let (mut e, k) = set_up(opcode, Instruction::SE_Vx_Vy);
        e.registers[1] = 0xF1;
        e.registers[2] = 0xF1;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index + 2)as u16);
        e.registers[2] = 0xFF;
        no_screen_test(0x5120, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index + 2) as u16);
    }

    #[test]
    fn ld_vx() {
        let opcode: u16 = 0x6123;
        let (mut e, k) = set_up(opcode, Instruction::LD_Vx);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x23);
    }
    
    #[test]
    fn add_vx() {
        let opcode: u16 = 0x7123;
        let (mut e, k) = set_up(opcode, Instruction::ADD_Vx);
        e.registers[1] = 15;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 15 + 0x23);
    }

    #[test]
    fn ld_vx_vy() {
        let opcode: u16 = 0x8120;
        let (mut e, k) = set_up(opcode, Instruction::LD_Vx_Vy);
        e.registers[2] = 0xF3;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], e.registers[2]);
    }

    #[test]
    fn or_vx_vy() {
        let opcode: u16 = 0x8121;
        let (mut e, k) = set_up(opcode, Instruction::OR_Vx_Vy);
        e.registers[1] = 0xF0;
        e.registers[2] = 0x0F;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0xFF);
    }

    #[test]
    fn and_vx_vy() {
        let opcode: u16 = 0x8122;
        let (mut e, k) = set_up(opcode, Instruction::AND_Vx_Vy);
        e.registers[1] = 0xA0;
        e.registers[2] = 0xDF;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x80);
    }

    #[test]
    fn xor_vx_vy() {
        let opcode: u16 = 0x8123;
        let (mut e, k) = set_up(opcode, Instruction::XOR_Vx_Vy);
        e.registers[1] = 0xA0;
        e.registers[2] = 0xDF;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x7F);
    }

    #[test]
    fn add_vx_vy() {
        let opcode: u16 = 0x8124;
        let (mut e, k) = set_up(opcode, Instruction::ADD_Vx_Vy);
        e.registers[1] = 12;
        e.registers[2] = 24;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 36);
        assert_eq!(e.registers[e.flag_register_index], 0);
        e.registers[1] = 254;
        e.registers[2] = 2;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0);
        assert_eq!(e.registers[e.flag_register_index], 1);
    }

    #[test]
    fn sub_vx_vy() {
        let opcode: u16 = 0x8125;
        let (mut e, k) = set_up(opcode, Instruction::SUB_Vx_Vy);
        e.registers[1] = 12;
        e.registers[2] = 6;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 6);
        assert_eq!(e.registers[e.flag_register_index], 1);
        e.registers[2] = 12;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 250);
        assert_eq!(e.registers[e.flag_register_index], 0);
    }


    #[test]
    fn shr_vx() {
        let opcode: u16 = 0x8126;
        let (mut e, k) = set_up(opcode, Instruction::SHR_Vx);
        e.registers[1] = 0x0D;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x06);
        assert_eq!(e.registers[e.flag_register_index], 1);
        e.registers[1] = 0xFE;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x7F);
        assert_eq!(e.registers[e.flag_register_index], 0);
    }

    #[test]
    fn subn_vx_vy() {
        let opcode: u16 = 0x8127;
        let (mut e, k) = set_up(opcode, Instruction::SUBN_Vx_Vy);
        e.registers[1] = 6;
        e.registers[2] = 12;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 6);
        assert_eq!(e.registers[e.flag_register_index], 1);
        e.registers[2] = 1;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 251);
        assert_eq!(e.registers[e.flag_register_index], 0);
    }

    #[test]
    fn shl_vx() {
        let opcode: u16 = 0x812E;
        let (mut e, k) = set_up(opcode, Instruction::SHL_Vx);
        e.registers[1] = 0xF0;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0xE0);
        assert_eq!(e.registers[e.flag_register_index], 1);
        e.registers[1] = 0x7E;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0xFC);
        assert_eq!(e.registers[e.flag_register_index], 0);
    }

    #[test]
    fn sne_vx_vy() {
        let opcode: u16 = 0x9120;
        let (mut e, k) = set_up(opcode, Instruction::SNE_Vx_Vy);
        e.registers[1] = 0xF0;
        e.registers[2] = 0xF0;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, e.program_memory_index as u16);
        e.registers[2] = 0xFF;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc,(e.program_memory_index as u16) + 2);
    }

    #[test]
    fn ld_i() {
        let opcode: u16 = 0xA111;
        let (mut e, k) = set_up(opcode, Instruction::LD_I);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.address_register, 0x0111);
        no_screen_test(0xAF02, &mut e, &k);
        assert_eq!(e.address_register, 0x0F02);
    }

    #[test]
    fn jp_v0() {
        let opcode: u16 = 0xB111;
        let (mut e, k) = set_up(opcode, Instruction::JP_V0);
        e.registers[0] = 0x72;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, 0x0183);
        no_screen_test(0xB100, &mut e, &k);
        assert_eq!(e.pc, 0x0172);
    }

    /* Currently not testing rng, have manually verified function.
        May add unit tests in the future
    #[test]
    fn rnd_vx() {
        let opcode: u16 = 0xC1FF;
        let (mut e, k) = set_up(opcode, Instruction::RND_Vx);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(1,0);
    }
    */

    // TODO
    #[test]
    fn drw_vx_vy() {
        // let opcode: u16 = 0xD122;
        // let (mut e, k) = set_up(opcode, Instruction::SYS_addr);
        // no_screen_test(opcode, &mut e, &k);
        assert_eq!(1,0);
    }

    #[test]
    fn skp_vx() {
        let opcode: u16 = 0xE19E;
        let (mut e, mut k) = set_up(opcode, Instruction::SKP_Vx);
        e.registers[1] = 0x03;
        k.set_key(3, Key::Down);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16) + 2);
        no_screen_test(0xE29, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16) + 2);
        k.set_key(3, Key::Up);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16) + 2);
    }

    #[test]
    fn sknp_vx() {
        let opcode: u16 = 0xE1A1;
        let (mut e, mut k) = set_up(opcode, Instruction::SKNP_Vx);
        e.registers[1] = 0x03;
        k.set_key(3, Key::Down);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16));
        no_screen_test(0xE2A1, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16) + 2);
        k.set_key(3, Key::Up);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.pc, (e.program_memory_index as u16) + 4);
    }

    
    #[test]
    fn ld_vx_dt() {
        let opcode: u16 = 0xF107;
        let (mut e, k) = set_up(opcode, Instruction::LD_Vx_DT);
        e.delay_timer_register = 0x11;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x11);
        e.delay_timer_register = 0xF3;
        no_screen_test(0xF207, &mut e, &k);
        assert_eq!(e.registers[1], 0x11);
        assert_eq!(e.registers[2], 0xF3);
    }

    #[test]
    fn ld_vx_k() {
        // execution will be paused in outer loop for this opcode
        let opcode: u16 = 0xF10A;
        let (mut e, mut k) = set_up(opcode, Instruction::LD_Vx_K);
        k.set_key(3, Key::Down);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 3);
        k.set_key(3, Key::Up);
        k.set_key(10, Key::Down);
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[1], 0x0A);
    }

    #[test]
    fn ld_dt_vx() {
        let opcode: u16 = 0xF115;
        let (mut e, k) = set_up(opcode, Instruction::LD_DT_Vx);
        e.registers[1] = 0xFF;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.delay_timer_register, 0xFF);
        e.registers[0] = 0xAB;
        no_screen_test(0xF015, &mut e, &k);
        assert_eq!(e.delay_timer_register, 0xAB);
    }

    #[test]
    fn ld_st_vx() {
        let opcode: u16 = 0xF118;
        let (mut e, k) = set_up(opcode, Instruction::LD_ST_Vx);
        e.registers[1] = 0x1C;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.sound_timer_register, 0x1C);
        e.registers[0xA] = 0x02;
        no_screen_test(0xFA18, &mut e, &k);
        assert_eq!(e.sound_timer_register, 0x02);
    }

    #[test]
    fn add_i_vx() {
        let opcode: u16 = 0xF11E;
        let (mut e, k) = set_up(opcode, Instruction::ADD_I_Vx);
        e.address_register = 0xF12;
        e.registers[1] = 0x3A;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.address_register, 0xF4C);
        e.registers[2] = 0x11;
        no_screen_test(0xF21E, &mut e, &k);
        assert_eq!(e.address_register, 0xF5D);
    }

    #[test]
    fn ld_f_xv() {
        let opcode: u16 = 0xF129;
        let (mut e, k) = set_up(opcode, Instruction::LD_F_Vx);
        e.registers[1] = 0x03;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.address_register, 15);
        e.registers[3] = 0x0A;
        no_screen_test(0xF329, &mut e, &k);
        assert_eq!(e.address_register, 50);
    }

    #[test]
    fn ld_b_vx() {
        let opcode: u16 = 0xF133;
        let (mut e, k) = set_up(opcode, Instruction::LD_B_Vx);
        e.registers[1] = 0xFF; //255
        e.address_register = 0xF12;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.memory[e.address_register as usize], 2);
        assert_eq!(e.memory[e.address_register as usize + 1], 5);
        assert_eq!(e.memory[e.address_register as usize + 2], 5);
        e.registers[1] = 0x4C;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.memory[e.address_register as usize], 0);
        assert_eq!(e.memory[e.address_register as usize + 1], 7);
        assert_eq!(e.memory[e.address_register as usize + 2], 6);
        e.registers[1] = 0x02;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.memory[e.address_register as usize], 0);
        assert_eq!(e.memory[e.address_register as usize + 1], 0);
        assert_eq!(e.memory[e.address_register as usize + 2], 2);

    }

    #[test]
    fn ld_i_vx() {
        let opcode: u16 = 0xFE55;
        let (mut e, k) = set_up(opcode, Instruction::LD_I_Vx);
        e.address_register = 0xF13;
        e.registers[0] = 0x12;
        e.registers[1] = 0xFF;
        e.registers[2] = 0x01;
        e.registers[3] = 0xAB;
        e.registers[4] = 0x77;
        e.registers[5] = 0x23;
        e.registers[6] = 0xB7;
        e.registers[7] = 0x4B;
        e.registers[8] = 0xBB;
        e.registers[9] = 0x2B;
        e.registers[0xA] = 0xBB;
        e.registers[0xB] = 0xA2;
        e.registers[0xC] = 0xBB;
        e.registers[0xD] = 0xBB;
        e.registers[0xE] = 0xBB;

        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.memory[e.address_register as usize + 0], 0x12);
        assert_eq!(e.memory[e.address_register as usize + 1], 0xFF);
        assert_eq!(e.memory[e.address_register as usize + 2], 0x01);
        assert_eq!(e.memory[e.address_register as usize + 3], 0xAB);
        assert_eq!(e.memory[e.address_register as usize + 4], 0x77);
        assert_eq!(e.memory[e.address_register as usize + 5], 0x23);
        assert_eq!(e.memory[e.address_register as usize + 6], 0xB7);
        assert_eq!(e.memory[e.address_register as usize + 7], 0x4B);
        assert_eq!(e.memory[e.address_register as usize + 8], 0xBB);
        assert_eq!(e.memory[e.address_register as usize + 9], 0x2B);
        assert_eq!(e.memory[e.address_register as usize + 10], 0xBB);
        assert_eq!(e.memory[e.address_register as usize + 11], 0xA2);
        assert_eq!(e.memory[e.address_register as usize + 12], 0xBB);
        assert_eq!(e.memory[e.address_register as usize + 13], 0xBB);
        assert_eq!(e.memory[e.address_register as usize + 14], 0xBB);


    }

    #[test]
    fn ld_vx_i() {
        let opcode: u16 = 0xFE65;
        let (mut e, k) = set_up(opcode, Instruction::LD_Vx_I);
        e.address_register = 0xF42;
        e.memory[e.address_register as usize + 0] = 0x11;
        e.memory[e.address_register as usize + 1] = 0x12;
        e.memory[e.address_register as usize + 2] = 0x13;
        e.memory[e.address_register as usize + 3] = 0x14;
        e.memory[e.address_register as usize + 4] = 0x15;
        e.memory[e.address_register as usize + 5] = 0x16;
        e.memory[e.address_register as usize + 6] = 0x17;
        e.memory[e.address_register as usize + 7] = 0x18;
        e.memory[e.address_register as usize + 8] = 0x19;
        e.memory[e.address_register as usize + 9] = 0x1A;
        e.memory[e.address_register as usize + 10] = 0x75;
        e.memory[e.address_register as usize + 11] = 0x65;
        e.memory[e.address_register as usize + 12] = 0x45;
        e.memory[e.address_register as usize + 13] = 0x35;
        e.memory[e.address_register as usize + 14] = 0x34;
        no_screen_test(opcode, &mut e, &k);
        assert_eq!(e.registers[0], 0x11);
        assert_eq!(e.registers[1], 0x12);
        assert_eq!(e.registers[2], 0x13);
        assert_eq!(e.registers[3], 0x14);
        assert_eq!(e.registers[4], 0x15);
        assert_eq!(e.registers[5], 0x16);
        assert_eq!(e.registers[6], 0x17);
        assert_eq!(e.registers[7], 0x18);
        assert_eq!(e.registers[8], 0x19);
        assert_eq!(e.registers[9], 0x1A);
        assert_eq!(e.registers[10], 0x75);
        assert_eq!(e.registers[11], 0x65);
        assert_eq!(e.registers[12], 0x45);
        assert_eq!(e.registers[13], 0x35);
        assert_eq!(e.registers[14], 0x34);

    }
}