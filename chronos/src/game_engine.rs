use crate::window::{ChronosWindow, WinError, WindowConfig};

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Window error: {0}")]
    WindowError(#[from] WinError),
}

pub struct ChronosEngine {
    window: ChronosWindow,
}

impl ChronosEngine {
    #[must_use]
    pub fn new(window_config: WindowConfig) -> Self {
        Self {
            window: ChronosWindow::new(window_config),
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
}

impl Default for ChronosEngine {
    fn default() -> Self {
        Self {
            window: ChronosWindow::new(WindowConfig::default()),
        }
    }
}
