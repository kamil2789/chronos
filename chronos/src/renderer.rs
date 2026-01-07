pub mod shader_source;
pub mod wgpu;

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
