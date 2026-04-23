use crate::components::color::RGBA;
use crate::configs::Resolution;
use crate::renderer::Result;
use crate::renderer::{Renderer, wgpu::WgpuRenderer};
use crate::texture_registry::TextureRegistry;

impl Renderer for WgpuRenderer {
    fn render(
        &mut self,
        scene: &crate::scene::Scene,
        texture_registry: &TextureRegistry,
    ) -> Result<()> {
        self.render(scene, texture_registry)
    }

    fn render_to_buffer(
        &mut self,
        scene: &crate::scene::Scene,
        texture_registry: &TextureRegistry,
    ) -> Result<Vec<u8>> {
        self.set_background_color(&scene.background_color);
        WgpuRenderer::render_to_buffer(self, scene, texture_registry)
    }

    fn resize(&mut self, resolution: Resolution) {
        if resolution.width > 0 && resolution.height > 0 {
            self.gpu_context.resolution = resolution;
            if let (Some(surface), Some(config)) =
                (&self.gpu_context.surface, &mut self.gpu_context.config)
            {
                config.width = self.gpu_context.resolution.width;
                config.height = self.gpu_context.resolution.height;
                surface.configure(&self.gpu_context.device, config);
            }
        }
    }

    fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}
