use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::cmp::Eq;
use primitives::*;
use render::*;
use std::any::{Any, TypeId};

pub trait Element: Any {
    fn init(&mut self, node: NodeRef);
    fn measure(&self, node: NodeRef) -> Size;
    fn layout(&mut self, node: NodeRef, container: Size);
    fn render(&self, node: NodeRef) -> RenderCommandList;
}

impl Element {
    #[inline]
    pub fn can_cast<T: Any>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = self.get_type_id();
        t == boxed
    }

    #[inline]
    pub fn cast_element_ref<T: Any>(&self) -> Option<&T> {
        if self.can_cast::<T>() {
            unsafe {
                Some(&*(self as *const Element as *const T))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn cast_element_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.can_cast::<T>() {
            unsafe {
                Some(&mut *(self as *mut Element as *mut T))
            }
        } else {
            None
        }
    }
}

type NodeBox = Rc<RefCell<Node>>;
type NodeWeakBox = Weak<RefCell<Node>>;
type ElementBox = Rc<RefCell<Box<Element + 'static>>>;
type ElementWeakBox = Weak<RefCell<Box<Element + 'static>>>;

struct Node {
    pub parrent: Option<NodeWeakRef>,
    pub children: Vec<NodeRef>,
    pub rect: Rect,
}

pub struct NodeRef {
    node: NodeBox,
    element: ElementBox,
}

pub struct NodeWeakRef {
    node: NodeWeakBox,
    element: ElementWeakBox,
}

pub struct Tree {
    root: Option<NodeRef>,
}

impl NodeRef {
    fn new(node: NodeBox, element: ElementBox) -> NodeRef {
        NodeRef { 
            node: node,
            element: element,
        }
    }

    fn new_node(element: Box<Element + 'static>, parrent: Option<NodeWeakRef>) -> NodeRef {
        NodeRef::new(
            Rc::new(RefCell::new(Node {
                parrent: parrent,
                children: Vec::new(),
                rect: Rect::zero()
            })),
            Rc::new(RefCell::new(element)))
    }

    pub fn downgrade(&self) -> NodeWeakRef {
        NodeWeakRef::new(Rc::downgrade(&self.node), Rc::downgrade(&self.element))
    }

    pub fn children(&self) -> Ref<[NodeRef]> {
        Ref::map(self.node.borrow(), |n| &n.children[..])
    }

    pub fn add_child(&mut self, child: Box<Element + 'static>) -> NodeRef {
        let node = NodeRef::new_node(child, Some(self.downgrade()));
        self.node.borrow_mut().children.push(node.clone());
        node.init();
        node
    }

    pub fn parrent(&self) -> Option<NodeRef> {
        self.node.borrow().parrent.as_ref().map(|w| w.upgrade().unwrap())
    }

    pub fn rect(&self) -> Rect {
        self.node.borrow().rect
    }

    pub fn set_rect(&self, rect: Rect) {
        self.node.borrow_mut().rect = rect;
    }

    pub fn element(&self) -> Ref<Element + 'static> {
        Ref::map(self.element.borrow(), |b| &**b)
    }

    pub fn element_mut(&self) -> RefMut<Element + 'static> {
        RefMut::map(self.element.borrow_mut(), |b| &mut **b)
    }

    pub fn cast_element<T: Any>(&self) -> Option<Ref<T>> {
        let element = Ref::map(self.element.borrow(), |b| &**b);
        if element.can_cast::<T>() {
            Some(Ref::map(element, |e| e.cast_element_ref::<T>().unwrap()))
        } else {
            None
        }
    }

    pub fn cast_element_mut<T: Any>(&self) -> Option<RefMut<T>> {
        let element = RefMut::map(self.element.borrow_mut(), |b| &mut **b);
        if element.can_cast::<T>() {
            Some(RefMut::map(element, |e| e.cast_element_mut::<T>().unwrap()))
        } else {
            None
        }
    }

    fn init(&self) {
        self.element.borrow_mut().init(self.clone())
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.push_rect(self.node.borrow().rect);
        renderer.execute(self.element.borrow().render(self.clone()));

        for child in self.children().iter() {
            child.render(renderer);
        }

        renderer.pop_rect();
    }

    pub fn layout(&self, container: Size) {
        self.element.borrow_mut().layout(self.clone(), container);
    }

    pub fn measure(&self) -> Size {
        self.element.borrow().measure(self.clone())
    }
}

impl Clone for NodeRef {
    fn clone(&self) -> NodeRef {
        NodeRef::new(self.node.clone(), self.element.clone())
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &NodeRef) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
    }
}
impl Eq for NodeRef {}

impl NodeWeakRef {
    fn new(node: NodeWeakBox, element: ElementWeakBox) -> NodeWeakRef {
        NodeWeakRef { 
            node: node,
            element: element,
        }
    }

    pub fn empty() -> NodeWeakRef {
        NodeWeakRef { 
            node: Weak::new(),
            element: Weak::new(),
        }
    }

    pub fn upgrade(&self) -> Option<NodeRef> {
        Weak::upgrade(&self.node)
            .and_then(|n| 
                Weak::upgrade(&self.element)
                    .map(|e| (n, e)))
            .map(|(n, e)| NodeRef::new(n, e))
   }
}

impl Clone for NodeWeakRef {
    fn clone(&self) -> NodeWeakRef {
        NodeWeakRef::new(self.node.clone(), self.element.clone())
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn root(&self) -> Option<NodeRef> {
        self.root.as_ref().map(|r| r.clone())
    }

    pub fn set_root(&mut self, element: Option<Box<Element>>) -> Option<NodeRef> {
        self.root = element.map(|e| NodeRef::new_node(e, None));
        if let Some(root) = self.root.as_ref() {
            root.init();
        }
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct StubElement;

    impl Element for StubElement {
        fn init(&mut self, node: NodeRef) {

        }

        fn measure(&self, node: NodeRef) -> Size {
            Size::zero()
        }

        fn layout(&mut self, node: NodeRef, container: Size) {

        }

        fn render(&self, node: NodeRef) -> RenderCommandList {
            RenderCommandList::new()
        }
    }

    #[test]
    fn new_tree_has_empty_root() {
        let tree = Tree::new();
        assert!(tree.root() == None);
    }

    #[test]
    fn tree_set_root_sets_root() {
        let mut tree = Tree::new();
        tree.set_root(Some(Box::new(StubElement)));

        let root = tree.root();

        assert!(root.is_some());

        let root = root.unwrap();
        
        assert!(root.cast_element::<StubElement>().is_some());
    }

    #[test]
    fn tree_root_has_no_parrent() {
        let mut tree = Tree::new();
        tree.set_root(Some(Box::new(StubElement)));

        let root = tree.root().unwrap();

        assert!(root.parrent() == None);
    }

    #[test]
    fn add_child_adds_child() {
        let mut tree = Tree::new();
        tree.set_root(Some(Box::new(StubElement)));
        let mut root = tree.root().unwrap();

        assert_eq!(root.children().len(), 0);
        
        root.add_child(Box::new(StubElement));

        assert_eq!(root.children().len(), 1);
        {
            let child0 = root.children()[0].clone();
            assert!(child0.cast_element::<StubElement>().is_some());
        }

        root.add_child(Box::new(StubElement));
        
        assert_eq!(root.children().len(), 2);
        {
            let child0 = root.children()[0].clone();
            let child1 = root.children()[1].clone();
            assert!(child0.cast_element::<StubElement>().is_some());
            assert!(child1.cast_element::<StubElement>().is_some());
        }
    }
}