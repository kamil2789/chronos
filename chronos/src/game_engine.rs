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
    Wgpu,
    Vulkan,
}

pub struct ChronosEngine {
    #[allow(dead_code)]
    window: ChronosWindow,
    renderer: Box<dyn Renderer>,
    shader_manager: ShaderManager,
}

impl ChronosEngine {
    /// Creates a new instance of the `ChronosEngine` with the specified window configuration and renderer type.
    ///
    /// # Errors
    ///
    /// Returns an error if the window cannot be created or if the renderer fails to initialize.
    pub fn new(window_config: WindowConfig, renderer_type: &RendererType) -> Result<Self> {
        let window = ChronosWindow::new(window_config);
        let renderer = init_render(&window, renderer_type)?;
        Ok(Self {
            window,
            renderer,
            shader_manager: ShaderManager::default(),
        })
    }
    /// Starts the Chronos engine
    ///
    /// # Errors
    ///
    /// Returns an error if window cannot run
    pub fn start(&mut self) -> Result<()> {
        self.window.run()?;
        Ok(())
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
