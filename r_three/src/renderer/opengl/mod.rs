use crate::renderer::RendererInterface;
use winit::dpi::PhysicalSize;

pub(crate) struct GLRenderer {}

impl RendererInterface for GLRenderer {
    fn init(&mut self) {
        todo!()
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
