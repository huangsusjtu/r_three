pub mod context;
pub use context::*;
mod objects;


pub mod wgpu_renderer;
mod pipelines;

pub use wgpu_renderer::*;

pub trait VertexInterface {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
