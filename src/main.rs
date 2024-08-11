use chip8_emulator::Emulator;


fn main() {
    let mut e = Emulator::new();
    println!("Hello, world!");
    println!("{:?}", e);

    e.emulate(0x00E0);
}
