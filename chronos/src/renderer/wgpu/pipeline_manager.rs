use crate::renderer::{RendererError, Result};

use super::shaders::ShaderManager;

pub struct PipelineManager {
    pub uniform_color_pipeline: Option<wgpu::RenderPipeline>,
    pub color_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub vertex_color_pipeline: Option<wgpu::RenderPipeline>,
}

impl PipelineManager {
    pub fn new() -> Self {
        Self {
            uniform_color_pipeline: None,
            color_bind_group_layout: None,
            vertex_color_pipeline: None,
        }
    }

    pub fn build(
        &mut self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        shader_manager: &ShaderManager,
    ) -> Result<()> {
        let (uniform_color_pipeline, color_bind_group_layout) =
            Self::create_uniform_color_pipeline(device, texture_format, shader_manager)?;

        let vertex_color_pipeline =
            Self::create_vertex_color_pipeline(device, texture_format, shader_manager)?;

        self.uniform_color_pipeline = Some(uniform_color_pipeline);
        self.color_bind_group_layout = Some(color_bind_group_layout);
        self.vertex_color_pipeline = Some(vertex_color_pipeline);

        Ok(())
    }

    fn create_uniform_color_pipeline(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        shader_manager: &ShaderManager,
    ) -> Result<(wgpu::RenderPipeline, wgpu::BindGroupLayout)> {
        let shader = shader_manager.get_shader("uniform_color").ok_or_else(|| {
            RendererError::Initialization("Shader 'uniform_color' not found".to_string())
        })?;

        let color_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Color Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Uniform Color Pipeline Layout"),
            bind_group_layouts: &[Some(&color_bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Uniform Color Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    }],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
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

        Ok((pipeline, color_bind_group_layout))
    }

    fn create_vertex_color_pipeline(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        shader_manager: &ShaderManager,
    ) -> Result<wgpu::RenderPipeline> {
        let shader = shader_manager.get_shader("vertex_color").ok_or_else(|| {
            RendererError::Initialization("Shader 'vertex_color' not found".to_string())
        })?;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vertex Color Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Vertex Color Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: (std::mem::size_of::<[f32; 3]>()
                        + std::mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
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

        Ok(pipeline)
    }
}
