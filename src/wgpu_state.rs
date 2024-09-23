#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{EventLoop, EventLoopWindowTarget}, 
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};



// pub struct WgpuState<'a> {
//     // state for wgpu
//     surface: wgpu::Surface<'a>,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: winit::dpi::PhysicalSize<u32>,
//     window: &'a Window,
//     render_pipeline: wgpu::RenderPipeline,
//     bind_group: wgpu::BindGroup,
//     texture: wgpu::Texture,
//     // texture_data: Vec<Rgb>,
//     // // vars to keep track of misc state
//     surface_configured: bool,
//     k_texture_width: u32,
//     k_texture_height: u32,
// }