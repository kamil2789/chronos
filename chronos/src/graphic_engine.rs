mod game_loop;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use tracing::{error, info, warn};

use crate::configs::EngineConfig;
use crate::graphic_engine::game_loop::GameLoop;
use crate::renderer::{Renderer, RendererError, init_headless_render, init_render};
use crate::scene::{Scene, SceneManager};
use crate::texture_registry::{TextureData, TextureDataError, TextureRegistry};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(String),
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
    #[error("Renderer error: {0}")]
    RendererError(#[from] RendererError),
    #[error("Scene not found: {0}")]
    SceneNotFound(String),
    #[error("Invalid texture data: {0}")]
    InvalidTextureData(#[from] TextureDataError),
}

#[derive(Clone)]
pub enum RendererType {
    Wgpu,
}

pub struct ChronosEngine {
    window: Option<Arc<winit::window::Window>>,
    renderer: Option<Box<dyn Renderer>>,
    config: EngineConfig,
    scene_manager: SceneManager,
    texture_registry: TextureRegistry,
}

impl ChronosEngine {
    /// The main Chronos engine struct
    #[must_use]
    pub fn new(config: EngineConfig) -> Self {
        Self {
            window: None,
            renderer: None,
            config,
            scene_manager: SceneManager::default(),
            texture_registry: TextureRegistry::default(),
        }
    }

    /// Starts the Chronos engine
    ///
    /// # Errors
    ///
    /// Returns an error if window cannot run
    pub fn start(&mut self) -> Result<()> {
        if self.config.headless {
            self.init_headless_start()?;
            return Ok(());
        }

        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;
        Ok(())
    }

    pub fn register_scene(&mut self, scene: Scene) {
        self.scene_manager.register_scene(scene);
    }

    pub fn unregister_scene(&mut self, name: &str) {
        self.scene_manager.unregister_scene(name);
    }

    /// # Errors
    ///
    /// Returns an error if the scene does not exist
    pub fn set_current_scene(&mut self, name: &str) -> Result<()> {
        self.scene_manager.set_current_scene(name)
    }

    pub fn get_scenes(&self) -> impl Iterator<Item = &String> {
        self.scene_manager.get_scenes()
    }

    pub fn register_texture(&mut self, id: &str, data: TextureData) {
        self.texture_registry.register(id, data);
    }

    #[must_use]
    pub fn texture_registry(&self) -> &TextureRegistry {
        &self.texture_registry
    }

    /// # Errors
    ///
    /// Returns an error if rendering fails
    pub fn run_one_frame(&mut self) -> Result<Vec<u8>> {
        if !self.config.headless {
            warn!("run_one_frame is intended only for headless mode");
            return Ok(vec![]);
        }

        if let Some(renderer) = &mut self.renderer
            && let Some(current_scene) = &self.scene_manager.get_active_scene()
        {
            return renderer
                .render_to_buffer(current_scene, &self.texture_registry)
                .map_err(EngineError::RendererError);
        }
        Ok(vec![])
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<Arc<Window>> {
        let window_attributes = Window::default_attributes()
            .with_title(&self.config.window.title)
            .with_inner_size(LogicalSize::new(
                self.config.window.resolution.width,
                self.config.window.resolution.height,
            ))
            .with_resizable(self.config.window.resizable);

        let window = match event_loop.create_window(window_attributes) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                return Err(EngineError::WindowError(format!(
                    "Window creation error: {e}"
                )));
            }
        };

        Ok(window)
    }

    fn on_redraw_requested(&mut self, event_loop: &ActiveEventLoop) {
        let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) else {
            error!("Renderer or Window not initialized — shutting down");
            event_loop.exit();
            return;
        };

        if let Some(scene) = self.scene_manager.get_active_scene() {
            GameLoop::main_frame(renderer.as_mut(), window, scene, &self.texture_registry);
        }

        window.request_redraw();
    }

    fn on_close_requested(event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }

    fn on_resize_requested(&mut self, width: u32, height: u32) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height);
        }
    }

    fn init_start(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window = self.create_window(event_loop)?;
        let renderer = init_render(window.clone(), &self.config.renderer_type)?;

        self.window = Some(window);
        self.renderer = Some(renderer);
        Ok(())
    }

    fn init_headless_start(&mut self) -> Result<()> {
        let renderer = init_headless_render(
            self.config.window.resolution.width,
            self.config.window.resolution.height,
            &self.config.renderer_type,
        )?;
        self.renderer = Some(renderer);
        Ok(())
    }
}

impl ApplicationHandler for ChronosEngine {
    // run after event_loop.run_app is called
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init_start(event_loop) {
            error!(err = %e, "Failed to initialize engine");
            event_loop.exit();
        } else {
            info!("Engine initialized successfully");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                Self::on_close_requested(event_loop);
            }
            WindowEvent::Resized(new_size) => {
                self.on_resize_requested(new_size.width, new_size.height);
            }
            WindowEvent::RedrawRequested => {
                self.on_redraw_requested(event_loop);
            }
            _ => {}
        }
    }
}

pub struct HeadlessRenderer {
    renderer: Box<dyn Renderer>,
    texture_registry: TextureRegistry,
}

impl HeadlessRenderer {
    /// Creates a headless renderer that renders to an offscreen buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if GPU initialization fails.
    pub fn new(width: u32, height: u32, renderer_type: &RendererType) -> Result<Self> {
        let renderer = init_headless_render(width, height, renderer_type)?;
        Ok(Self {
            renderer,
            texture_registry: TextureRegistry::default(),
        })
    }

    pub fn register_texture(&mut self, id: &str, data: TextureData) {
        self.texture_registry.register(id, data);
    }

    /// Renders a single frame of the scene and returns raw RGBA pixel data.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    pub fn render_to_buffer(&mut self, scene: &Scene) -> Result<Vec<u8>> {
        self.renderer
            .render_to_buffer(scene, &self.texture_registry)
            .map_err(EngineError::RendererError)
    }
}
