use chip8_emulator::emulator::Emulator;
use chip8_emulator::keyboard::Keyboard;
use chip8_emulator::instruction::Instruction;
use chip8_emulator::screen::Screen;
use std::{time::{ Duration, Instant}, process::exit, env, fs};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use pixels::{Error, Pixels, SurfaceTexture};


const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn load_rom(filename: &str, e: &mut Emulator) {
    let contents = fs::read(filename)
        .expect("Rom file not found.");
    // dbg!(&contents);
    for i in 0..contents.len() {
        e.set_memory(contents[i], i + 0x200);
    }

}



fn main() -> Result<(), Error>{
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() != 2 {
        print!("Needs a filename");
        exit(-1);
    }

    let mut test_screen = Screen::new(WIDTH, HEIGHT);
    for j in (0..WIDTH).step_by(8) {
       for i in 0..HEIGHT {
           test_screen.set_byte_pixels(0xCC, j,i);
       }
    }

    dbg!(test_screen.screen_to_render());

    let mut e = Emulator::new();
    let mut k = Keyboard::new();
    let mut s = Screen::new(WIDTH, HEIGHT);
    load_rom(&args[1], &mut e);
    // e.print_memory();
    // println!("{:?}", e);


    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Chip8")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop).unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };


    let mut time_mark = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        e.emulate_step(&k, &mut s);

        if let Event::RedrawRequested(_) = event {
            draw_pixels(pixels.frame_mut(), &s.screen_to_render());
            if let Err(err) = pixels.render() {
                print!("PIXEL DRAW ERROR");
                *control_flow = ControlFlow::Exit;
                return
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("PIXEL RESIZE ERROR");
                    *control_flow = ControlFlow::Exit;
                    return
                }
            }
        }

        // let now = Instant::now();
        // if let Some(time_passed) = now.checked_duration_since(time_mark) {
        //     if time_passed.as_micros() > 16 {
        //         println!("time passed = {}, redraw requested", time_passed.as_micros());
        //         window.request_redraw();
        //         time_mark = Instant::now();
        //     }
        // }
        window.request_redraw();

    });

    Ok(())
}

fn draw_pixels(pixels_buffer: &mut [u8], screen_buffer: &Vec<[u8; 4]>) {
    println!("DRAW CALLED");
    for (pixel, cell) in pixels_buffer.chunks_exact_mut(4).zip(screen_buffer.iter()) {
        pixel.copy_from_slice(cell);
    }
}
