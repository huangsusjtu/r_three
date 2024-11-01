use crate::core::next_object_id;
use crate::geometries::Geometry;
use crate::materials::MeshBasicMaterial;
use crate::Object3D;
use r_three_macro::{add_object3d_attribute, object3d};
use std::cell::RefCell;
use std::rc::Rc;

#[add_object3d_attribute]
#[derive(object3d)]
pub struct Mesh {}

impl Mesh {
    pub fn new(geometry: Box<dyn Geometry>, material: MeshBasicMaterial) -> Mesh {
        Mesh {
            id: next_object_id(),
            is_self_dirty: false,
            is_child_dirty: false,
            position: glam::Mat4::IDENTITY,

            parent: None,
            children: vec![],
            this: None,
        }
    }
}
