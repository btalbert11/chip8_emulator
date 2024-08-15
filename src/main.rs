use chip8_emulator::{Emulator, Keyboard};
use std::{process::exit, env, fs};
// use winit::{
//     event_loop::EventLoop,
//     window::{Window, WindowBuilder},
// };

fn load_rom(filename: &str, e: &mut Emulator) {
    let contents = fs::read(filename)
        .expect("Rom file not found.");
    dbg!(&contents);
    for i in 0..contents.len() {
        e.set_memory(contents[i], i + 0x200);
    }

}

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() != 2 {
        print!("Needs a filename");
        exit(-1);
    }

    let mut e = Emulator::new();
    let mut k = Keyboard::new();
    load_rom(&args[1], &mut e);
    e.print_memory();
    println!("{:?}", e);


    // // TODO write emulate loop
    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    // e.emulate(0x00E0, &k);
    e.emulate(0x1723, &k);
    e.emulate(0x2123, &k);
    e.emulate(0x00EE, &k);
    e.emulate(0x3F23, &k);
    e.emulate(0x5AF0, &k);
}
