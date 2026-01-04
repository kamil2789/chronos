use crate::{game_engine::RendererType, window::ChronosWindow};

pub mod shader_source;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("File could not be opened, path: {0}")]
    ShaderSourceFile(String),
    #[error("Shader compilation error: {0}")]
    Compilation(String),
    #[error("Shader link error: {0}")]
    Link(String),
    #[error("Renderer initialization error: {0}")]
    Initialization(String),
}

#[allow(dead_code)]
pub enum ShaderId {
    Vulkan(u64),
}

pub trait Renderer {
    fn compile_shader(&mut self, source: &shader_source::ShaderSource) -> Result<ShaderId>;
}

pub fn init_render(
    _window: &ChronosWindow,
    renderer_type: &RendererType,
) -> Result<Box<dyn Renderer>> {
    match renderer_type {
        RendererType::Wgpu => unimplemented!("Wgpu renderer is not implemented yet"),
        RendererType::Vulkan => unimplemented!("Vulkan renderer is not implemented yet"),
    }
}
