use std::sync::Arc;
use wgpu::MemoryHints;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::renderer::Renderer;
use crate::renderer::{RendererError, Result};

pub struct WgpuRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let gpu_instance = Self::create_gpu_instance();
        let size = window.inner_size();
        let surface = Self::create_surface(&gpu_instance, window)?;
        let adapter = Self::create_adapter(&gpu_instance, &surface).await?;
        let (device, queue) = Self::create_connection_with_gpu(&adapter).await?;

        let config = Self::create_surface_config(&surface, &adapter, size);

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }

    //Need refactor
    pub fn render(&mut self) -> std::result::Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn create_gpu_instance() -> wgpu::Instance {
        wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    fn create_surface(
        gpu_instance: &wgpu::Instance,
        window: Arc<Window>,
    ) -> Result<wgpu::Surface<'static>> {
        gpu_instance
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
        gpu_instance: &wgpu::Instance,
        surface: &wgpu::Surface<'static>,
    ) -> Result<wgpu::Adapter> {
        gpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| RendererError::Initialization(format!("Failed to find adapter: {e}")))
    }

    async fn create_connection_with_gpu(
        adapter: &wgpu::Adapter,
    ) -> Result<(wgpu::Device, wgpu::Queue)> {
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
                RendererError::Initialization(format!("Failed to create connection with GPU: {e}",))
            })
    }
}

impl Renderer for WgpuRenderer {
    //fn compile_shader(&mut self, _source: &shader_source::ShaderSource) -> Result<ShaderId> {
    //    unimplemented!("Wgpu shader compilation is not implemented yet");
    //}

    fn render(&mut self) -> Result<()> {
        self.render().map_err(|e| match e {
            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                RendererError::Surface("Surface lost or outdated - resize required".to_string())
            }
            _ => RendererError::Render(format!("Failed to render frame: {e}")),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
