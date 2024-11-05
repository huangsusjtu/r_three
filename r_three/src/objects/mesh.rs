use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use r_three_macro::{add_object3d_attribute, object3d};

use crate::core::next_object_id;
use crate::geometries::{CircleGeometry, Geometry};
use crate::materials::MeshBasicMaterial;
use crate::primitive::MeshPrimitive;
use crate::vertex::VertexWithColor;
use crate::{Object3D, Primitive};

#[add_object3d_attribute]
#[derive(object3d)]
pub struct Mesh {
    geometry: Box<dyn Geometry>,
    material: MeshBasicMaterial,
}

impl Mesh {
    pub fn new(geometry: Box<dyn Geometry>, material: MeshBasicMaterial) -> Mesh {
        let mut mesh = Mesh {
            id: next_object_id(),
            is_self_dirty: false,
            is_child_dirty: false,
            position: glam::Mat4::IDENTITY,

            parent: None,
            children: vec![],
            this: None,
            primitive: None,

            geometry,
            material,
        };

        mesh.generate();

        mesh
    }

    fn generate(&mut self) {
        // 创建数据片元
        let mut primitive = MeshPrimitive::default();
        // let color_vertex = self
        //     .geometry
        //     .vertices()
        //     .iter()
        //     .map(|v| VertexWithColor {
        //         position: v.position,
        //         color: self.material.color.into(),
        //     })
        //     .collect();

        primitive.fill_data(self
                                .geometry
                                .vertices(), self.geometry.indices());
        self.primitive = Some(Rc::new(RefCell::new(Box::new(primitive))));
    }
}
