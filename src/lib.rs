use emulator::Emulator;
use keyboard::{Key, Keyboard};
use screen::Screen;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{
    env, 
    fs, 
    process::exit,
    io::{stdout, stdin, Write},
};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop, ActiveEventLoop},
    application::ApplicationHandler,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
use chrono::{Local, DateTime};
#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

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

#[cfg(target_arch = "wasm32")]
const BREAKOUT_ROM: &[u8] = include_bytes!("../Breakout.ch8");


fn read_file(e: &mut Emulator, file_path: Option<&String>) -> Result<(), ()>{
    let mut new_rom_file = String::from("");
    match file_path {
        Some(f) => new_rom_file = f.clone(),
        None => {
            println!("Enter new Rom filepath:");
            let _ = stdout().flush();
            match stdin().read_line(&mut new_rom_file) {
                Ok(_) => (),
                Err(_) => {
                    print!("Error reading filename");
                    return Err(());
                }
            }
            new_rom_file = new_rom_file.trim().to_string();

        }
    }

    let contents: Vec<u8> = match fs::read(new_rom_file) {
        Ok(file_contents) => file_contents,
        Err(_) => {
            println!("Rom file not found.");
            return Err(());
        }
    };

    match e.load_rom(contents) {
        Ok(_) => (),
        Err(_) => { 
            print!("Could not load rom");
            return Err(());
        }
    }
    Ok(())
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init_loggers() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
}

struct App {
    window: Option<Window>,
    e: Emulator,
    k: Keyboard,
    s: Screen,
    pixels: Option<Pixels>,
    now: chrono::NaiveTime,
}

impl App {
    pub fn new(e: Emulator, k: Keyboard, s: Screen, now: chrono::NaiveTime, pixels: Option<Pixels>) -> Self {
        App {
            window: None,
            e: e,
            k: k,
            s: s,
            pixels: pixels,
            now: now,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        self.now = Local::now().time();


        if let Some(window) = self.window.as_ref() {
        
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

                    let p: Option<Pixels> = None;
                    let future = async move {
                        p = {
                            let surface_texture = SurfaceTexture::new(WIDTH as u32 * 8, HEIGHT as u32 * 8, &window);
                            match Pixels::new_async(WIDTH as u32, HEIGHT as u32, surface_texture).await {
                                Ok(p) =>  Some(p),
                                Err(_) => panic!("failed to create Pixels object"),
                            }
                        }
                    };
                    wasm_bindgen_futures::spawn_local(future);
                    self.pixels = p;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.pixels = {
                    let surface_texture = SurfaceTexture::new(WIDTH as u32 * 8, HEIGHT as u32 * 8, &window);
                    match Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture) {
                        Ok(p) =>  Some(p),
                        Err(_) => panic!("failed to create Pixels object"),
                    }
                };
            }
        }
        self.window.as_ref().unwrap().request_redraw();
    }
    
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let diff = (Local::now().time() - self.now).num_microseconds().unwrap_or(0);
        if diff > 1851 {
            self.e.emulate_step(&self.k, &mut self.s, diff);
            self.now = Local::now().time();
        }
        
        #[cfg(target_arch = "wasm32")]
        console::log_1(&format!("{:?}", self.pixels.as_ref()).into());
        if let (Some(window), Some(pixels)) = (self.window.as_ref(), self.pixels.as_mut()) {

                match event {
                    // window management
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput { 
                        event: KeyEvent {state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::Escape), ..},
                        ..
                    } => event_loop.exit(),
                    WindowEvent::RedrawRequested => {
                        #[cfg(target_arch = "wasm32")] {
                            console::log_3(&"window id".into(), &format!("{:?}", window_id).into(), &format!("{:?}", window.id()).into());
                            console::log_1(&"Redraw requested".into());
                        }
                        if window_id == window.id() {
                            if window.inner_size().width <= 0 {
                                return;
                            }
                            draw_pixels(pixels.frame_mut(), &self.s.screen_to_render());
                            if let Err(err) = pixels.render() {
                                println!("PIXEL DRAW ERROR: {}", err);
                                event_loop.exit();
                                return;
                            }
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width <= 0 || physical_size.height <= 0 {
                            return;
                        }
                        if let Err(err) = pixels.resize_surface(physical_size.width, physical_size.height) {
                            println!("PIXEL RESIZE ERROR: {}", err);
                            event_loop.exit();
                            return;
                        }
                    },
                    // change rom
                    WindowEvent::KeyboardInput { 
                        event:
                            KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::Enter), ..},
                            ..
                    } => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            match read_file(&mut self.e, None) {
                                Ok(_) => {
                                    self.s.clear_screen();
                                },
                                Err(_) => (),
                            }
                            self.now = Local::now().time();
                        }
                    },
                    // player input management
                    // TODO Add an option to change keybindings on start up. Can save in a config file
                    // Left
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowLeft), ..},
                        .. 
                    } => self.k.set_key(4, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowLeft), ..},
                        .. 
                    } => self.k.set_key(4, Key::Up),
                    // Right
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowRight), ..},
                        .. 
                    } => self.k.set_key(6, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowRight), ..},
                        .. 
                    } => self.k.set_key(6, Key::Up),
                    // Down
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowDown), ..},
                        .. 
                    } => self.k.set_key(8, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowDown), ..},
                        .. 
                    } => self.k.set_key(8, Key::Up),
                    // Up
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::ArrowUp), ..},
                        .. 
                    } => self.k.set_key(2, Key::Down),
                    WindowEvent::KeyboardInput { 
                        event:
                        KeyEvent { state: ElementState::Released, physical_key: PhysicalKey::Code(KeyCode::ArrowUp), ..},
                        .. 
                    } => self.k.set_key(2, Key::Up),
                    
                    
                    _ => (),
                }

                window.request_redraw();
            }

    }
            // TODO readd the rest of the key bindings
           
            // TODO instead of redrawing the frame every loop, can either send a signal from emulator when
            // a drw instrctuion is run, or just check the instruction in this loop
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub async fn run(mut e: Emulator) {
    
    
    // let mut e = Emulator::new();
    let mut k = Keyboard::new();
    let mut s = Screen::new(WIDTH, HEIGHT);
    
    
    #[cfg(target_arch = "wasm32")] {
        e.load_rom(BREAKOUT_ROM.to_vec());
    }
    #[cfg(not(target_arch = "wasm32"))]
    { 
        let args: Vec<String> = env::args().collect();
        let mut filename: String = String::new();
        if args.len() != 2 {
            println!("Needs a filename");
            exit(-1);
        } else {
            filename = String::from(&args[1]);
            let _ = read_file(&mut e, Some(&filename));
        }
    }
    
    let mut now = Local::now().time();
    
    // let window = {
        //     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    //     let scaled_size = LogicalSize::new(WIDTH as f64 * 8.0, HEIGHT as f64 * 8.0);
    //     WindowBuilder::new()
    //     .with_title("Chip8")
    //     .with_inner_size(scaled_size)
    //     .with_min_inner_size(size)
    //     .build(&event_loop)
    //     .unwrap()
    // };
    
    
    let event_loop = EventLoop::new().unwrap();
    
    let mut app = App::new(e, k, s, now, None);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = event_loop.run_app(&mut app);
    #[cfg(target_arch="wasm32")]
    event_loop.spawn_app(app);
    

    // let _ = event_loop.run(move |event, control_flow| {
        
    // });
}

// TODO Pixels needs to be initalized with new_async on web, but the event loop needs to be running to create
// a window for pixels, but the event loop does not have any async method to create pixels in
// This is also true of wgpu in general. There is a hacky work around I found on the web, but I would rather
// just put this project on hold.

/*
//TODO
    Currently I cannot figure out a way to implement a simple file selector from the javascript side.
    I can't send data from JS to the running emulator, since the eventloop need to have ownership of it.
    Options are
    - figure out how to use mutex accross JS and rust
    - figure out how to reload the wasm instance
        This seems harder to figure out than it should, I think I either need to stop using wasmpack
        and manually instanciate the wasm bundle, or use JS Blobs and objectURL.
        I don't think web workers will work from an initial glance, since winit needs access to DOM elements that workers dont have
    - use a file selector in rust that works on web
        Apparently this is not easy to do in winit?
    - add a dependancy to manually send the 'ESC' key to the window so that winit can close before I attempt to create a new one

    This will work if I start by asking the user to upload a rom and then only initializing wasm afterwards.
    As of right now though, I am putting this on hold.

 */