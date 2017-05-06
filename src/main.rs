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

struct TestElement {
    margin: Thickness,
}

impl Element for TestElement {
    fn init(&mut self, node: NodeRef){
        info!("[TestElement] Init");
    }

    fn measure(&self, node: NodeRef) -> Size {
        info!("[TestElement] Measure");
        Size::new(10.0, 70.0)
    }

    fn layout(&mut self, mut node: NodeRef, container: Size) {
        info!("[TestElement] Layout");
        node.set_rect(Rect::from_size(container).inset(self.margin));
    }

    fn render(&self, node: NodeRef) -> RenderCommandList {
        info!("[TestElement] Render");
        let mut cmd = RenderCommandList::new();
        cmd.add(RenderCommand::Clear(Color::argb(1.0, 1.0, 0.0, 0.0)));
        cmd
    }
}

struct TestBorder {
    margin: Thickness,
    node: NodeWeakRef,
    content: Option<NodeWeakRef>,
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

    fn render(&self, node: NodeRef) -> RenderCommandList {
        info!("[TestBorder] Render");
        let mut cmd = RenderCommandList::new();
        cmd.add(RenderCommand::Clear(Color::argb(1.0, 0.0, 1.0, 0.0)));
        cmd
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

    pub fn set_content(&mut self, content: Box<Element>) -> NodeRef {
        let content = self.node().add_child(content);
        self.content = Some(content.downgrade());
        content
    }

    fn node(&self) -> NodeRef {
        self.node.upgrade().unwrap()
    }

    fn content(&self) -> Option<NodeRef> {
        self.content.as_ref().map(|c| c.upgrade().unwrap())
    }
}

struct TestList {
    node: NodeWeakRef,
    child_items: Vec<NodeWeakRef>,
}

struct TestListItem {
    vertical_offset: f32,
}

impl TestList {
    pub fn new() -> TestList {
        TestList {
            node: NodeWeakRef::empty(),
            child_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, element: Box<Element>) -> NodeRef {
        let mut item = self.node.upgrade().unwrap().add_child(Box::new(TestListItem {
            vertical_offset: 0.0,
        }));
        self.child_items.push(item.downgrade());
        item.add_child(element)
    }
}

impl Element for TestList {
    fn init(&mut self, node: NodeRef){
        info!("[TestList] Init");
        self.node = node.downgrade();
    }

    fn measure(&self, node: NodeRef) -> Size {
        info!("[TestList] Measure");
        Size::zero() // Change
    }

    fn layout(&mut self, mut node: NodeRef, container: Size) {
        info!("[TestList] Layout");
        let rect = Rect::from_size(container);
        node.set_rect(rect);
        let mut offset_acc = 0.0;
        for item in &self.child_items {
            let item = item.upgrade().unwrap();
            let size = item.measure();
            item.cast_element_mut::<TestListItem>().unwrap().vertical_offset = offset_acc;
            offset_acc += size.h;
            item.layout(Size::new(container.w, size.h));
        }
    }

    fn render(&self, node: NodeRef) -> RenderCommandList {
        info!("[TestList] Render");
        let mut cmd = RenderCommandList::new();
        cmd.add(RenderCommand::Clear(Color::argb(1.0, 0.0, 1.0, 0.0)));
        cmd
    }
}

impl Element for TestListItem {
    fn init(&mut self, node: NodeRef){
        info!("[TestListItem] Init");
    }

    fn measure(&self, node: NodeRef) -> Size {
        info!("[TestListItem] Measure");
        let children = node.children();
        if children.len() != 1 {
            panic!("TestListItem: Child count must be equal to 1");
        }
        children[0].measure()
    }

    fn layout(&mut self, mut node: NodeRef, container: Size) {
        info!("[TestListItem] Layout");
        let rect = Rect::from_bounds(container.w, self.vertical_offset, 0.0, self.vertical_offset + container.h);
        node.set_rect(rect);
        
        let children = node.children();
        if children.len() != 1 {
            panic!("TestListItem: Child count must be equal to 1");
        }
        info!("[TestListItem] Rect: {:?}", rect);
        children[0].layout(rect.size);
    }

    fn render(&self, node: NodeRef) -> RenderCommandList {
        info!("[TestListItem] Render");
        RenderCommandList::new()
    }
}

fn main() {
    my_logger::init(log::LogLevel::Debug);

    let mut window = window::WindowBuilder::new()
        .with_title("My new window!")
        .build();

    {
        let mut border = window.set_content(Box::new(TestBorder::new(Thickness::hv(4.0, 8.0))));

        let mut list = border.cast_element_mut::<TestBorder>().unwrap().set_content(Box::new(TestList::new()));

        list.cast_element_mut::<TestList>().unwrap().add_item(Box::new(TestElement {
            margin: Thickness::hv(6.0, 6.0),
        }));
        
        list.cast_element_mut::<TestList>().unwrap().add_item(Box::new(TestElement {
            margin: Thickness::hv(6.0, 6.0),
        }));
    }

    window.run_loop();
}