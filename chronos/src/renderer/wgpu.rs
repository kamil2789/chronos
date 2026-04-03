mod frame;
mod gpu_context;
mod pipeline_manager;
mod renderer_impl;
mod shaders;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use winit::window::Window;

use crate::components::color::RGBA;
use crate::renderer::Result;
use crate::renderer::wgpu::frame::EntityRenderCache;
use crate::renderer::wgpu::gpu_context::GpuContext;
use crate::renderer::wgpu::pipeline_manager::PipelineManager;

pub struct WgpuRenderer {
    gpu_context: GpuContext,
    pipeline_manager: PipelineManager,
    background_color: RGBA,
    // RefCell for interior mutability to allow caching during immutable render pass
    entity_cache: RefCell<HashMap<usize, EntityRenderCache>>,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let gpu_context = GpuContext::new(window).await?;
        let shader_manager = shaders::ShaderManager::new(&gpu_context.device);
        let mut pipeline_manager = PipelineManager::new();
        pipeline_manager.build(&gpu_context.device, &gpu_context.config, &shader_manager)?;

        Ok(Self {
            gpu_context,
            pipeline_manager,
            background_color: RGBA::default(),
            entity_cache: RefCell::new(HashMap::new()),
        })
    }
}
