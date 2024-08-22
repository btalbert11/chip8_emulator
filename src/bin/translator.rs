use chip8_emulator::instruction::Instruction;
use std::{env, process::exit, fs, io::Write};


fn translate_instruction(first_byte: u8, second_byte: u8) -> Option<String> {
    let opcode = ((first_byte as u16) << 8) | (second_byte as u16);
    let sn = first_byte & 0x0F;
    let tn = (second_byte & 0xF0) >> 4;
    let fourth_nibble = second_byte & 0x0F;
    match Instruction::parse_opcode(opcode) {
        Instruction::SYS_addr => Some(String::from(format!("SYS_addr, {}", opcode & 0x0FFF))),
        Instruction::CLS => Some(String::from("CLS")),
        Instruction::RET => Some(String::from("RET")),
        Instruction::JP_addr => Some(String::from(format!("JP_addr, {:#06x}", opcode & 0x0FFF))),
        Instruction::CALL_addr => Some(String::from(format!("CALL_addr, {:#06x}", opcode & 0x0FFF))),
        Instruction::SE_Vx => Some(String::from(format!("SE_Vx, V{}, {:#04x}", sn, second_byte))),
        Instruction::SNE_Vx => Some(String::from(format!(""))),
        Instruction::SE_Vx_Vy => Some(String::from(format!(""))),
        Instruction::LD_Vx => Some(String::from(format!("LD_Vx, V{}, {:#04x}", sn, second_byte))),
        Instruction::ADD_Vx => Some(String::from(format!("ADD_Vx, V{}, {:#04x}", sn, second_byte))),
        Instruction::LD_Vx_Vy => Some(String::from(format!("LD_Vx_Vy, V{}, V{}", sn, tn))),
        Instruction::OR_Vx_Vy => Some(String::from(format!(""))),
        Instruction::AND_Vx_Vy => Some(String::from(format!(""))),
        Instruction::XOR_Vx_Vy => Some(String::from(format!(""))),
        Instruction::ADD_Vx_Vy => Some(String::from(format!(""))),
        Instruction::SUB_Vx_Vy => Some(String::from(format!(""))),
        Instruction::SHR_Vx => Some(String::from(format!(""))),
        Instruction::SUBN_Vx_Vy => Some(String::from(format!(""))),
        Instruction::SHL_Vx => Some(String::from(format!(""))),
        Instruction::SNE_Vx_Vy => Some(String::from(format!(""))),
        Instruction::LD_I => Some(String::from(format!("LD_I, {:#06x}", opcode & 0x0FFF))),
        Instruction::JP_V0 => Some(String::from(format!(""))),
        Instruction::RND_Vx => Some(String::from(format!("RND_Vx, V{:#04x}, {:#04x}", sn, second_byte))),
        Instruction::DRW_Vx_Vy => Some(String::from(format!("DRW_Vx_Vy, V{}, V{}, {:#04x}", sn, tn, fourth_nibble))),
        Instruction::SKP_Vx => Some(String::from(format!(""))),
        Instruction::SKNP_Vx => Some(String::from(format!(""))),
        Instruction::LD_Vx_DT => Some(String::from(format!(""))),
        Instruction::LD_Vx_K => Some(String::from(format!(""))),
        Instruction::LD_DT_Vx => Some(String::from(format!(""))),
        Instruction::LD_ST_Vx => Some(String::from(format!(""))),
        Instruction::ADD_I_Vx => Some(String::from(format!(""))),
        Instruction::LD_F_Vx => Some(String::from(format!(""))),
        Instruction::LD_B_Vx => Some(String::from(format!(""))),
        Instruction::LD_I_Vx => Some(String::from(format!(""))),
        Instruction::LD_Vx_I => Some(String::from(format!(""))),
        Instruction::SCD_nibble => Some(String::from(format!(""))),
        Instruction::SCR => Some(String::from(format!(""))),
        Instruction::SCL => Some(String::from(format!(""))),
        Instruction::EXIT => Some(String::from(format!(""))),
        Instruction::LOW => Some(String::from(format!(""))),
        Instruction::HIGH => Some(String::from(format!(""))),
        Instruction::DRW_Vx_Vy_0 => Some(String::from(format!(""))),
        Instruction::LD_HF_Vx => Some(String::from(format!(""))),
        Instruction::LD_R_Vx => Some(String::from(format!(""))),
        Instruction::LD_Vx_R => Some(String::from(format!(""))),
        Instruction::Invalid_Instruction => None,
    }
}

fn main() {

    println!("{}", std::env::current_dir().unwrap().display());

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() != 3 {
        println!("Please provide a source file and a destination file");
        exit(-1);
    }

    let source = fs::read(&args[1])
        .expect("ROM not found");

    let mut destination = fs::File::create(&args[2])
        .expect("Destination file already exists");

    
    let mut stream = source.iter();
    while let Some(first_byte) = stream.next() {
        if let Some(second_byte) = stream.next() {
            println!("bytes: {:#04x}, {:#04x}", *first_byte, *second_byte);

            if let Some(mut instrution_string) = translate_instruction(*first_byte, *second_byte) {
                instrution_string.push('\n');
                destination.write(instrution_string.as_bytes());
            }
            else {
                println!("INVALID INSTRUTION: {:#04x}, {:#04x}", *first_byte, *second_byte);
                exit(-1);
            };
        }
    }



}