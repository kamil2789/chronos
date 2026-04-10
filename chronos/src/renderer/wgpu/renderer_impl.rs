use crate::components::color::RGBA;
use crate::renderer::{Renderer, wgpu::WgpuRenderer};
use crate::renderer::{RendererError, Result};

impl Renderer for WgpuRenderer {
    fn render(&mut self, scene: &crate::scene::Scene) -> Result<()> {
        self.render(scene).map_err(|e| match e {
            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                RendererError::Surface("Surface lost or outdated - resize required".to_string())
            }
            _ => RendererError::Render(format!("Failed to render frame: {e}")),
        })
    }

    fn render_to_buffer(&mut self, scene: &crate::scene::Scene) -> Result<Vec<u8>> {
        self.set_background_color(&scene.background_color);
        WgpuRenderer::render_to_buffer(self, scene)
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu_context.width = width;
            self.gpu_context.height = height;
            if let (Some(surface), Some(config)) =
                (&self.gpu_context.surface, &mut self.gpu_context.config)
            {
                config.width = width;
                config.height = height;
                surface.configure(&self.gpu_context.device, config);
            }
        }
    }

    fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}
