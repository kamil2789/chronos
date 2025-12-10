pub mod shader_source;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("File could not be opened, path: {0}")]
    ShaderSourceFileError(String),
    #[error("Error when loading data from a file ")]
    ReadFileError(#[from] std::io::Error),
}

pub trait Render {}
