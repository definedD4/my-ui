use glutin;

use primitives::*;
use tree;
use element::*;

pub struct Window {
    window: glutin::Window,
    tree: ElementTree,
}

pub struct WindowBuilder {
    size: Size,
    title: String,
    content: Option<Box<Element>>,
}

impl Window {
    fn new(window: glutin::Window) -> Window {
        Window { 
            window: window,
            tree: tree::Tree::new(),
        }
    }

    fn set_content(&mut self, element: Box<Element>) {
        self.tree.set_root_element(element);
    }


}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder { 
            size: Size(600.0, 400.0),
            title: String::new(),
            content: None,
        }
    }

    pub fn build(self) -> Window {
        // TODO Add Result propagation
        let mut window = glutin::WindowBuilder::new()
            .with_dimensions(self.size.0 as u32, self.size.1 as u32)
            .with_title(self.title)
            .build()
            .map(|w| Window::new(w))
            .unwrap();
        if let Some(content) = self.content {
            window.set_content(content);
        }
        window
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

    pub fn with_content<T: Element + 'static>(mut self, content: T) -> WindowBuilder {
        self.content = Some(Box::new(content));
        self
    }
}