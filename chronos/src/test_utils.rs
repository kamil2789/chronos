use std::sync::OnceLock;

use crate::window::{ChronosWindow, WindowConfig, WindowMode};

static _WIN_INSTANCE: OnceLock<ChronosWindow> = OnceLock::new();

fn _get_window() -> &'static ChronosWindow {
    _WIN_INSTANCE.get_or_init(|| {
        let mut window = ChronosWindow::new(WindowConfig {
            window_mode: WindowMode::Test,
            ..Default::default()
        });

        window.run().unwrap();
        window
    })
}
