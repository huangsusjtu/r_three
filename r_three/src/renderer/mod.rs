use std::sync::{Arc, RwLock};
use winit::window::Window;
use crate::camera::Camera;
use crate::pipelines::PipelineStorage;
use crate::Scene;

pub mod pipelines;
pub mod primitive;
pub mod vertex;
mod wgpu_renderer;
mod shaders;

pub trait VertexInterface {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub trait RendererInterface {
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);

    fn render(&mut self, scene: &Scene, camera: &dyn Camera) -> anyhow::Result<()>;
}

pub struct WgpuRenderer {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    _adapter: wgpu::Adapter,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,

    pipelines: Arc<RwLock<PipelineStorage>>,
}

#[derive(Clone)]
pub(crate) struct RenderContext<'a> {
    pub storage: Arc<RwLock<pipelines::PipelineStorage>>,
    pub(crate) target_view: &'a wgpu::TextureView,
    pub(crate) camera: &'a dyn Camera,
}

pub trait Primitive {
    fn render(&mut self, context: RenderContext) -> anyhow::Result<()>;
    fn destroy(&mut self, context: RenderContext) -> anyhow::Result<()>;
}
