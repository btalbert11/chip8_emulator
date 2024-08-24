use chip8_emulator::instruction::Instruction;
use std::{env, process::exit, fs, io::Write};


fn translate_instruction(pc: u16, first_byte: u8, second_byte: u8) -> Option<String> {
    let opcode = ((first_byte as u16) << 8) | (second_byte as u16);
    let sn = first_byte & 0x0F;
    let tn = (second_byte & 0xF0) >> 4;
    let fourth_nibble = second_byte & 0x0F;
    let s = String::from(format!("{:#06x}: ", pc));
    match Instruction::parse_opcode(opcode) {
        Instruction::SYS_addr => Some(format!("{}SYS_addr, {}", s, opcode & 0x0FFF)),
        Instruction::CLS => Some(format!("{}CLS", s)),
        Instruction::RET => Some(format!("{}RET", s)),
        Instruction::JP_addr => Some(format!("{}JP_addr, {:#06x}", s, opcode & 0x0FFF)),
        Instruction::CALL_addr => Some(format!("{}CALL_addr, {:#06x}", s, opcode & 0x0FFF)),
        Instruction::SE_Vx => Some(format!("{}SE_Vx, V{}, {:#04x}", s, sn, second_byte)),
        Instruction::SNE_Vx => Some(format!("{}", s)),
        Instruction::SE_Vx_Vy => Some(format!("{}", s)),
        Instruction::LD_Vx => Some(format!("{}LD_Vx, V{}, {:#04x}", s, sn, second_byte)),
        Instruction::ADD_Vx => Some(format!("{}ADD_Vx, V{}, {:#04x}", s, sn, second_byte)),
        Instruction::LD_Vx_Vy => Some(format!("{}LD_Vx_Vy, V{}, V{}", s, sn, tn)),
        Instruction::OR_Vx_Vy => Some(format!("{}", s)),
        Instruction::AND_Vx_Vy => Some(format!("{}", s)),
        Instruction::XOR_Vx_Vy => Some(format!("{}", s)),
        Instruction::ADD_Vx_Vy => Some(format!("{}", s)),
        Instruction::SUB_Vx_Vy => Some(format!("{}", s)),
        Instruction::SHR_Vx => Some(format!("{}", s)),
        Instruction::SUBN_Vx_Vy => Some(format!("{}", s)),
        Instruction::SHL_Vx => Some(format!("{}", s)),
        Instruction::SNE_Vx_Vy => Some(format!("{}", s)),
        Instruction::LD_I => Some(format!("{}LD_I, {:#06x}", s, opcode & 0x0FFF)),
        Instruction::JP_V0 => Some(format!("{}", s)),
        Instruction::RND_Vx => Some(format!("{}RND_Vx, V{}, {:#04x}", s, sn, second_byte)),
        Instruction::DRW_Vx_Vy => Some(format!("{}DRW_Vx_Vy, V{}, V{}, {:#04x}", s, sn, tn, fourth_nibble)),
        Instruction::SKP_Vx => Some(format!("{}", s)),
        Instruction::SKNP_Vx => Some(format!("{}", s)),
        Instruction::LD_Vx_DT => Some(format!("{}", s)),
        Instruction::LD_Vx_K => Some(format!("{}", s)),
        Instruction::LD_DT_Vx => Some(format!("{}", s)),
        Instruction::LD_ST_Vx => Some(format!("{}", s)),
        Instruction::ADD_I_Vx => Some(format!("{}", s)),
        Instruction::LD_F_Vx => Some(format!("{}", s)),
        Instruction::LD_B_Vx => Some(format!("{}", s)),
        Instruction::LD_I_Vx => Some(format!("{}", s)),
        Instruction::LD_Vx_I => Some(format!("{}", s)),
        Instruction::SCD_nibble => Some(format!("{}", s)),
        Instruction::SCR => Some(format!("{}", s)),
        Instruction::SCL => Some(format!("{}", s)),
        Instruction::EXIT => Some(format!("{}", s)),
        Instruction::LOW => Some(format!("{}", s)),
        Instruction::HIGH => Some(format!("{}", s)),
        Instruction::DRW_Vx_Vy_0 => Some(format!("{}", s)),
        Instruction::LD_HF_Vx => Some(format!("{}", s)),
        Instruction::LD_R_Vx => Some(format!("{}", s)),
        Instruction::LD_Vx_R => Some(format!("{}", s)),
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

    
    let mut pc: u16 = 0x200;

    let mut stream = source.iter();
    while let Some(first_byte) = stream.next() {
        if let Some(second_byte) = stream.next() {
            println!("bytes: {:#04x}, {:#04x}", *first_byte, *second_byte);

            if let Some(mut instrution_string) = translate_instruction(pc, *first_byte, *second_byte) {
                instrution_string.push('\n');
                destination.write(instrution_string.as_bytes());
                pc += 2;
            }
            else {
                println!("INVALID INSTRUTION: {:#04x}, {:#04x}", *first_byte, *second_byte);
                exit(-1);
            };
        }
    }



}