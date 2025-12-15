use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowId};

pub type Result<T> = std::result::Result<T, WinError>;

#[derive(thiserror::Error, Debug)]
pub enum WinError {
    #[error("Failed to initialize window: {0}")]
    InternalWinitError(#[from] EventLoopError),
    #[error("Window creation error: {0}")]
    WindowCreationError(#[from] winit::error::OsError),
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Eq)]
pub enum WindowMode {
    Normal,
    Test,
}

pub struct WindowConfig {
    pub resolution: Resolution,
    pub title: String,
    pub resizable: bool,
    pub window_mode: WindowMode,
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
            window_mode: WindowMode::Normal,
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
        if self.config.window_mode == WindowMode::Normal {
            self.run_normal_mode()
        } else if self.config.window_mode == WindowMode::Test {
            self.run_test_mode()
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn get_window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    #[must_use]
    pub fn get_inner_size(&self) -> Option<PhysicalSize<u32>> {
        self.window.as_ref().map(Window::inner_size)
    }

    fn run_normal_mode(&mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)?;
        Ok(())
    }

    #[allow(deprecated)]
    fn run_test_mode(&mut self) -> Result<()> {
        let event_loop = ChronosWindow::build_test_event_loop()?;
        let window_attrs = Window::default_attributes().with_visible(false);
        self.window = Some(event_loop.create_window(window_attrs)?);
        Ok(())
    }

    fn build_test_event_loop() -> Result<EventLoop<()>> {
        #[cfg(windows)]
        {
            let event_loop = winit::event_loop::EventLoop::builder()
                .with_any_thread(true)
                .build()?;
            Ok(event_loop)
        }

        #[cfg(not(windows))]
        {
            let event_loop = winit::event_loop::EventLoop::new()?;
            Ok(event_loop)
        }
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
mod tests {}
