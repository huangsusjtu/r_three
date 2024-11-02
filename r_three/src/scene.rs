use crate::color::Color;
use crate::{Object3D, ObjectTree};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Scene {
    pub(crate) tree: ObjectTree,

    pub(crate) background_color: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            tree: ObjectTree::new(),
            background_color: Color::WHITE,
        }
    }

    ///设置场景的背景色
    pub fn set_clear_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn add(&mut self, obj: Rc<RefCell<Box<dyn Object3D>>>) -> bool {
        self.tree.add_child(obj)
    }
    pub fn remove(&mut self, obj: Rc<RefCell<Box<dyn Object3D>>>) -> bool {
        let obj_id = obj.borrow().id();
        self.tree.remove_child(obj_id)
    }
}

unsafe impl Sync for Scene {}
unsafe impl Send for Scene {}

