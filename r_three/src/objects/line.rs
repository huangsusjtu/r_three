use crate::core::next_object_id;
use crate::geometries::{CircleGeometry, Geometry};
use crate::materials::LineBasicMaterial;
use crate::vertex::Vertex;
use crate::{BufferGeometry, Object3D};
use r_three_macro::{add_object3d_attribute, object3d};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

#[add_object3d_attribute]
#[derive(object3d)]
pub struct Line {
    geometry: BufferGeometry,
    material: LineBasicMaterial,
}

impl Line {
    pub fn new(geometry: BufferGeometry, material: LineBasicMaterial) -> Self {
        let mut line = Line {
            id: next_object_id(),
            is_self_dirty: false,
            is_child_dirty: false,
            position: glam::Mat4::IDENTITY,

            parent: None,
            children: vec![],
            this: None,

            //
            geometry,
            material,
        };
        line.generate();

        line
    }

    fn generate(&self) {
        let mut vertex: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];

        let points = &self.geometry.position;
        let thickness = self.material.line_width;
        // 1. compute triangle vertex by points
        for i in 1..self.geometry.position.len() {
            let theta = {
                let mut theta = (points[i].y - points[i - 1].y).atan2(points[i].x - points[i - 1].x);
                theta as f32 + PI / 2.0
            };
            let (delta_x, delta_y) = (theta.cos() * thickness / 2.0, theta.sin() * thickness / 2.0);

            vertex.push(Vertex {
                position: [
                    points[i - 1].x as f32 + delta_x,
                    points[i - 1].y as f32 + delta_y,
                    points[i - 1].z as f32,
                ],
            });
            vertex.push(Vertex {
                position: [
                    points[i - 1].x as f32 - delta_x,
                    points[i - 1].y as f32 - delta_y,
                    points[i - 1].z as f32,
                ],
            });
            vertex.push(Vertex {
                position: [
                    points[i].x as f32 + delta_x,
                    points[i].y as f32 + delta_y,
                    points[i].z as f32,
                ],
            });
            vertex.push(Vertex {
                position: [
                    points[i].x as f32 - delta_x,
                    points[i].y as f32 - delta_y,
                    points[i].z as f32,
                ],
            });
            let l = (vertex.len() - 1) as u32;
            indices.extend([l - 3, l - 2, l - 1]);
            indices.extend([l - 1, l - 2, l]);
        }

        // 2.
        if self.material.linecap.eq("round") {
            let c1 = CircleGeometry::new(
                points.first().unwrap().clone(),
                thickness / 2.0,
                16,
                0.0,
                PI * 2.0,
            );
            let c2 = CircleGeometry::new(points.last().unwrap().clone(), thickness / 2.0, 16, 0.0, PI * 2.0);
            merge_vertex_and_indices(&mut vertex, &mut indices, c1.vertices(), c1.indices());
            merge_vertex_and_indices(&mut vertex, &mut indices, c2.vertices(), c2.indices());
        } else if self.material.linecap.eq("square") {
            // todo
        } else {
        }

        // 3.
        if self.material.linejoin.eq("round") {
            for i in 1..points.len() - 1 {
                let c = CircleGeometry::new(points[i], thickness / 2.0, 16, 0.0, PI * 2.0);
                merge_vertex_and_indices(&mut vertex, &mut indices, c.vertices(), c.indices());
            }
        } else if self.material.linejoin.eq("square") {
            // todo
        } else {
        }
    }
}

fn merge_vertex_and_indices(
    vertex: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    mut vertex_a: Vec<Vertex>,
    mut indices_a: Vec<u32>,
) {
    indices_a.iter().for_each(|i| {
        indices.push(*i + vertex.len() as u32);
    });
    vertex.append(&mut vertex_a);
}
//
// impl Renderable for Line {
//     fn render(&mut self, context: RenderContext) -> anyhow::Result<()> {
//         Ok(())
//     }
// }
