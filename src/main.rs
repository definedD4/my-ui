#![feature(ptr_eq)]
#![feature(get_type_id)]

#[macro_use]
extern crate glium;

#[macro_use]
extern crate log;

mod primitives;
mod tree;
mod window;
mod property;
mod render;
mod my_logger;

use std::any::Any;

use primitives::*;
use render::*;
use tree::*;

struct TestBorder {
    margin: Thickness,
    node: NodeWeakRef,
    content: Option<NodeWeakRef>,
}

struct TestElement {
    margin: Thickness,
}

impl Element for TestElement {
    fn init(&mut self, node: NodeRef){
        info!("[TestElement] Init");
    }

    fn measure(&self, node: NodeRef) -> Size {
        info!("[TestElement] Measure");
        Size::zero()
    }

    fn layout(&mut self, mut node: NodeRef, container: Size) {
        info!("[TestElement] Layout");
        node.set_rect(Rect::from_size(container).inset(self.margin));
    }

    fn render(&self, node: NodeRef, renderer: &mut Renderer) {
        info!("[TestElement] Render");
        renderer.clear(Color::argb(1.0, 1.0, 0.0, 0.0))
    }
}

impl Element for TestBorder {
    fn init(&mut self, node: NodeRef){
        info!("[TestBorder] Init");
        self.node = node.downgrade();
    }

    fn measure(&self, node: NodeRef) -> Size {
        info!("[TestBorder] Measure");
        Size::zero()
    }

    fn layout(&mut self, mut node: NodeRef, container: Size) {
        info!("[TestBorder] Layout");
        let inner = Rect::from_size(container).inset(self.margin);
        node.set_rect(inner);
        if let Some(content) = self.content() {
            content.layout(inner.size);
        }
    }

    fn render(&self, node: NodeRef, renderer: &mut Renderer) {
        info!("[TestBorder] Render");
        renderer.clear(Color::argb(1.0, 0.0, 1.0, 0.0));
        if let Some(content) = self.content() {
            content.render(renderer);
        }
    }
}

impl TestBorder {
    pub fn new(margin: Thickness) -> TestBorder {
        TestBorder {
            margin: margin,
            node: NodeWeakRef::empty(),
            content: None,
        }
    }

    pub fn set_content(&mut self, content: Box<Element>) {
        let content = self.node().add_child(content);
        self.content = Some(content.downgrade());
    }

    fn node(&self) -> NodeRef {
        self.node.upgrade().unwrap()
    }

    fn content(&self) -> Option<NodeRef> {
        self.content.as_ref().map(|c| c.upgrade().unwrap())
    }
}


fn main() {
    my_logger::init(log::LogLevel::Debug);

    let mut window = window::WindowBuilder::new()
        .with_title("My new window!")
        .build();

    {
        let mut border = window.set_content(Box::new(TestBorder::new(Thickness::hv(4.0, 8.0))));

        border.cast_element_mut::<TestBorder>().unwrap().set_content(Box::new(TestElement {
            margin: Thickness::hv(6.0, 6.0),
        }));
    }

    window.run_loop();
}