use glium;
use glium::glutin;

use primitives::*;
use tree::*; 
use render::*;

pub struct Window {
    display: glium::Display,
    rendering_context: RenderingContext,
    tree: Tree,
    size: Size,
    title: String,
}

pub struct WindowBuilder {
    size: Size,
    title: String,
    content: Option<Box<Element>>,
}

impl Window {
    fn new(display: glium::Display, size: Size, title: String) -> Window {
        let rendering_context = RenderingContext::new(&display);
        Window { 
            display: display,
            rendering_context: rendering_context,
            tree: Tree::new(),
            size: size,
            title: title,
        }
    }

    pub fn set_content(&mut self, element: Box<Element>) -> NodeRef {
        self.tree.set_root(Some(element)).unwrap()
    }

    pub fn run_loop(&mut self) {
        use glium::glutin::Event::*;

        'main: loop {
            self.layout_content();
            self.render();
            for event in self.display.wait_events() {
                match event {
                    Closed => {
                        info!("[Window] Event: Closed");
                        break 'main;
                    },
                    Refresh => { 
                        info!("[Window] Event: Refresh");
                        self.render();
                    },
                    Resized(w, h) => {
                        info!("[Window] Event: Resized ({}, {})", w, h);

                        self.size = Size::new(w as f32, h as f32);
                        self.layout_content();
                        self.render();
                    }
                    _ => {},
                }
            }
        }
    }

    fn render(&self) {
        use glium::Surface;
        if let Some(root) = self.tree.root() {
            let mut suface = self.display.draw();
            suface.clear_color(1.0, 1.0, 1.0, 1.0);
            root.render(&mut Renderer::new(&mut suface, &self.rendering_context, self.size, Rect::from_size(self.size)));
            suface.finish().unwrap();
        }

        //self.display.swap_buffers().unwrap();
    }

    fn layout_content(&self) {
        if let Some(mut root) = self.tree.root() {
            root.layout(self.size);
        }
    }
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder { 
            size: Size::new(600.0, 400.0),
            title: String::new(),
            content: None,
        }
    }

    pub fn build(self) -> Window {
        use glium::DisplayBuild;

        // TODO Add Result propagation
        let mut window = glutin::WindowBuilder::new()
            .with_dimensions(self.size.w as u32, self.size.h as u32)
            .with_title(self.title.clone())
            .with_vsync()
            .build_glium()
            .map(|display| Window::new(display, self.size, self.title.clone()))
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