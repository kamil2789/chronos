use tracing::warn;
use wgpu::util::DeviceExt;

use crate::components::color::Color;
use crate::components::shape::Shape;
use crate::entity;
use crate::scene::Scene;

use super::WgpuRenderer;

/// Cache for entity rendering resources to avoid recreating buffers every frame
pub struct EntityRenderCache {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    // color_buffer must stay alive for bind_group to work
    #[allow(dead_code)]
    pub color_buffer: Option<wgpu::Buffer>,
    pub color_bind_group: Option<wgpu::BindGroup>,
}

impl WgpuRenderer {
    pub fn render(&mut self, scene: &Scene) -> std::result::Result<(), wgpu::SurfaceError> {
        let surface = self
            .gpu_context
            .surface
            .as_ref()
            .expect("render() requires a windowed context");
        let current_frame = surface.get_current_texture()?;
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

            self.draw_entities(&mut render_pass, scene);
        }

        self.gpu_context
            .queue
            .submit(std::iter::once(frame_commands.finish()));
        current_frame.present();

        Ok(())
    }

    pub fn render_to_buffer(&mut self, scene: &Scene) -> crate::renderer::Result<Vec<u8>> {
        let width = self.gpu_context.width;
        let height = self.gpu_context.height;
        let texture_format = self.gpu_context.texture_format;

        let texture = self
            .gpu_context
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Headless Render Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bytes_per_pixel = 4u32;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;
        let buffer_size = u64::from(padded_bytes_per_row * height);

        let output_buffer = self
            .gpu_context
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Headless Output Buffer"),
                size: buffer_size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let mut encoder =
            self.gpu_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Headless Render Encoder"),
                });

        {
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear((&self.background_color).into()),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            };
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Headless Render Pass"),
                color_attachments: &[Some(color_attachment)],
                ..Default::default()
            });

            self.draw_entities(&mut render_pass, scene);
        }

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let submission_index = self
            .gpu_context
            .queue
            .submit(std::iter::once(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });
        self.gpu_context
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: Some(submission_index),
                timeout: None,
            })
            .ok();
        receiver.recv().unwrap().map_err(|e| {
            crate::renderer::RendererError::Render(format!("Failed to map buffer: {e}"))
        })?;

        let data = buffer_slice.get_mapped_range();
        let mut pixels = Vec::with_capacity((width * height * bytes_per_pixel) as usize);
        for row in 0..height {
            let start = (row * padded_bytes_per_row) as usize;
            let end = start + unpadded_bytes_per_row as usize;
            pixels.extend_from_slice(&data[start..end]);
        }
        drop(data);
        output_buffer.unmap();

        Ok(pixels)
    }

    fn draw_entities(&self, render_pass: &mut wgpu::RenderPass, scene: &Scene) {
        let entities = entity::query_entities!(scene.entity_manager, Shape, Color);

        for entity_id in entities {
            if let (Some(shape), Some(color)) = (
                scene.entity_manager.get_component::<Shape>(entity_id),
                scene
                    .entity_manager
                    .get_component::<crate::components::color::Color>(entity_id),
            ) {
                if color.is_uniform() {
                    if let Some(pipeline) = &self.pipeline_manager.uniform_color_pipeline {
                        self.render_entity_with_uniform_color(
                            render_pass,
                            pipeline,
                            entity_id,
                            shape,
                            color,
                        );
                    }
                } else if let Some(pipeline) = &self.pipeline_manager.vertex_color_pipeline {
                    self.render_entity_with_vertex_color(
                        render_pass,
                        pipeline,
                        entity_id,
                        shape,
                        color,
                    );
                }
            }
        }
    }

    fn render_entity_with_vertex_color(
        &self,
        render_pass: &mut wgpu::RenderPass,
        pipeline: &wgpu::RenderPipeline,
        entity_id: usize,
        shape: &Shape,
        color: &Color,
    ) {
        if let Some(vertex_colors) = color.get_vertex_colors() {
            let vertices = shape.get_vertices();

            if vertex_colors.len() != vertices.len() * 4 {
                warn!(
                    expected = vertices.len() * 4,
                    got = vertex_colors.len(),
                    "Vertex color count mismatch"
                );
                return;
            }

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
        shape: &Shape,
        color: &Color,
    ) {
        if let (Some(rgba), Some(layout)) = (
            color.get_uniform_color(),
            &self.pipeline_manager.color_bind_group_layout,
        ) {
            {
                let mut cache_map = self.entity_cache.borrow_mut();
                cache_map.entry(entity_id).or_insert_with(|| {
                    let vertices: Vec<[f32; 3]> = shape
                        .get_vertices()
                        .iter()
                        .map(|v| [v.x, v.y, v.z])
                        .collect();

                    let vertex_buffer = self.gpu_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Entity Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    );

                    let vertex_count = u32::try_from(vertices.len()).unwrap_or(0);
                    let color_data: [f32; 4] = rgba.to_normalized_f32_array();

                    let color_buffer = self.gpu_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Color Uniform Buffer"),
                            contents: bytemuck::cast_slice(&color_data),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        },
                    );

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
