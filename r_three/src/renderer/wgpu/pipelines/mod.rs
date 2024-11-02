use crate::camera::CameraInterface;
use winit::dpi::Size;

mod mesh;

pub trait Pipeline {
    /// 数据刷到GPU侧
    fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_size: Size,
        camera: &dyn CameraInterface,
    );

    /// 绘制
    fn draw(&self, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder);


    /// 释放资源
    fn destroy(&mut self);
}
