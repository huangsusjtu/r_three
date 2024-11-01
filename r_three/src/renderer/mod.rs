use crate::Scene;

#[cfg(feature = "opengl")]
pub mod opengl;
pub mod vertex;
#[cfg(feature = "wgpu")]
pub mod wgpu;

pub trait RendererInterface {
    fn init(&mut self) {}

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);

    fn update(&mut self) {}

    fn render(&mut self, scene: &Scene) -> anyhow::Result<()>;
}

#[cfg(feature = "wgpu")]
pub type RenderContext = f64;
#[cfg(feature = "opengl")]
pub type RenderContext = f64;

pub trait Renderable {
    fn render(&mut self, context: RenderContext) -> anyhow::Result<()>;
}
