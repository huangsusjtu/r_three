use crate::geometries::Geometry;
use crate::vertex::Vertex;
use glam::{Vec2, Vec3};

pub struct ShapeGeometry {
    vertices: Vec<Vertex>,
    uvs: Option<Vec<Vec2>>,
    indices: Vec<u32>,
}

impl ShapeGeometry {
    pub fn new() -> ShapeGeometry {
        ShapeGeometry {
            vertices: vec![],
            uvs: None,
            indices: vec![],
        }
    }

    pub fn add(&mut self, p: glam::Vec3) {
        self.vertices.push(Vertex {
            position: [p.x, p.y, p.z],
        });
        if self.vertices.len() >= 3 {
            for i in 0..self.vertices.len() - 2 {
                self.indices.extend([i as u32, i as u32 + 1, self.vertices.len() as u32 - 1]);
            }
        }
    }
}

impl From<Vec<Vec3>> for ShapeGeometry {
    fn from(vertices: Vec<Vec3>) -> Self {
        let mut indices = vec![];
        for i in 0..vertices.len() - 2 {
            indices.extend([i as u32, i as u32 + 1, vertices.len() as u32 - 1]);
        }
        ShapeGeometry {
            vertices: vertices
                .iter()
                .map(|p| Vertex {
                    position: [p.x, p.y, p.z],
                })
                .collect(),
            uvs: None,
            indices,
        }
    }
}

impl Geometry for ShapeGeometry {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
