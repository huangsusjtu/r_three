use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use bytemuck::Pod;
use rand::Rng;
use wgpu::util::DeviceExt;

use crate::camera::Camera;
use crate::renderer::pipelines::Pipeline;
use crate::vertex::{Vertex, VertexWithColor};
use crate::VertexInterface;

pub struct MeshPipeline {
    device: Arc<wgpu::Device>,
    pipeline: wgpu::RenderPipeline,

    // vertex data
    data: HashMap<u32, Data>,

    // uniform
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl MeshPipeline {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>, format: wgpu::TextureFormat) -> Self {
        // uniform data format
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera uniform buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera uniform bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera uniform bind group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("mesh shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../shaders/mesh.wgsl"
                ))),
            });
            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("mesh render pipeline layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Mesh Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    compilation_options: Default::default(),
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
                cache: None,
            })
        };

        Self {
            device,
            pipeline,
            data: Default::default(),
            camera_bind_group,
            camera_buffer,
        }
    }

    pub fn add_mesh_data<T: Pod>(&mut self, mesh: (Vec<T>, Vec<u32>)) -> anyhow::Result<u32> {
        // vertices of face
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(" vertex buffer"),
            contents: bytemuck::cast_slice(&mesh.0),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(" index Buffer"),
            contents: bytemuck::cast_slice(&mesh.1),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = mesh.1.len() as u32;

        //   分配数据ID
        let mut rng = rand::thread_rng();
        let mut id: u32 = rng.gen();
        while self.data.contains_key(&id) {
            id = rng.gen();
        }
        self.data.insert(
            id,
            Data {
                vertex_buffer,
                index_buffer,
                num_indices,
            },
        );
        Ok(id)
    }

    pub fn remove_mesh_data(&mut self, id: u32) {
        if let Some(data) = self.data.remove(&id) {
            data.vertex_buffer.destroy();
            data.index_buffer.destroy();
        }
    }
}

impl Pipeline for MeshPipeline {
    fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue, camera: &dyn Camera) {
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&CameraUniform::new(camera)),
        );
    }

    fn draw(&self, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        if self.data.is_empty() {
            return;
        }

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mesh.pipeline.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

        for (_id, data) in self.data.iter() {
            render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
            render_pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..data.num_indices, 0, 0..1);
        }
    }

    fn destroy(&self) {
        for (_id, data) in self.data.iter() {
            data.vertex_buffer.destroy();
            data.index_buffer.destroy();
        }
        self.camera_buffer.destroy();
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        CameraUniform {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
impl CameraUniform {
    pub fn new(camera: &dyn Camera) -> Self {
        let view_proj = camera.build_view_projection_matrix().to_cols_array_2d();

        // tracing::trace!("{:#?}", view_proj);
        Self { view_proj }
    }
}

struct Data {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}
