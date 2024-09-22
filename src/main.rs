use pixels::{Error, Pixels, SurfaceTexture};
use std::{env, fs, process::exit};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use chip8_emulator::run;



fn main() {
    let _ = run();

}
