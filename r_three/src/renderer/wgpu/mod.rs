pub mod context;
pub use context::*;
mod objects;
pub mod wgpu_renderer;

pub use wgpu_renderer::*;

pub trait VertexInterface {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
