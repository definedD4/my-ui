use primitives::*;
use tree;
use render::Renderer;

pub trait Element {
    fn init(&mut self, node: ElementNodeRef);
    fn measure(&self, node: ElementNodeRef) -> Size;
    fn layout(&mut self, node: ElementNodeRef, container: Size);
    fn render(&self, node: ElementNodeRef, renderer: &mut Renderer);
}

pub type ElementTree = tree::Tree<ElementContext>;

pub type ElementNodeRef = tree::NodeRef<ElementContext>;

pub type ElementNodeWeakRef = tree::NodeWeakRef<ElementContext>;

pub struct ElementContext {
    element: Box<Element>,
    node: ElementNodeWeakRef,
    rect: Rect,
}

impl ElementContext {
    fn new(element: Box<Element>, node: ElementNodeWeakRef) -> ElementContext {
        ElementContext {
            element: element,
            node: node,
            rect: Rect::zero(),
        }
    }

    fn set_node(&mut self, node: ElementNodeWeakRef) {
        self.node = node;
    }

    pub fn node(&self) -> ElementNodeRef {
        self.node.upgrade().unwrap()
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn init(&mut self) {
        let node = self.node();
        self.element.init(node);
    }

    pub fn measure(&self) -> Size {
        self.element.measure(self.node())
    }

    pub fn layout(&mut self, container: Size) {
        let node = self.node();
        self.element.layout(node, container);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let rect = self.rect;
        // let sub_renderer = renderer.sub_renderer(rect);
        self.element.render(self.node(), renderer);
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