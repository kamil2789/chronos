use crate::components::color::RGBA;
use crate::renderer::{Renderer, wgpu::WgpuRenderer};
use crate::renderer::{RendererError, Result};

impl Renderer for WgpuRenderer {
    fn build_pipelines(&mut self) -> Result<()> {
        // Create pipeline for uniform color
        let (uniform_color_pipeline, color_bind_group_layout) =
            Self::create_uniform_color_pipeline(
                &self.gpu_context.device,
                &self.gpu_context.config,
                &self.shader_manager,
            )?;

        // Create pipeline for vertex color
        let vertex_color_pipeline = Self::create_vertex_color_pipeline(
            &self.gpu_context.device,
            &self.gpu_context.config,
            &self.shader_manager,
        )?;

        // Save pipelines and bind group layout
        self.uniform_color_pipeline = Some(uniform_color_pipeline);
        self.color_bind_group_layout = Some(color_bind_group_layout);
        self.vertex_color_pipeline = Some(vertex_color_pipeline);

        Ok(())
    }

    fn render(&mut self, scene: &crate::scene::Scene) -> Result<()> {
        self.render(scene).map_err(|e| match e {
            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                RendererError::Surface("Surface lost or outdated - resize required".to_string())
            }
            _ => RendererError::Render(format!("Failed to render frame: {e}")),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu_context.config.width = width;
            self.gpu_context.config.height = height;
            self.gpu_context
                .surface
                .configure(&self.gpu_context.device, &self.gpu_context.config);
        }
    }

    fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}
