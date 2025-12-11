use crate::renderer::opengl::OpenGL;
use crate::renderer::shader_source::{ShaderManager, ShaderSource};
use crate::renderer::{Renderer, RendererError};
use crate::window::{ChronosWindow, WinError, WindowConfig};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(#[from] WinError),
    #[error("Renderer error: {0}")]
    RendererError(#[from] RendererError),
}

pub struct ChronosEngine {
    window: ChronosWindow,
    renderer: Box<dyn Renderer>,
    shader_manager: ShaderManager,
}

impl ChronosEngine {
    #[must_use]
    pub fn new(window_config: WindowConfig, renderer: Box<dyn Renderer>) -> Self {
        Self {
            window: ChronosWindow::new(window_config),
            renderer: renderer,
            shader_manager: ShaderManager::default(),
        }
    }

    /// Runs the game engine's main loop.
    ///
    /// # Errors
    ///
    /// Returns an error if the window fails to run or encounters a runtime error.
    pub fn run(&mut self) -> Result<()> {
        self.window.run()?;
        Ok(())
    }

    pub fn load_shader(&mut self, name: &str, shader_source: &ShaderSource) -> Result<()> {
        self.shader_manager
            .register_from_source(name, shader_source);
        self.renderer.compile_shader(shader_source)?;
        Ok(())
    }
}

impl Default for ChronosEngine {
    fn default() -> Self {
        Self {
            window: ChronosWindow::new(WindowConfig::default()),
            renderer: Box::new(OpenGL::default()),
            shader_manager: ShaderManager::default(),
        }
    }
}
