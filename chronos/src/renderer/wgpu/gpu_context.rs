use std::sync::Arc;
use tracing::info;
use wgpu::MemoryHints;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::{
    configs,
    renderer::{RendererError, Result},
};

pub struct GpuContext {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture_format: wgpu::TextureFormat,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub resolution: configs::Resolution,
}

impl GpuContext {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let instance = Self::create_instance();
        let size = window.inner_size();
        let surface = Self::create_surface(&instance, window)?;
        let adapter = Self::create_adapter(&instance, Some(&surface)).await?;
        let (device, queue) = Self::create_device(&adapter).await?;

        let config = Self::create_surface_config(&surface, &adapter, size);
        surface.configure(&device, &config);
        let resolution = configs::Resolution {
            width: config.width,
            height: config.height,
        };

        Ok(Self {
            surface: Some(surface),
            device,
            queue,
            resolution,
            texture_format: config.format,
            config: Some(config),
        })
    }

    pub async fn new_headless(width: u32, height: u32) -> Result<Self> {
        let instance = Self::create_instance();
        let adapter = Self::create_adapter(&instance, None).await?;
        let (device, queue) = Self::create_device(&adapter).await?;
        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        Ok(Self {
            surface: None,
            device,
            queue,
            texture_format,
            config: None,
            resolution: configs::Resolution { width, height },
        })
    }

    fn create_instance() -> wgpu::Instance {
        wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..wgpu::InstanceDescriptor::new_without_display_handle()
            }
            .with_env(),
        )
    }

    fn create_surface(
        instance: &wgpu::Instance,
        window: Arc<Window>,
    ) -> Result<wgpu::Surface<'static>> {
        instance
            .create_surface(window)
            .map_err(|e| RendererError::Initialization(format!("Failed to create surface: {e}")))
    }

    fn create_surface_config(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        size: PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        }
    }

    async fn create_adapter(
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'static>>,
    ) -> Result<wgpu::Adapter> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: surface,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| RendererError::Initialization(format!("Failed to find adapter: {e}")))?;

        let adapter_info = adapter.get_info();
        info!(
            "Selected WGPU adapter: backend={:?}, name='{}', device_type={:?}, driver='{}', driver_info='{}'",
            adapter_info.backend,
            adapter_info.name,
            adapter_info.device_type,
            adapter_info.driver,
            adapter_info.driver_info,
        );

        Ok(adapter)
    }

    async fn create_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue)> {
        adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Chronos Device"),
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|e| {
                RendererError::Initialization(format!("Failed to create connection with GPU: {e}"))
            })
    }
}
