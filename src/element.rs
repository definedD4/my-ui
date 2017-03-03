use tree;

pub trait Element {

}

pub type ElementTree = tree::Tree<ElementContext>;

pub type ElementNodeRef = tree::NodeRef<ElementContext>;

pub type ElementNodeWeakRef = tree::NodeWeakRef<ElementContext>;

pub struct ElementContext {
    element: Box<Element>,
    node: ElementNodeWeakRef,
}

impl ElementContext {
    fn new(element: Box<Element>, node: ElementNodeWeakRef) -> ElementContext {
        ElementContext {
            element: element,
            node: node
        }
    }

    fn set_node(&mut self, node: ElementNodeWeakRef) {
        self.node = node;
    }
}

impl ElementTree {
    pub fn set_root_element(&mut self, element: Box<Element>) {
        self.set_root(Some(ElementContext::new(element, ElementNodeWeakRef::empty())));
        let mut root = self.root().unwrap().clone();
        let weak = root.downgrade();
        root.data_mut().set_node(weak);
    }
}