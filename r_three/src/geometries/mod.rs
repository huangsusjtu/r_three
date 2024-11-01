mod box_geometry;
pub use box_geometry::*;

mod shape_geometry;
pub use shape_geometry::*;
mod circle_geometry;
pub use circle_geometry::*;

use crate::vertex::Vertex;

// 定义几何形状的 trait
pub trait Geometry {
    fn vertices(&self) -> Vec<Vertex>;
    // fn normals(&self) -> Vec<Vec3>;
    fn indices(&self) -> Vec<u32>;
}
