use crate::geometries::Geometry;
use crate::vertex::Vertex;
use glam::Vec3;

pub struct BoxGeometry {
    vertices: Vec<Vertex>,
    normals: Vec<Vec3>,
    indices: Vec<u32>,
}

impl BoxGeometry {
    pub fn new(length: f32, width: f32, height: f32) -> BoxGeometry {
        let vertices = vec![
            Vertex {
                position: [-length / 2.0, -width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [length / 2.0, -width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [length / 2.0, width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [-length / 2.0, width / 2.0, -height / 2.0],
            },
            Vertex {
                position: [-length / 2.0, -width / 2.0, height / 2.0],
            },
            Vertex {
                position: [length / 2.0, -width / 2.0, height / 2.0],
            },
            Vertex {
                position: [length / 2.0, width / 2.0, height / 2.0],
            },
            Vertex {
                position: [-length / 2.0, width / 2.0, height / 2.0],
            },
        ];

        let normals = vec![
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 1, 5, 6, 6, 2, 1, 5, 4, 7, 7, 6, 5, 4, 0, 3, 3, 7, 4, 3, 2, 6, 6, 7, 3, 4, 5,
            1, 1, 0, 4,
        ];

        BoxGeometry {
            vertices,
            normals,
            indices,
        }
    }
}

impl Geometry for BoxGeometry {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    // fn normals(&self) -> Vec<Vec3> {
    //     self.normals.clone()
    // }

    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
