use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use crate::core::group::Group;
use crate::Object3D;

type TreeNode = Rc<RefCell<Box<dyn Object3D>>>;

/// 3d对象树
pub struct ObjectTree
{
    root: TreeNode,
}

impl ObjectTree
{
    pub fn new() -> Self {
        ObjectTree {
            root: Group::new().to_object(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) -> bool {
        self.root.borrow_mut().add_child(child)
    }

    pub fn remove_child(&mut self, child_id: u32) -> bool {
        self.root.borrow_mut().remove_child(child_id)
    }

    pub fn clear(&mut self) {
        // todo:
    }
}

pub struct TreeIterator
{
    nodes: LinkedList<TreeNode>,
}

impl Iterator for TreeIterator

{
    type Item = TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop_front()
    }
}

impl IntoIterator for &ObjectTree
{
    type Item = TreeNode;
    type IntoIter = TreeIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut nodes = LinkedList::new();
        let mut queue = LinkedList::new();
        queue.push_back(self.root.clone());
        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            nodes.push_back(p.clone());
            let p = p.borrow();
            for i in 0..p.child_num() {
                let c = p.get_child_by_index(i);
                queue.push_back(c);
            }
        }

        TreeIterator { nodes }
    }
}
