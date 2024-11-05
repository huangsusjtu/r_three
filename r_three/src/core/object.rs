use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicU32;
use crate::Primitive;

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
    fn get_child_by_index(&self, index: usize) -> Rc<RefCell<Box<dyn Object3D>>>;
    fn child_num(&self) -> usize;

    fn set_this(&mut self, this: Rc<RefCell<Box<dyn Object3D>>>);
    fn set_parent(&mut self, parent: Option<Rc<RefCell<Box<dyn Object3D>>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<Box<dyn Object3D>>>>;
    fn to_object(self) -> Rc<RefCell<Box<dyn Object3D>>>;

    fn to_primitive(&self) -> Option<Rc<RefCell<Box<dyn Primitive>>>>;
    fn on_before_render(&self) {}

    fn on_after_render(&self) {}
}
