use crate::core::next_object_id;
use crate::{Object3D, RenderContext, Renderable};
use r_three_macro::{add_object3d_attribute, object3d};
use std::cell::RefCell;
use std::rc::Rc;

#[add_object3d_attribute]
#[derive(object3d)]
pub struct Group {}

impl Group {
    pub fn new() -> Self {
        Group {
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
impl Renderable for Group {
    fn render(&mut self, context: RenderContext) -> anyhow::Result<()> {
        Ok(())
    }
}
