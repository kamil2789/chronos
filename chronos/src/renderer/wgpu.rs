use std::sync::Arc;
use winit::window::Window;

use crate::renderer::{Renderer, shader_source};
use crate::renderer::{Result, ShaderId, RendererError};

pub struct WgpuRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        // Instance - handle to GPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Surface - miejsce gdzie renderujemy (okno)
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| RendererError::Initialization(format!("Failed to create surface: {}", e)))?;

        // Adapter - fizyczna karta graficzna
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| RendererError::Initialization(format!("Failed to find adapter: {}", e)))?;

        // Device + Queue - połączenie z GPU
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Chronos Device"),
                    required_features: wgpu::Features::empty(),
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                },
            )
            .await
            .map_err(|e| RendererError::Initialization(format!("Failed to create device: {}", e)))?;

        let surface_caps = surface.get_capabilities(&adapter);

        // Format powierzchni - preferujemy sRGB
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> std::result::Result<(), wgpu::SurfaceError> {
        if !self.is_surface_configured {
            return Ok(());
        }

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
}

impl Renderer for WgpuRenderer {
    fn compile_shader(&mut self, _source: &shader_source::ShaderSource) -> Result<ShaderId> {
        unimplemented!("Wgpu shader compilation is not implemented yet");
    }
}