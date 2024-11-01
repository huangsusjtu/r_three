use crate::core::group::Group;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicU32;

type ObjectTreeNode = Rc<RefCell<Box<dyn Object3D>>>;

/// 3d对象树
pub struct ObjectTree {
    root: ObjectTreeNode,
}

impl ObjectTree {
    pub fn new() -> Self {
        ObjectTree {
            root: Group::new().as_object(),
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Box<dyn Object3D>>>) -> bool {
        self.root.borrow_mut().add_child(child)
    }
    pub fn remove_child(&mut self, child_id: u32) -> bool {
        self.root.borrow_mut().remove_child(child_id)
    }

    pub fn clear(&mut self) {
        // todo:
    }
}

#[inline]
pub(crate) fn next_object_id() -> u32 {
    static OBJECT_ID: AtomicU32 = AtomicU32::new(0);
    OBJECT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub trait Object3D: Any {
    fn id(&self) -> u32;

    fn mark_dirty(&mut self);

    fn is_dirty(&self) -> bool;

    fn mark_child_dirty(&mut self);
    fn has_child_dirty(&self) -> bool;

    fn add_child(&mut self, child: Rc<RefCell<Box<dyn Object3D>>>) -> bool;
    fn remove_child(&mut self, child_id: u32) -> bool;

    fn set_this(&mut self, this: Rc<RefCell<Box<dyn Object3D>>>);
    fn set_parent(&mut self, parent: Option<Rc<RefCell<Box<dyn Object3D>>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<Box<dyn Object3D>>>>;
    fn as_object(self) -> Rc<RefCell<Box<dyn Object3D>>>;
    fn on_before_render(&self) {}

    fn on_after_render(&self) {}
}
