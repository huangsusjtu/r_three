use crate::renderer::primitive::MeshPrimitive;
use crate::{Primitive, RenderContext, VertexInterface};

impl<T> MeshPrimitive<T>
where
    T: VertexInterface + bytemuck::Pod,
{
    pub fn fill_data(&mut self, vertx: Vec<T>, indices: Vec<u32>) {
        self.dirty_data = Some((vertx, indices));
    }
}

impl<T> Primitive for MeshPrimitive<T>
where
    T: VertexInterface + bytemuck::Pod,
{
    fn render(&mut self, context: RenderContext) -> anyhow::Result<()> {
        if self.dirty_data.is_some() {
            let mut storage = context.storage.write().unwrap();
            let pipeline = storage.get_mut::<crate::pipelines::MeshPipeline>().unwrap();

            let data = std::mem::take(&mut self.dirty_data);
            let new_id = pipeline.add_mesh_data(data.unwrap())?;
            if let Some(old_id) = self.data_id {
                pipeline.remove_mesh_data(old_id);
            }
            self.data_id = Some(new_id);
        }
        Ok(())
    }

    fn destroy(&mut self, context: RenderContext) -> anyhow::Result<()> {
        if let Some(id) = self.data_id {
            context
                .storage
                .write()
                .unwrap()
                .get_mut::<crate::pipelines::MeshPipeline>()
                .unwrap()
                .remove_mesh_data(id);
        }

        Ok(())
    }
}
