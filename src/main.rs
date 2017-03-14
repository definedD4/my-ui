#![feature(ptr_eq)]

#[macro_use]
extern crate glium;

mod primitives;
mod tree;
mod window;
mod element;
mod property;
mod render;

use primitives::*;
use element::*;
use render::*;

struct TestElement {

}

impl Element for TestElement {
    fn init(&mut self, node: ElementNodeRef){

    }

    fn measure(&self, node: ElementNodeRef) -> Size {
        Size::zero()
    }

    fn layout(&mut self, node: ElementNodeRef, container: Size) {

    }

    fn render(&self, node: ElementNodeRef, renderer: &mut Renderer) {
        renderer.clear(Color::argb(1.0, 1.0, 0.0, 0.0))
    }
}

fn main() {
    let mut window = window::WindowBuilder::new()
        .with_title("My new window!")
        .with_content(TestElement {})
        .build();

    window.run_loop();
}