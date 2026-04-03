mod game_loop;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::configs::EngineConfig;
use crate::graphic_engine::game_loop::GameLoop;
use crate::renderer::{Renderer, RendererError, init_render};
use crate::scene::Scene;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(String),
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
    #[error("Renderer initialization error: {0}")]
    RendererInitialization(#[from] RendererError),
}

#[derive(Clone)]
pub enum RendererType {
    Wgpu,
}

pub struct ChronosEngine {
    window: Option<Arc<winit::window::Window>>,
    renderer: Option<Box<dyn Renderer>>,
    config: EngineConfig,
    scene: Scene,
}

impl ChronosEngine {
    /// The main Chronos engine struct
    #[must_use]
    pub fn new(config: EngineConfig) -> Self {
        Self {
            window: None,
            renderer: None,
            config,
            scene: Scene::default(),
        }
    }

    /// Starts the Chronos engine
    ///
    /// # Errors
    ///
    /// Returns an error if window cannot run
    pub fn start(&mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)?;
        Ok(())
    }

    pub fn register_scene(&mut self, scene: Scene) {
        self.scene = scene;
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

    fn on_redraw_requested(&mut self) {
        if let Some(window) = &self.window {
            if let Some(renderer) = &mut self.renderer {
                GameLoop::main_frame(renderer.as_mut(), window, &self.scene);
            }
            window.request_redraw();
        }
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
}

impl ApplicationHandler for ChronosEngine {
    // run after event_loop.run_app is called
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init_start(event_loop) {
            eprintln!("Failed to initialize engine: {e}");
            event_loop.exit();
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
                self.on_redraw_requested();
            }
            _ => {}
        }
    }
}
