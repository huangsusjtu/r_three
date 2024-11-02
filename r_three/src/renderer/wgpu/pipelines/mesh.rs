use crate::camera::CameraInterface;
use crate::vertex::Vertex;
use crate::wgpu::pipelines::Pipeline;
use crate::wgpu::VertexInterface;
use wgpu::util::DeviceExt;
use winit::dpi::Size;

pub struct MeshPipeline<'a> {
    device: &'a wgpu::Device,
    pipeline: wgpu::RenderPipeline,

    // vertex data
    data: Option<Data>,

    // uniform
    uniform_bind_group: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
}

impl<'a> MeshPipeline<'a> {
    pub fn new(device: &'a wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        // uniform data format
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(" uniform buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
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
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
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
            data: None,
            uniform_bind_group,
            uniforms,
        }
    }

    pub fn update_mesh_data(&mut self, mesh: (Vec<Vertex>, Vec<u32>)) {
        if let Some(old) = self.data.as_ref() {
            old.vertex_buffer.destroy();
            old.index_buffer.destroy();
        }
        //vertices of face
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

        self.data = Some(Data {
            vertex_buffer,
            index_buffer,
            num_indices,
        });
    }
}

impl<'a> Pipeline for MeshPipeline<'a> {
    fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_size: Size,
        camera: &dyn CameraInterface,
    ) {
        queue.write_buffer(&self.uniforms, 0, bytemuck::bytes_of(&Uniforms::new(camera)));
    }

    fn draw(&self, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        if let Some(data) = self.data.as_ref() {
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

            // render_pass.set_scissor_rect(
            //     viewport.x,
            //     viewport.y,
            //     viewport.width,
            //     viewport.height,
            // );

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
            render_pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..data.num_indices, 0, 0..1);
        }
    }

    fn destroy(&mut self) {
        if let Some(old) = self.data.as_ref() {
            old.vertex_buffer.destroy();
            old.index_buffer.destroy();
        }
        self.uniforms.destroy();
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    camera_proj: glam::Mat4,
    camera_pos: glam::Vec4,
}

impl Uniforms {
    pub fn new(camera: &dyn CameraInterface) -> Self {
        let camera_proj = camera.build_view_projection_matrix();

        Self {
            camera_proj,
            camera_pos: glam::Vec4::new(camera.eye().x, camera.eye().y, camera.eye().z, 1.0),
        }
    }
}

struct Data {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}
