use std::sync::Arc;

use winit::window::Window;

use crate::renderer::{Renderer, RendererError};

pub struct GameLoop {}

impl GameLoop {
    pub fn new() -> Self {
        Self {}
    }

    pub fn main_frame(renderer: &mut dyn Renderer, window: &Window) {
        GameLoop::run_render(renderer, window);
    }

    fn run_render(renderer: &mut dyn Renderer, window: &Window) {
        match renderer.render() {
            Ok(_) => {}
            Err(RendererError::Surface(_)) => {
                let size = window.inner_size();
                renderer.resize(size.width, size.height);
            }
            Err(e) => {
                eprintln!("Render error: {:?}", e);
            }
        }
    }
}
