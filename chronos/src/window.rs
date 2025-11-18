use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub type Result<T> = std::result::Result<T, WinError>;

#[derive(thiserror::Error, Debug)]
pub enum WinError {
    #[error("Failed to initialize window: {0}")]
    InternalWinitError(#[from] EventLoopError),
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct WindowConfig {
    pub resolution: Resolution,
    pub title: String,
    pub resizable: bool,
}

pub struct ChronosWindow {
    window: Option<Window>,
    config: WindowConfig,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution {
                width: 1280,
                height: 720,
            },
            title: "Chronos Engine".to_string(),
            resizable: true,
        }
    }
}

impl ChronosWindow {
    #[must_use]
    pub fn new(config: WindowConfig) -> Self {
        Self {
            window: None,
            config,
        }
    }

    /// Runs the window event loop.
    ///
    /// # Errors
    ///
    /// Returns an error if the event loop fails to initialize or run.
    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)?;
        Ok(())
    }
}

impl ApplicationHandler for ChronosWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(&self.config.title)
            .with_inner_size(LogicalSize::new(
                self.config.resolution.width,
                self.config.resolution.height,
            ))
            .with_resizable(self.config.resizable);

        if let Ok(window) = event_loop.create_window(window_attributes) {
            self.window = Some(window);
        } else {
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                println!("Window resized to: {}x{}", new_size.width, new_size.height);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    
}