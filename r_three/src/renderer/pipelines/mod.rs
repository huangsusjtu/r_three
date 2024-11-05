use winit::dpi::Size;

use crate::camera::Camera;

mod mesh;
mod pipeline_storage;
pub use pipeline_storage::*;

pub use mesh::*;

pub trait Pipeline {
    /// 数据刷到GPU侧
    fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue, camera: &dyn Camera);

    /// 绘制
    fn draw(&self, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder);

    /// 释放GPU资源
    fn destroy(&self);

}
