pub mod opengl;
pub mod shader_source;

pub type Result<T> = std::result::Result<T, RendererError>;
pub type ShaderId = u32;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("File could not be opened, path: {0}")]
    ShaderSourceFileError(String),
    #[error("Shader compilation error: {0}")]
    CompilationError(String),
    #[error("Shader link error: {0}")]
    LinkError(String),
}

pub trait Renderer {
    fn compile_shader(&mut self, source: &shader_source::ShaderSource) -> Result<ShaderId>;
}
