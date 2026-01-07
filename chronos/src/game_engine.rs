use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::renderer::shader_source::{ShaderManager, ShaderSource};
use crate::renderer::wgpu::WgpuRenderer;
use crate::window::{ChronosWindow, WinError, WindowConfig};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(#[from] WinError),
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
}

pub struct ChronosEngine {
    window: ChronosWindow,
    renderer: Option<WgpuRenderer>,
    shader_manager: ShaderManager,
    frame_count: u64,
}

impl ChronosEngine {
    /// Creates a new instance of the `ChronosEngine` with the specified window configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub fn new(window_config: WindowConfig) -> Result<Self> {
        let window = ChronosWindow::new(window_config);
        Ok(Self {
            window,
            renderer: None,
            shader_manager: ShaderManager::default(),
            frame_count: 0,
        })
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
    ///
    /// # Errors
    ///
    /// Returns an error if shader compilation fails or if the renderer encounters an error.
    pub fn load_shader(&mut self, name: &str, shader_source: &ShaderSource) -> Result<()> {
        self.shader_manager
            .register_from_source(name, shader_source);
        // TODO: Implement shader compilation when renderer is ready
        Ok(())
    }
}

impl ApplicationHandler for ChronosEngine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Tworzymy okno
        let window_attributes = Window::default_attributes()
            .with_title(&self.window.config.title)
            .with_inner_size(LogicalSize::new(
                self.window.config.resolution.width,
                self.window.config.resolution.height,
            ))
            .with_resizable(self.window.config.resizable);

        let window = match event_loop.create_window(window_attributes) {
            Ok(w) => Arc::new(w),
            Err(_) => {
                event_loop.exit();
                return;
            }
        };

        self.window.window = Some(window.clone());

        // Inicjalizujemy renderer (blokujemy na async)
        match pollster::block_on(WgpuRenderer::new(window)) {
            Ok(renderer) => {
                self.renderer = Some(renderer);
            }
            Err(_) => {
                event_loop.exit();
            }
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
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                self.frame_count += 1;
                
                // Wyświetl co 60 klatek
                if self.frame_count % 60 == 0 {
                    println!("Frame: {}", self.frame_count);
                }
                
                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            // Rekonfiguruj surface jeśli stracony
                            if let Some(window) = &self.window.window {
                                let size = window.inner_size();
                                renderer.resize(size.width, size.height);
                            }
                        }
                        Err(e) => {
                            eprintln!("Render error: {:?}", e);
                        }
                    }
                }

                // Żądaj kolejnej klatki
                if let Some(window) = &self.window.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
