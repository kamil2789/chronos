mod gpu_context;
mod render_loop;
mod renderer_impl;
mod shaders;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::components::color::{Color, RGBA};
use crate::components::shape::Shape;
use crate::entity;
use crate::renderer::wgpu::gpu_context::GpuContext;
use crate::renderer::{RendererError, Result};
use crate::scene::Scene;

/// Cache for entity rendering resources to avoid recreating buffers every frame
#[allow(dead_code)]
struct EntityRenderCache {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    // For uniform color pipeline
    // color_buffer must stay alive for bind_group to work
    color_buffer: Option<wgpu::Buffer>,
    color_bind_group: Option<wgpu::BindGroup>,
}

pub struct WgpuRenderer {
    gpu_context: GpuContext,
    shader_manager: shaders::ShaderManager,
    background_color: RGBA,
    // Pipeline for rendering with uniform color
    uniform_color_pipeline: Option<wgpu::RenderPipeline>,
    color_bind_group_layout: Option<wgpu::BindGroupLayout>,
    // Pipeline for rendering with per-vertex colors
    vertex_color_pipeline: Option<wgpu::RenderPipeline>,
    // Cache for entity buffers to avoid recreating them every frame
    // Using RefCell for interior mutability to allow caching during rendering
    entity_cache: RefCell<HashMap<usize, EntityRenderCache>>,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let gpu_context = GpuContext::new(window).await?;
        let shader_manager = shaders::ShaderManager::new(&gpu_context.device);

        Ok(Self {
            gpu_context,
            shader_manager,
            background_color: RGBA::default(),
            uniform_color_pipeline: None,
            color_bind_group_layout: None,
            vertex_color_pipeline: None,
            entity_cache: RefCell::new(HashMap::new()),
        })
    }

    pub fn render(&mut self, scene: &Scene) -> std::result::Result<(), wgpu::SurfaceError> {
        let current_frame = self.gpu_context.surface.get_current_texture()?;
        let frame_view = current_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut frame_commands =
            self.gpu_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: &frame_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear((&self.background_color).into()),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            };
            let mut render_pass = frame_commands.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                ..Default::default()
            });

            let entities = entity::query_entities!(scene.entity_manager, Shape, Color);

            for entity_id in entities {
                if let (Some(shape), Some(color)) = (
                    scene.entity_manager.get_component::<Shape>(entity_id),
                    scene
                        .entity_manager
                        .get_component::<crate::components::color::Color>(entity_id),
                ) {
                    if color.is_uniform() {
                        // Render with uniform color
                        if let Some(pipeline) = &self.uniform_color_pipeline {
                            self.render_entity_with_uniform_color(
                                &mut render_pass,
                                pipeline,
                                entity_id,
                                shape,
                                color,
                            );
                        }
                    } else {
                        // Render with per-vertex colors
                        if let Some(pipeline) = &self.vertex_color_pipeline {
                            self.render_entity_with_vertex_color(
                                &mut render_pass,
                                pipeline,
                                entity_id,
                                shape,
                                color,
                            );
                        }
                    }
                }
            }
        }

        self.gpu_context
            .queue
            .submit(std::iter::once(frame_commands.finish()));
        current_frame.present();

        Ok(())
    }

    // Pipeline for rendering with uniform color
    fn create_uniform_color_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_manager: &shaders::ShaderManager,
    ) -> Result<(wgpu::RenderPipeline, wgpu::BindGroupLayout)> {
        let shader = shader_manager.get_shader("uniform_color").ok_or_else(|| {
            RendererError::Initialization("Shader 'uniform_color' not found".to_string())
        })?;

        // Create bind group layout for color uniform buffer
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
            bind_group_layouts: &[&color_bind_group_layout],
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

        Ok((pipeline, color_bind_group_layout))
    }

    // Pipeline for rendering with per-vertex colors
    fn create_vertex_color_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_manager: &shaders::ShaderManager,
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

        Ok(pipeline)
    }

    fn render_entity_with_vertex_color(
        &self,
        render_pass: &mut wgpu::RenderPass,
        pipeline: &wgpu::RenderPipeline,
        entity_id: usize,
        shape: &crate::components::shape::Shape,
        color: &crate::components::color::Color,
    ) {
        // Get per-vertex colors
        if let Some(vertex_colors) = color.get_vertex_colors() {
            let vertices = shape.get_vertices();

            // Validation: check if color count matches vertex count (4 floats per vertex: r,g,b,a)
            if vertex_colors.len() != vertices.len() * 4 {
                eprintln!(
                    "Warning: vertex color count mismatch. Expected {} floats, got {}",
                    vertices.len() * 4,
                    vertex_colors.len()
                );
                return;
            }

            // Try to get cached buffer, or create new one
            {
                let mut cache_map = self.entity_cache.borrow_mut();
                cache_map.entry(entity_id).or_insert_with(|| {
                    // Build interleaved vertex buffer: [pos.x, pos.y, pos.z, color.r, color.g, color.b, color.a, ...]
                    let mut vertex_data: Vec<f32> = Vec::with_capacity(vertices.len() * 7);
                    for (i, vertex) in vertices.iter().enumerate() {
                        vertex_data.push(vertex.x);
                        vertex_data.push(vertex.y);
                        vertex_data.push(vertex.z);
                        vertex_data.push(vertex_colors[i * 4]);
                        vertex_data.push(vertex_colors[i * 4 + 1]);
                        vertex_data.push(vertex_colors[i * 4 + 2]);
                        vertex_data.push(vertex_colors[i * 4 + 3]);
                    }

                    // Create vertex buffer
                    let vertex_buffer = self.gpu_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Color Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertex_data),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    );

                    EntityRenderCache {
                        vertex_buffer,
                        vertex_count: u32::try_from(vertices.len()).unwrap_or(0),
                        color_buffer: None,
                        color_bind_group: None,
                    }
                });
            }

            // Now borrow immutably to use the cache
            let cache_borrow = self.entity_cache.borrow();
            let cache = cache_borrow.get(&entity_id).unwrap();

            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, cache.vertex_buffer.slice(..));
            render_pass.draw(0..cache.vertex_count, 0..1);
        }
    }

    fn render_entity_with_uniform_color(
        &self,
        render_pass: &mut wgpu::RenderPass,
        pipeline: &wgpu::RenderPipeline,
        entity_id: usize,
        shape: &crate::components::shape::Shape,
        color: &crate::components::color::Color,
    ) {
        // Get color from component and prepare uniform buffer
        if let (Some(rgba), Some(layout)) =
            (color.get_uniform_color(), &self.color_bind_group_layout)
        {
            // Try to get cached resources, or create new ones
            {
                let mut cache_map = self.entity_cache.borrow_mut();
                cache_map.entry(entity_id).or_insert_with(|| {
                    // Konwertuj Vec3 do [f32; 3] (x, y, z)
                    let vertices: Vec<[f32; 3]> = shape
                        .get_vertices()
                        .iter()
                        .map(|v| [v.x, v.y, v.z])
                        .collect();

                    // Create vertex buffer for this specific shape
                    let vertex_buffer = self.gpu_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Entity Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    );

                    let vertex_count = u32::try_from(vertices.len()).unwrap_or(0);

                    // Get color as f32 array directly (no f64->f32 casting)
                    let color_data: [f32; 4] = rgba.to_normalized_f32_array();

                    // Create uniform buffer for color
                    let color_buffer = self.gpu_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Color Uniform Buffer"),
                            contents: bytemuck::cast_slice(&color_data),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        },
                    );

                    // Create bind group
                    let color_bind_group =
                        self.gpu_context
                            .device
                            .create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some("Color Bind Group"),
                                layout,
                                entries: &[wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: color_buffer.as_entire_binding(),
                                }],
                            });

                    EntityRenderCache {
                        vertex_buffer,
                        vertex_count,
                        color_buffer: Some(color_buffer),
                        color_bind_group: Some(color_bind_group),
                    }
                });
            }

            // Now borrow immutably to use the cache
            let cache_borrow = self.entity_cache.borrow();
            let cache = cache_borrow.get(&entity_id).unwrap();

            render_pass.set_pipeline(pipeline);
            if let Some(bind_group) = &cache.color_bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
            }
            render_pass.set_vertex_buffer(0, cache.vertex_buffer.slice(..));
            render_pass.draw(0..cache.vertex_count, 0..1);
        }
    }
}
