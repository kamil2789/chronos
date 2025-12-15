use std::sync::OnceLock;

use crate::{
    renderer::opengl::{OpenGL, init_opengl},
    window::{ChronosWindow, WindowConfig, WindowMode},
};

static WIN_INSTANCE: OnceLock<ChronosWindow> = OnceLock::new();
static OPENGL_API_INSTANCE: OnceLock<OpenGL> = OnceLock::new();

fn get_window() -> &'static ChronosWindow {
    WIN_INSTANCE.get_or_init(|| {
        let mut window = ChronosWindow::new(WindowConfig {
            window_mode: WindowMode::Test,
            ..Default::default()
        });

        window.run().unwrap();
        window
    })
}

pub fn get_opengl_api() -> &'static OpenGL {
    OPENGL_API_INSTANCE.get_or_init(|| {
        let window = get_window();
        let opengl = init_opengl(window).unwrap();
        opengl
    })
}
