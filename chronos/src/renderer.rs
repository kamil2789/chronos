use crate::{components::color::RGBA, graphic_engine::RendererType, renderer::wgpu::WgpuRenderer};
use std::sync::Arc;
use winit::window::Window;

pub mod wgpu;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("Renderer initialization error: {0}")]
    Initialization(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("Surface error: {0}")]
    Surface(String),
    #[error("Shader error: {0}")]
    Shader(String),
}

#[allow(dead_code)]
pub enum ShaderId {
    Vulkan(u64),
}

pub trait Renderer {
    fn render(&mut self, scene: &crate::scene::Scene) -> Result<()>;
    fn render_to_buffer(&mut self, scene: &crate::scene::Scene) -> Result<Vec<u8>>;
    fn resize(&mut self, width: u32, height: u32);
    fn set_background_color(&mut self, color: &RGBA);
}

pub fn init_render(window: Arc<Window>, renderer_type: &RendererType) -> Result<Box<dyn Renderer>> {
    match renderer_type {
        RendererType::Wgpu => {
            let render = pollster::block_on(WgpuRenderer::new(window))?;
            Ok(Box::new(render))
        }
    }
}

pub fn init_headless_render(
    width: u32,
    height: u32,
    renderer_type: &RendererType,
) -> Result<Box<dyn Renderer>> {
    match renderer_type {
        RendererType::Wgpu => {
            let render = pollster::block_on(WgpuRenderer::new_headless(width, height))?;
            Ok(Box::new(render))
        }
    }
}
