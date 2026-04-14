use tracing::error;
use winit::window::Window;

use crate::{
    renderer::{Renderer, RendererError},
    scene::Scene,
    texture_registry::TextureRegistry,
};

pub struct MainLoop {}

impl MainLoop {
    pub fn main_frame(
        renderer: &mut dyn Renderer,
        window: &Window,
        actual_scene: &Scene,
        texture_registry: &TextureRegistry,
    ) {
        MainLoop::run_render(renderer, window, actual_scene, texture_registry);
    }

    fn run_render(
        renderer: &mut dyn Renderer,
        window: &Window,
        actual_scene: &Scene,
        texture_registry: &TextureRegistry,
    ) {
        renderer.set_background_color(&actual_scene.background_color);
        match renderer.render(actual_scene, texture_registry) {
            Ok(()) => {}
            Err(RendererError::Surface(_)) => {
                let size = window.inner_size();
                renderer.resize(size.width, size.height);
            }
            Err(e) => {
                error!(err = %e, "Render error");
            }
        }
    }
}
