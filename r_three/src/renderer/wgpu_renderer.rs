use std::any::{Any, TypeId};
use std::iter;
use std::sync::{Arc, RwLock};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::*;
use winit::window::Window;

use crate::camera::Camera;
use crate::color::Color;
use crate::renderer::RendererInterface;
use crate::pipelines::{MeshPipeline, Pipeline, PipelineStorage};
use crate::{RenderContext, WgpuRenderer};
use crate::{renderer, Scene};



impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let default_backends = if cfg!(feature = "opengl") {
            wgpu::Backends::GL
        } else {
            wgpu::Backends::PRIMARY
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env().unwrap_or(default_backends),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let mut config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &config);

        let caps = surface.get_capabilities(&adapter);
        let prefer_texture_format = config.format;
        let format = if cfg!(all(target_arch = "wasm32", not(feature = "webgl"))) {
            // Chrome WebGPU doesn't support sRGB:
            // unsupported swap chain format "xxxx8unorm-srgb"
            prefer_texture_format.remove_srgb_suffix()
        } else {
            prefer_texture_format
        };
        config.format = prefer_texture_format;

        let mut renderer = Self {
            window,
            surface,
            _adapter: adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            config,
            size,
            pipelines: Arc::new(RwLock::new(PipelineStorage::default())),
        };
        renderer.init_pipeline();

        renderer
    }
    fn init_pipeline(&mut self) {
        // init pipelines
        let mut pipelines = self.pipelines.write().unwrap();
        pipelines.store(MeshPipeline::new(
            self.device.clone(),
            self.queue.clone(),
            self.config.format,
        ));
        // todo: huangsu add more
    }

    fn draw_background(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        background_color: Color,
    ) {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: background_color.r,
                        g: background_color.g,
                        b: background_color.b,
                        a: background_color.a,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
    }

    fn draw_scene(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        scene: &Scene,
        camera: &dyn Camera,
    ) {
        let render_context = RenderContext {
            storage: self.pipelines.clone(),
            target_view: &target,
            camera,
        };

        // 把场景里所有的节点里的脏数据 刷新到GPU
        // 如果没有脏数据， render方法等于是空
        {
            scene.tree.into_iter().for_each(|node| {
                if let Some(render_node) = node.borrow().to_primitive() {
                    _ = render_node.borrow_mut().render(render_context.clone());
                }
            });
        }
    }

    fn flush_pipeline(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        camera: &dyn Camera,
    ) {
        let mut pipelines = self.pipelines.write().unwrap();
        if let Some(m) = pipelines.get_mut::<MeshPipeline>() {
            m.update(self.device.as_ref(), self.queue.as_ref(), camera); // pipeline本身的少量数据刷新到GPU
            m.draw(&target, encoder); // 真正的调GPU绘制
        }
    }
}
impl RendererInterface for WgpuRenderer {
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self, scene: &Scene, camera: &dyn Camera) -> anyhow::Result<()> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // 绘制 background
        self.draw_background(&view, &mut encoder, scene.background_color);
        // 绘制场景
        self.draw_scene(&view, &mut encoder, scene, camera);
        // pipeline刷新绘制
        self.flush_pipeline(&view, &mut encoder, camera);


        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
