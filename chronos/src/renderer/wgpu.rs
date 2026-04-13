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

pub struct GpuTextureResource {
    #[allow(dead_code)]
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

pub struct WgpuRenderer {
    gpu_context: GpuContext,
    pipeline_manager: PipelineManager,
    background_color: RGBA,
    entity_cache: RefCell<HashMap<usize, EntityRenderCache>>,
    texture_gpu_cache: RefCell<HashMap<String, GpuTextureResource>>,
    last_scene_name: RefCell<Option<String>>,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let gpu_context = GpuContext::new(window).await?;
        let shader_manager = shaders::ShaderManager::new(&gpu_context.device).await?;
        let mut pipeline_manager = PipelineManager::new();
        pipeline_manager.build(
            &gpu_context.device,
            gpu_context.texture_format,
            &shader_manager,
        )?;

        Ok(Self {
            gpu_context,
            pipeline_manager,
            background_color: RGBA::default(),
            entity_cache: RefCell::new(HashMap::new()),
            texture_gpu_cache: RefCell::new(HashMap::new()),
            last_scene_name: RefCell::new(None),
        })
    }

    pub async fn new_headless(width: u32, height: u32) -> Result<Self> {
        let gpu_context = GpuContext::new_headless(width, height).await?;
        let shader_manager = shaders::ShaderManager::new(&gpu_context.device).await?;
        let mut pipeline_manager = PipelineManager::new();
        pipeline_manager.build(
            &gpu_context.device,
            gpu_context.texture_format,
            &shader_manager,
        )?;

        Ok(Self {
            gpu_context,
            pipeline_manager,
            background_color: RGBA::default(),
            entity_cache: RefCell::new(HashMap::new()),
            texture_gpu_cache: RefCell::new(HashMap::new()),
            last_scene_name: RefCell::new(None),
        })
    }
}
