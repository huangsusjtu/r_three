mod mesh;

pub use mesh::*;

use crate::vertex::Vertex;
use crate::{Primitive, VertexInterface};

pub(crate) struct MeshPrimitive<T: VertexInterface + Sized> {
    dirty_data: Option<(Vec<T>, Vec<u32>)>,

    data_id: Option<u32>,
}

impl<T> Default for MeshPrimitive<T>
where
    T: VertexInterface + Sized,
{
    fn default() -> Self {
        MeshPrimitive {
            dirty_data: None,
            data_id: None,
        }
    }
}
