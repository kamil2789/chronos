use crate::{game_engine::RendererType, window::ChronosWindow};

pub mod opengl;
pub mod shader_source;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("File could not be opened, path: {0}")]
    ShaderSourceFileError(String),
    #[error("Shader compilation error: {0}")]
    CompilationError(String),
    #[error("Shader link error: {0}")]
    LinkError(String),
}

#[allow(dead_code)]
pub enum ShaderId {
    OpenGL(glow::Program),
    Vulkan(u64),
}

pub trait Renderer {
    fn compile_shader(&mut self, source: &shader_source::ShaderSource) -> Result<ShaderId>;
}

pub fn init_render(window: &ChronosWindow, renderer_type: RendererType) -> Box<dyn Renderer> {
    match renderer_type {
        RendererType::OpenGL => Box::new(opengl::init_opengl(&window.get_window().unwrap())),
        RendererType::Vulkan => unimplemented!("Vulkan renderer is not implemented yet"),
    }
}
