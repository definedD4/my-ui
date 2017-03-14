#![feature(ptr_eq)]

#[macro_use]
extern crate glium;

#[macro_use]
extern crate log;

mod primitives;
mod tree;
mod window;
mod element;
mod property;
mod render;
mod my_logger;

use primitives::*;
use element::*;
use render::*;

struct TestElement {
    margin: Thickness,
}

impl Element for TestElement {
    fn init(&mut self, node: ElementNodeRef){
        info!("[Element] Init");
    }

    fn measure(&self, node: ElementNodeRef) -> Size {
        info!("[Element] Measure");
        Size::zero()
    }

    fn layout(&mut self, node: ElementNodeRef, container: Size) {
        info!("[Element] Layout");
        node.data_mut().set_rect(Rect::from_size(container).inset(self.margin));
    }

    fn render(&self, node: ElementNodeRef, renderer: &mut Renderer) {
        info!("[Element] Render");
        renderer.clear(Color::argb(1.0, 1.0, 0.0, 0.0))
    }
}

fn main() {
    my_logger::init(log::LogLevel::Debug);

    let mut window = window::WindowBuilder::new()
        .with_title("My new window!")
        .with_content(TestElement {
            margin: Thickness::hv(8.0, 4.0),
        })
        .build();

    window.run_loop();
}