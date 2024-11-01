use crate::geometries::Geometry;
use crate::vertex::Vertex;

pub struct CircleGeometry {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    normals: Vec<glam::Vec3>,
}

impl CircleGeometry {
    pub fn new(center: glam::Vec3, radius: f32, segments: u32, theta_start: f32, theta_length: f32) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normals: Vec<glam::Vec3> = Vec::new();
        let mut uvs: Vec<glam::Vec2> = Vec::new();

        vertices.push(Vertex {
            position: [center.x + 0.0, center.y + 0.0, center.z + 0.0],
        });
        normals.push(glam::Vec3::new(0.0, 0.0, 1.0));

        let mut i = 3;
        for s in 0..segments + 1 {
            let segment = theta_start + s as f32 / segments as f32 * theta_length;
            vertices.push(Vertex {
                position: [
                    center.x + radius * segment.cos(),
                    center.y + radius * segment.sin(),
                    center.z + 0.0,
                ],
            });
            normals.push(glam::Vec3::new(0.0, 0.0, 1.0));

            i += 3;
        }
        for i in 1..segments {
            indices.extend([i, i + 1, 0]);
        }
        CircleGeometry {
            vertices,
            indices,
            normals,
        }
    }
}

impl Geometry for CircleGeometry {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
