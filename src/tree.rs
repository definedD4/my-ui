use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};


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

    fn element<'a>(&'a self) -> &'a Element {
        &*self.element
    }

    fn children(&self) -> &[ElementNodeRef] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<ElementNodeRef> {
        &mut self.children
    }
}

impl ElementNodeRef {
    fn new(node: Rc<RefCell<ElementNode>>) -> ElementNodeRef {
        ElementNodeRef { node: node}
    }

    pub fn downgrade(&self) -> ElementNodeWeakRef {
        ElementNodeWeakRef::new(Rc::downgrade(&self.node))
    }

    pub fn element<'a>(&'a self) -> Ref<'a, Element> {
        Ref::map(self.node.borrow(), |n| n.element::<'a>())
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
}

impl ElementNodeWeakRef {
    fn new(node: Weak<RefCell<ElementNode>>) -> ElementNodeWeakRef {
        ElementNodeWeakRef { node: node}
    }

    pub fn upgrade(&self) -> ElementNodeRef {
        // TODO: remove unwrap
        ElementNodeRef::new(Weak::upgrade(&self.node).unwrap())
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