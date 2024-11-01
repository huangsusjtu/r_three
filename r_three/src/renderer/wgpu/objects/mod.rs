use crate::objects::Line;
use crate::{RenderContext, Renderable};

impl Renderable for Line {
    fn render(&mut self, context: RenderContext) -> anyhow::Result<()> {
        Ok(())
    }
}
