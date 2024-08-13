use chip8_emulator::{Emulator, Keyboard};
use std::fs;


fn load_rom(filename: &str, e: Emulator) {
    let contents = fs::read_to_string(filename)
        .expect("Rom file not found.");
    

}

fn main() {
    let mut e = Emulator::new();
    let mut k = Keyboard::new();
    println!("Hello, world!");
    println!("{:?}", e);

    // e.emulate(0x00E0, &k);
    e.emulate(0x1723, &k);
    e.emulate(0x00EE, &k);
    e.emulate(0x3F23, &k);
    e.emulate(0x5AF0, &k);
}
