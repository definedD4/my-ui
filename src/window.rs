use glutin;

use primitives::*;
use tree;

pub struct Window {
    window: glutin::Window,
    tree: tree::ElementTree, 
}

pub struct WindowBuilder {
    size: Size,
    title: String,
}

impl Window {
    fn new(window: glutin::Window) -> Window {
        Window { 
            window: window,
            tree: tree::ElementTree::new(),
        }
    }

    fn set_content<T: tree::Element + 'static>(&mut self, element: T) {
        self.tree.set_root(Some(element));
    }


}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder { 
            size: Size(400.0, 600.0),
            title: String::new()
        }
    }

    pub fn build(self) -> Window {
        // TODO Add Result propagation
        let window = glutin::WindowBuilder::new()
            .with_dimensions(self.size.0 as u32, self.size.1 as u32)
            .with_title(self.title)
            .build().unwrap();
        Window::new(window)
    }

    pub fn with_size(mut self, size: Size) -> WindowBuilder {
        self.size = size;
        self
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> WindowBuilder {
        // TODO: Pass into to glutin builder
        self.title = title.into();
        self
    }
}