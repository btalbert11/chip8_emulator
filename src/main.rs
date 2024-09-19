use chip8_emulator::emulator::Emulator;
use chip8_emulator::keyboard::{Key, Keyboard};
use chip8_emulator::screen::Screen;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{env, fs, process::exit};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// TODO implement other screen sizes for different chip8 instruction sets
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn load_rom(filename: &str, e: &mut Emulator) {
    let contents = fs::read(filename).expect("Rom file not found.");
    for i in 0..contents.len() {
        e.set_memory(contents[i], i + e.program_start_address());
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Needs a filename");
        exit(-1);
    }

    let mut e = Emulator::new();
    let mut k = Keyboard::new();
    let mut s = Screen::new(WIDTH, HEIGHT);
    load_rom(&args[1], &mut e);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Chip8")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        e.emulate_step(&k, &mut s);

        if let Event::RedrawRequested(_) = event {
            draw_pixels(pixels.frame_mut(), &s.screen_to_render());
            if let Err(err) = pixels.render() {
                println!("PIXEL DRAW ERROR: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("PIXEL RESIZE ERROR: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // TODO Add an option to change keybindings on start up. Can save in a config file
            // and load that file on startup
            if input.key_pressed(VirtualKeyCode::Numpad7) {
                k.set_key(1, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad7) {
                k.set_key(1, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                k.set_key(2, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Down) {
                k.set_key(2, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Numpad3) {
                k.set_key(3, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad3) {
                k.set_key(3, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                k.set_key(4, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Left) {
                k.set_key(4, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Numpad5) {
                k.set_key(5, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad5) {
                k.set_key(5, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                k.set_key(6, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Right) {
                k.set_key(6, Key::Up)
            }

            if input.key_pressed(VirtualKeyCode::Numpad1) {
                k.set_key(7, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad1) {
                k.set_key(7, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Down) {
                k.set_key(8, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Down) {
                k.set_key(8, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Numpad3) {
                k.set_key(9, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad3) {
                k.set_key(9, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::Numpad0) {
                k.set_key(0, Key::Down);
            }

            if input.key_released(VirtualKeyCode::Numpad0) {
                k.set_key(0, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::A) {
                k.set_key(10, Key::Down);
            }

            if input.key_pressed(VirtualKeyCode::A) {
                k.set_key(10, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::B) {
                k.set_key(11, Key::Down);
            }

            if input.key_released(VirtualKeyCode::B) {
                k.set_key(11, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::C) {
                k.set_key(12, Key::Down);
            }

            if input.key_released(VirtualKeyCode::C) {
                k.set_key(12, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::D) {
                k.set_key(13, Key::Down);
            }

            if input.key_released(VirtualKeyCode::D) {
                k.set_key(13, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::E) {
                k.set_key(14, Key::Down);
            }

            if input.key_released(VirtualKeyCode::E) {
                k.set_key(14, Key::Up);
            }

            if input.key_pressed(VirtualKeyCode::F) {
                k.set_key(15, Key::Down);
            }

            if input.key_released(VirtualKeyCode::F) {
                k.set_key(15, Key::Up);
            }
        }
        // TODO instead of redrawing the frame every loop, can either send a signal from emulator when
        // a drw instrctuion is run, or just check the instruction in this loop
        window.request_redraw();
    });
}

fn draw_pixels(pixels_buffer: &mut [u8], screen_buffer: &Vec<[u8; 4]>) {
    for (pixel, cell) in pixels_buffer.chunks_exact_mut(4).zip(screen_buffer.iter()) {
        pixel.copy_from_slice(cell);
    }
}
