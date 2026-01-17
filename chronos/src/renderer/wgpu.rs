mod shaders;

use std::sync::Arc;
use wgpu::MemoryHints;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::components::color::RGBA;
use crate::renderer::Renderer;
use crate::renderer::{RendererError, Result};

pub struct WgpuRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    shader_manager: shaders::ShaderManager,
    //fields to refactor
    background_color: RGBA,
    // Pipeline dla trójkąta z jednolitym kolorem
    uniform_color_pipeline: Option<wgpu::RenderPipeline>,
    uniform_color_vertex_buffer: Option<wgpu::Buffer>,
    // Pipeline dla trójkąta z kolorami per wierzchołek
    vertex_color_pipeline: Option<wgpu::RenderPipeline>,
    vertex_color_vertex_buffer: Option<wgpu::Buffer>,
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

        // Inicjalizacja ShaderManager (bez kompilacji - to zrobi compile_all_shaders())
        let shader_manager = shaders::ShaderManager::default();

        Ok(Self {
            surface,
            device,
            queue,
            config,
            shader_manager,
            background_color: RGBA::default(),
            uniform_color_pipeline: None,
            uniform_color_vertex_buffer: None,
            vertex_color_pipeline: None,
            vertex_color_vertex_buffer: None,
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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
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

            // Renderowanie pierwszego trójkąta (lewy - jednolity kolor)
            if let (Some(pipeline), Some(buffer)) = (
                &self.uniform_color_pipeline,
                &self.uniform_color_vertex_buffer,
            ) {
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..3, 0..1);
            }

            // Renderowanie drugiego trójkąta (prawy - kolory per wierzchołek)
            if let (Some(pipeline), Some(buffer)) = (
                &self.vertex_color_pipeline,
                &self.vertex_color_vertex_buffer,
            ) {
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..3, 0..1);
            }
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

    // Pipeline dla trójkąta z jednolitym kolorem (lewy trójkąt)
    fn create_uniform_color_triangle_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_manager: &shaders::ShaderManager,
    ) -> Result<(wgpu::RenderPipeline, wgpu::Buffer)> {
        let shader = shader_manager.get_shader("uniform_color").ok_or_else(|| {
            RendererError::Initialization("Shader 'uniform_color' not found".to_string())
        })?;

        // Wierzchołki dla lewego trójkąta (tylko pozycje)
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct Vertex {
            position: [f32; 2],
        }

        let vertices = [
            Vertex {
                position: [-0.8, -0.5],
            }, // lewy dolny
            Vertex {
                position: [-0.3, -0.5],
            }, // prawy dolny
            Vertex {
                position: [-0.55, 0.5],
            }, // górny
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Color Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Uniform Color Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Uniform Color Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    }],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Ok((pipeline, vertex_buffer))
    }

    // Pipeline dla trójkąta z kolorami per wierzchołek (prawy trójkąt)
    fn create_vertex_color_triangle_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_manager: &shaders::ShaderManager,
    ) -> Result<(wgpu::RenderPipeline, wgpu::Buffer)> {
        let shader = shader_manager.get_shader("vertex_color").ok_or_else(|| {
            RendererError::Initialization("Shader 'vertex_color' not found".to_string())
        })?;

        // Wierzchołki dla prawego trójkąta (pozycje + kolory)
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        let vertices = [
            Vertex {
                position: [0.3, -0.5],
                color: [1.0, 0.0, 0.0],
            }, // lewy dolny - czerwony
            Vertex {
                position: [0.8, -0.5],
                color: [0.0, 1.0, 0.0],
            }, // prawy dolny - zielony
            Vertex {
                position: [0.55, 0.5],
                color: [0.0, 0.0, 1.0],
            }, // górny - niebieski
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Color Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vertex Color Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Vertex Color Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Ok((pipeline, vertex_buffer))
    }
}

impl Renderer for WgpuRenderer {
    fn compile_all_shaders(&mut self) -> Result<()> {
        // Kompiluj shadery
        self.shader_manager.compile_all(&self.device).map_err(|e| {
            RendererError::Initialization(format!("Failed to compile shaders: {e}"))
        })?;

        // Twórz pipeline'y używając skompilowanych shaderów
        let (uniform_color_pipeline, uniform_color_vertex_buffer) =
            Self::create_uniform_color_triangle_pipeline(
                &self.device,
                &self.config,
                &self.shader_manager,
            )?;
        let (vertex_color_pipeline, vertex_color_vertex_buffer) =
            Self::create_vertex_color_triangle_pipeline(
                &self.device,
                &self.config,
                &self.shader_manager,
            )?;

        // Zapisz pipeline'y
        self.uniform_color_pipeline = Some(uniform_color_pipeline);
        self.uniform_color_vertex_buffer = Some(uniform_color_vertex_buffer);
        self.vertex_color_pipeline = Some(vertex_color_pipeline);
        self.vertex_color_vertex_buffer = Some(vertex_color_vertex_buffer);

        Ok(())
    }

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

    fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }
}
