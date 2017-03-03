use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::cmp::Eq;

struct Node<T> {
    data: T,
    parrent: Option<NodeWeakRef<T>>,
    children: Vec<NodeRef<T>>,
}

pub struct NodeRef<T> {
    node: Rc<RefCell<Node<T>>>,
}

pub struct NodeWeakRef<T> {
    node: Weak<RefCell<Node<T>>>,
}

pub struct Tree<T> {
    root: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    fn new(data: T,
           parrent: Option<NodeWeakRef<T>>) -> Node<T> {
        Node { 
            data: data,
            parrent: parrent,
            children: Vec::new(),
        }
    }

    fn data(&self) -> &T {
        &self.data
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn children(&self) -> &[NodeRef<T>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<NodeRef<T>> {
        &mut self.children
    }

    fn parrent(&self) -> Option<NodeRef<T>> {
        self.parrent.as_ref().and_then(|r| r.upgrade())
    }
}

impl<T> NodeRef<T> {
    fn new(node: Rc<RefCell<Node<T>>>) -> NodeRef<T> {
        NodeRef { node: node }
    }

    fn new_node(data: T, parrent: Option<NodeWeakRef<T>>) -> NodeRef<T> {
        NodeRef::new(Rc::new(RefCell::new(Node::new(data, parrent))))
    }

    pub fn downgrade(&self) -> NodeWeakRef<T> {
        NodeWeakRef::new(Rc::downgrade(&self.node))
    }

    pub fn data(&self) -> Ref<T> {
        Ref::map(self.node.borrow(), |n| n.data())
    }

    pub fn data_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.node.borrow_mut(), |n| n.data_mut())
    }

    pub fn children(&self) -> Ref<[NodeRef<T>]> {
        Ref::map(self.node.borrow(), |n| n.children())
    }

    pub fn add_child(&mut self, child: T) {
        self.node.borrow_mut().children_mut().push(
            NodeRef::new_node(child, Some(self.downgrade()))
        );
    }

    fn parrent(&self) -> Option<NodeRef<T>> {
        self.node.borrow().parrent()
    }
}

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> NodeRef<T> {
        NodeRef::new(self.node.clone())
    }
}

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &NodeRef<T>) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }
}
impl<T> Eq for NodeRef<T> {}

impl<T> NodeWeakRef<T> {
    fn new(node: Weak<RefCell<Node<T>>>) -> NodeWeakRef<T> {
        NodeWeakRef { node: node }
    }

    pub fn empty() -> NodeWeakRef<T> {
        NodeWeakRef { node: Weak::new() }
    }

    pub fn upgrade(&self) -> Option<NodeRef<T>> {
        Weak::upgrade(&self.node).map(|r| NodeRef::new(r))
    }
}

impl<T> Clone for NodeWeakRef<T> {
    fn clone(&self) -> NodeWeakRef<T> {
        NodeWeakRef::new(self.node.clone())
    }
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn root(&self) -> Option<NodeRef<T>> {
        self.root.as_ref().map(|r| r.clone())
    }

    pub fn set_root(&mut self, data: Option<T>) {
        self.root = data.map(|d| NodeRef::new_node(d, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree_has_empty_root() {
        let tree = Tree::<i32>::new();
        assert!(tree.root() == None);
    }

    #[test]
    fn tree_set_root_sets_root() {
        let mut tree = Tree::<i32>::new();
        tree.set_root(Some(42));

        let root = tree.root();

        assert!(root.is_some());

        let root = root.unwrap();
        
        assert_eq!(*root.data(), 42);
    }

    #[test]
    fn tree_root_has_no_parrent() {
        let mut tree = Tree::<i32>::new();
        tree.set_root(Some(42));

        let root = tree.root().unwrap();

        assert!(root.parrent() == None);
    }

    #[test]
    fn add_child_adds_child() {
        let mut tree = Tree::<i32>::new();
        tree.set_root(Some(0));
        let mut root = tree.root().unwrap();

        assert_eq!(root.children().len(), 0);
        
        root.add_child(1);

        assert_eq!(root.children().len(), 1);
        {
            let child0 = root.children()[0].clone();
            assert_eq!(*child0.data(), 1);
        }

        root.add_child(2);
        
        assert_eq!(root.children().len(), 2);
        {
            let child0 = root.children()[0].clone();
            let child1 = root.children()[1].clone();
            assert_eq!(*child0.data(), 1);
            assert_eq!(*child1.data(), 2);
        }
    }
}