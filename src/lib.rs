use emulator::Emulator;
use keyboard::{Key, Keyboard};
use screen::Screen;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{env, fs, process::exit};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, Window},
};
// use winit_input_helper::WinitInputHelper;
#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

pub mod emulator;
pub mod instruction;
pub mod keyboard;
pub mod screen;
pub mod wgpu_state;


fn draw_pixels(pixels_buffer: &mut [u8], screen_buffer: &Vec<[u8; 4]>) {
    for (pixel, cell) in pixels_buffer.chunks_exact_mut(4).zip(screen_buffer.iter()) {
        pixel.copy_from_slice(cell);
    }
}

// TODO implement other screen sizes for different chip8 instruction sets
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

const BREAKOUT_ROM: &[u8] = include_bytes!("../Breakout.ch8");

fn load_rom(contents: Option<Vec<u8>>, filename: Option<&str>, e: &mut Emulator) {
    let mut c: Vec<u8>;
    if let Some(f) = filename {
        c = fs::read(f).expect("Rom file not found.");
    }
    else if let Some(con) = contents {
        c = con;
    }
    else {
        panic!("No Rom provided");
    }

    for i in 0..c.len() {
        e.set_memory(c[i], i + e.program_start_address());
    }
}

// TODO move this to lib file to make it work on web maybe?
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {

    
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    
    let args: Vec<String> = env::args().collect();
    let mut filename: String = String::new();


    let mut e = Emulator::new();
    let mut k = Keyboard::new();
    let mut s = Screen::new(WIDTH, HEIGHT);
    
    
    #[cfg(target_arch = "wasm32")] {
        load_rom(Some(BREAKOUT_ROM.to_vec()), None, &mut e);
    }
    #[cfg(not(target_arch = "wasm32"))]
    { 
        if args.len() != 2 {
            println!("Needs a filename");
            exit(-1);
        } else {
            filename = String::from(&args[1]);
            load_rom(None, Some(&filename), &mut e);
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    console::log_1(&filename.into());
    
    
    let event_loop = EventLoop::new().unwrap();
    
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
    
    #[cfg(target_arch = "wasm32")]
    {
    // Winit prevents sizing with CSS, so we have to set
    // the size manually when on web.
    use winit::dpi::PhysicalSize;
    let _ = window.request_inner_size(PhysicalSize::new(450, 450));
    
    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("chip8-emulator")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
    }
    
    
    let mut pixels = {
        let window_size = window.inner_size();
        #[cfg(target_arch = "wasm32")]
        console::log_1(&"Made it here".into());
        // #[cfg(target_arch = "wasm32")]
        // panic!("{:?}", window_size.width);        
        let surface_texture = SurfaceTexture::new(WIDTH as u32 * 3, HEIGHT as u32 * 3, &window);
        match Pixels::new_async(WIDTH as u32, HEIGHT as u32, surface_texture).await {
            Ok(p) =>  p,
            Err(_) => panic!("failed to create Pixels object"),
        }
    };
    
    let _ = event_loop.run(move |event, control_flow| {
        e.emulate_step(&k, &mut s);

        match event {
            Event::WindowEvent { 
                window_id, 
                ref event 
            } if window_id == window.id() => {
                match event {
                    // window management
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput { 
                        event: KeyEvent {state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::Escape), ..},
                        ..
                    } => control_flow.exit(),
                    WindowEvent::RedrawRequested if window_id == window.id() => {
                        if window.inner_size().width <= 0 {
                            return;
                        }
                        draw_pixels(pixels.frame_mut(), &s.screen_to_render());
                        if let Err(err) = pixels.render() {
                            println!("PIXEL DRAW ERROR: {}", err);
                            control_flow.exit();
                            return;
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width <= 0 || physical_size.height <= 0 {
                            return;
                        }
                        if let Err(err) = pixels.resize_surface(physical_size.width, physical_size.height) {
                            println!("PIXEL RESIZE ERROR: {}", err);
                            control_flow.exit();
                            return;
                        }
                    },
                    // player input management
                    // Left
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowLeft), ..},
                        .. 
                    } => k.set_key(4, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowLeft), ..},
                        .. 
                    } => k.set_key(4, Key::Up),
                    // Right
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowRight), ..},
                        .. 
                    } => k.set_key(6, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowRight), ..},
                        .. 
                    } => k.set_key(6, Key::Up),
                    // Down
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowDown), ..},
                        .. 
                    } => k.set_key(8, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowDown), ..},
                        .. 
                    } => k.set_key(8, Key::Up),
                    // Up
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowUp), ..},
                        .. 
                    } => k.set_key(2, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowUp), ..},
                        .. 
                    } => k.set_key(2, Key::Up),
                    
                    
                    _ => (),
                }
            }
            _ => (),
        }

        // if input.update(&event) {

        //     // TODO Add an option to change keybindings on start up. Can save in a config file
        //     // and load that file on startup
        //     if input.key_pressed(VirtualKeyCode::Numpad7) {
        //         k.set_key(1, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad7) {
        //         k.set_key(1, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::Numpad3) {
        //         k.set_key(3, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad3) {
        //         k.set_key(3, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::Numpad5) {
        //         k.set_key(5, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad5) {
        //         k.set_key(5, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::Numpad1) {
        //         k.set_key(7, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad1) {
        //         k.set_key(7, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::Numpad3) {
        //         k.set_key(9, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad3) {
        //         k.set_key(9, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::Numpad0) {
        //         k.set_key(0, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::Numpad0) {
        //         k.set_key(0, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::A) {
        //         k.set_key(10, Key::Down);
        //     }

        //     if input.key_pressed(VirtualKeyCode::A) {
        //         k.set_key(10, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::B) {
        //         k.set_key(11, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::B) {
        //         k.set_key(11, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::C) {
        //         k.set_key(12, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::C) {
        //         k.set_key(12, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::D) {
        //         k.set_key(13, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::D) {
        //         k.set_key(13, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::E) {
        //         k.set_key(14, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::E) {
        //         k.set_key(14, Key::Up);
        //     }

        //     if input.key_pressed(VirtualKeyCode::F) {
        //         k.set_key(15, Key::Down);
        //     }

        //     if input.key_released(VirtualKeyCode::F) {
        //         k.set_key(15, Key::Up);
        //     }
        // }
        // TODO instead of redrawing the frame every loop, can either send a signal from emulator when
        // a drw instrctuion is run, or just check the instruction in this loop
        window.request_redraw();
    });
}