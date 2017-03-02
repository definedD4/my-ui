use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::cmp::Eq;

pub trait Element {
    fn print(&self);
}

struct ElementNode {
    element: Box<Element>,
    parrent: Option<ElementNodeWeakRef>,
    children: Vec<ElementNodeRef>,
}

#[derive(Clone)]
pub struct ElementNodeRef {
    node: Rc<RefCell<ElementNode>>,
}

#[derive(Clone)]
pub struct ElementNodeWeakRef {
    node: Weak<RefCell<ElementNode>>,
}

pub struct ElementTree {
    root: Option<ElementNodeRef>,
}

impl ElementNode {
    fn new(element: Box<Element>,
           parrent: Option<ElementNodeWeakRef>) -> ElementNode {
        ElementNode { 
            element: element,
            parrent: parrent,
            children: Vec::new(),
        }
    }

    fn element(&self) -> &(Element + 'static) {
        &*self.element
    }

    fn children(&self) -> &[ElementNodeRef] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ElementNodeRef> {
        &mut self.children
    }

    fn parrent(&self) -> Option<ElementNodeRef> {
        self.parrent.as_ref().and_then(|r| r.upgrade())
    }
}

impl ElementNodeRef {
    fn new(node: Rc<RefCell<ElementNode>>) -> ElementNodeRef {
        ElementNodeRef { node: node}
    }

    pub fn downgrade(&self) -> ElementNodeWeakRef {
        ElementNodeWeakRef::new(Rc::downgrade(&self.node))
    }

    pub fn element(&self) -> Ref<Element + 'static> {
        Ref::map(self.node.borrow(), |n| n.element())
    }

    pub fn children(&self) -> Ref<[ElementNodeRef]> {
        Ref::map(self.node.borrow(), |n| n.children())
    }

    pub fn add_child<T: Element + 'static>(&mut self, element: T) {
        self.node.borrow_mut().children_mut().push(
            ElementNodeRef::new(
                Rc::new(RefCell::new(ElementNode::new(
                    Box::new(element),
                    Some(self.downgrade())
                )))
            )
        )
    }

    fn parrent(&self) -> Option<ElementNodeRef> {
        self.node.borrow().parrent()
    }
}

impl PartialEq for ElementNodeRef {
    fn eq(&self, other: &ElementNodeRef) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }
}
impl Eq for ElementNodeRef {}

impl ElementNodeWeakRef {
    fn new(node: Weak<RefCell<ElementNode>>) -> ElementNodeWeakRef {
        ElementNodeWeakRef { node: node}
    }

    pub fn upgrade(&self) -> Option<ElementNodeRef> {
        Weak::upgrade(&self.node).map(|r| ElementNodeRef::new(r))
    }
}

impl ElementTree {
    pub fn new() -> ElementTree {
        ElementTree { root: None }
    }

    pub fn root(&self) -> Option<ElementNodeRef> {
        self.root.clone()
    }

    pub fn set_root<T: Element + 'static>(&mut self, element: Option<T>) {
        self.root = element.map(|e| ElementNodeRef::new(
            Rc::new(RefCell::new(ElementNode::new(
                Box::new(e),
                None
            ))))
        )
    }
}

#[cfg(test)]
mod test {
    impl Element for i32 {
        fn print(&self) {
            println!("{}", &self);
        }
    }

    use super::*;

    #[test]
    fn new_tree_has_empty_root() {
        let tree = ElementTree::new();
        assert!(tree.root() == None);
    }

    #[test]
    fn tree_set_root_sets_root() {
        let mut tree = ElementTree::new();
        tree.set_root(Some(42));

        let root = tree.root();

        assert!(root.is_some());
    }

    #[test]
    fn tree_root_has_no_parrent() {
        let mut tree = ElementTree::new();
        tree.set_root(Some(42));

        let root = tree.root().unwrap();

        assert!(root.parrent() == None);
    }
}