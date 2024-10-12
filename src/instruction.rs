#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
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

// TODO add function to translate instructions into opcodes to allow for writing new programs
impl Instruction {
    pub fn parse_opcode(opcode: u16) -> Instruction {
        let high_byte: u8 = ((opcode >> 8) & 0xFF) as u8;
        let low_byte: u8 = (opcode & 0xFF) as u8;
        let first_nibble = high_byte & 0xF0; // always instruction indicator
        let last_nibble = low_byte & 0x0F; // sometimes instruction indicator
        let third_nibble = low_byte & 0xF0; // Sometimes instruction indicator.
        if high_byte == 0x00 && low_byte == 0xEE {
            return Instruction::RET;
        } else if high_byte == 0x00 && low_byte == 0xFB {
            return Instruction::SCR;
        }
        // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFC {
            return Instruction::SCL;
        }
        // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFD {
            return Instruction::EXIT;
        }
        // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFE {
            return Instruction::LOW;
        }
        // Super chip-48 instruction
        else if high_byte == 0x00 && low_byte == 0xFF {
            return Instruction::HIGH;
        }
        // Super chip-48 instruction
        else if first_nibble == 0x00 && third_nibble == 0xE0 {
            return Instruction::CLS;
        } else if first_nibble == 0x00 && third_nibble == 0xC0 {
            return Instruction::SCD_nibble;
        }
        // Super chip-48 instruction
        else if first_nibble == 0x00 {
            return Instruction::SYS_addr;
        } else if first_nibble == 0x10 {
            return Instruction::JP_addr;
        } else if first_nibble == 0x20 {
            return Instruction::CALL_addr;
        } else if first_nibble == 0x30 {
            return Instruction::SE_Vx;
        } else if first_nibble == 0x40 {
            return Instruction::SNE_Vx;
        } else if first_nibble == 0x50 {
            return Instruction::SE_Vx_Vy;
        } else if first_nibble == 0x60 {
            return Instruction::LD_Vx;
        } else if first_nibble == 0x70 {
            return Instruction::ADD_Vx;
        } else if first_nibble == 0x80 && last_nibble == 0x00 {
            return Instruction::LD_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x01 {
            return Instruction::OR_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x02 {
            return Instruction::AND_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x03 {
            return Instruction::XOR_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x04 {
            return Instruction::ADD_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x05 {
            return Instruction::SUB_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x06 {
            return Instruction::SHR_Vx;
        } else if first_nibble == 0x80 && last_nibble == 0x07 {
            return Instruction::SUBN_Vx_Vy;
        } else if first_nibble == 0x80 && last_nibble == 0x0E {
            return Instruction::SHL_Vx;
        } else if first_nibble == 0x90 && last_nibble == 0x00 {
            return Instruction::SNE_Vx_Vy;
        } else if first_nibble == 0xA0 {
            return Instruction::LD_I;
        } else if first_nibble == 0xB0 {
            return Instruction::JP_V0;
        } else if first_nibble == 0xC0 {
            return Instruction::RND_Vx;
        } else if first_nibble == 0xD0 && last_nibble == 0x00 {
            return Instruction::DRW_Vx_Vy_0;
        }
        // Super chip-48 instructions
        else if first_nibble == 0xD0 {
            return Instruction::DRW_Vx_Vy;
        } else if first_nibble == 0xE0 && low_byte == 0x9E {
            return Instruction::SKP_Vx;
        } else if first_nibble == 0xE0 && low_byte == 0xA1 {
            return Instruction::SKNP_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x07 {
            return Instruction::LD_Vx_DT;
        } else if first_nibble == 0xF0 && low_byte == 0x0A {
            return Instruction::LD_Vx_K;
        } else if first_nibble == 0xF0 && low_byte == 0x15 {
            return Instruction::LD_DT_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x18 {
            return Instruction::LD_ST_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x1E {
            return Instruction::ADD_I_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x29 {
            return Instruction::LD_F_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x33 {
            return Instruction::LD_B_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x55 {
            return Instruction::LD_I_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x65 {
            return Instruction::LD_Vx_I;
        }
        // Super chip-48 instructions
        else if first_nibble == 0xF0 && low_byte == 0x30 {
            return Instruction::LD_HF_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x75 {
            return Instruction::LD_R_Vx;
        } else if first_nibble == 0xF0 && low_byte == 0x85 {
            return Instruction::LD_Vx_R;
        } else {
            return Instruction::Invalid_Instruction;
        }
    }
}