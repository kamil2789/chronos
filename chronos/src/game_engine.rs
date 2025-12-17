use crate::renderer::shader_source::{ShaderManager, ShaderSource};
use crate::renderer::{Renderer, RendererError, init_render};
use crate::window::{ChronosWindow, WinError, WindowConfig};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(#[from] WinError),
    #[error("Renderer error: {0}")]
    RendererError(#[from] RendererError),
}

pub enum RendererType {
    OpenGL,
    Vulkan,
}

pub struct ChronosEngine {
    #[allow(dead_code)]
    window: ChronosWindow,
    renderer: Box<dyn Renderer>,
    shader_manager: ShaderManager,
}

impl ChronosEngine {
    /// Starts the Chronos engine with the given window configuration and renderer type.
    ///
    /// # Errors
    ///
    /// Returns an error if window creation or renderer initialization fails.
    pub fn start(window_config: WindowConfig, renderer_type: &RendererType) -> Result<Self> {
        let mut window = ChronosWindow::new(window_config);
        window.run()?;
        let renderer = init_render(&window, renderer_type)?;
        let engine = ChronosEngine {
            window,
            renderer,
            shader_manager: ShaderManager::default(),
        };
        Ok(engine)
    }

    /// Loads a shader into the engine.
    ///
    /// # Errors
    ///
    /// Returns an error if shader compilation fails or if the renderer encounters an error.
    pub fn load_shader(&mut self, name: &str, shader_source: &ShaderSource) -> Result<()> {
        self.shader_manager
            .register_from_source(name, shader_source);
        self.renderer.compile_shader(shader_source)?;
        // TODO: Store the compiled shader ID associated with the name.
        Ok(())
    }
}
