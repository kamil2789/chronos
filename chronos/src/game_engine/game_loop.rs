use winit::window::Window;

use crate::{
    renderer::{Renderer, RendererError},
    scene::Scene,
};

pub struct GameLoop {}

impl GameLoop {
    pub fn main_frame(renderer: &mut dyn Renderer, window: &Window, actual_scene: &Scene) {
        GameLoop::run_render(renderer, window, actual_scene);
    }

    fn run_render(renderer: &mut dyn Renderer, window: &Window, actual_scene: &Scene) {
        renderer.set_background_color(&actual_scene.background_color);
        match renderer.render() {
            Ok(()) => {}
            Err(RendererError::Surface(_)) => {
                let size = window.inner_size();
                renderer.resize(size.width, size.height);
            }
            Err(e) => {
                eprintln!("Render error: {e}");
            }
        }
    }
}
