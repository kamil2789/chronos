use std::sync::Arc;

use winit::window::Window;

use crate::{components::color::RGBA, game_engine::RendererType, renderer::wgpu::WgpuRenderer};

pub mod shader_source;
pub mod wgpu;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    /*
    #[error("File could not be opened, path: {0}")]
    ShaderSourceFile(String),
    #[error("Shader compilation error: {0}")]
    Compilation(String),
    #[error("Shader link error: {0}")]
    Link(String),
    */
    #[error("Renderer initialization error: {0}")]
    Initialization(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("Surface error: {0}")]
    Surface(String),
}

#[allow(dead_code)]
pub enum ShaderId {
    Vulkan(u64),
}

pub trait Renderer {
    //fn compile_shader(&mut self, source: &shader_source::ShaderSource) -> Result<ShaderId>;
    fn render(&mut self) -> Result<()>;
    fn resize(&mut self, _width: u32, _height: u32);
    fn set_background_color(&mut self, color: &RGBA);
}

pub fn init_render(window: Arc<Window>, renderer_type: &RendererType) -> Result<Box<dyn Renderer>> {
    match renderer_type {
        RendererType::Wgpu => {
            let render = match pollster::block_on(WgpuRenderer::new(window)) {
                Ok(renderer) => Box::new(renderer),
                Err(_) => {
                    return Err(RendererError::Initialization(
                        "Failed to initialize WGPU renderer".to_string(),
                    ));
                }
            };
            Ok(render)
        }
    }
}
