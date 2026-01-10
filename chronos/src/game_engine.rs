mod game_loop;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::components::color::RGBA;
use crate::configs::EngineConfig;
use crate::game_engine::game_loop::GameLoop;
use crate::renderer::shader_source::{ShaderManager, ShaderSource};
use crate::renderer::{Renderer, init_render};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(String),
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
}

#[derive(Clone)]
pub enum RendererType {
    Wgpu,
}

pub struct ChronosEngine {
    window: Option<Arc<winit::window::Window>>,
    renderer: Option<Box<dyn Renderer>>,
    shader_manager: ShaderManager,
    config: EngineConfig,
}

impl ChronosEngine {
    /// The main Chronos engine struct
    #[must_use]
    pub fn new(config: EngineConfig) -> Self {
        Self {
            window: None,
            renderer: None,
            shader_manager: ShaderManager::default(),
            config,
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

    /// Loads a shader into the engine.
    pub fn load_shader(&mut self, name: &str, shader_source: &ShaderSource) {
        self.shader_manager
            .register_from_source(name, shader_source);
    }

    pub fn set_background_color(&mut self, color: &RGBA) {
        if let Some(renderer) = &mut self.renderer {
            renderer.set_background_color(color);
        }
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
                GameLoop::main_frame(renderer.as_mut(), window);
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
}

impl ApplicationHandler for ChronosEngine {
    // run after event_loop.run_app is called
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(window) = self.create_window(event_loop) {
            match init_render(window.clone(), &self.config.renderer_type) {
                Ok(renderer) => self.renderer = Some(renderer),
                Err(e) => {
                    eprintln!("Failed to initialize renderer: {e}");
                    event_loop.exit();
                }
            }
            self.window = Some(window);
        } else {
            eprintln!("Failed to create window.");
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
