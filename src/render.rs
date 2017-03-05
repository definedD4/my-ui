use primitives::*;
use glutin;

pub trait Renderer {
    fn clear(&self, color: Color);
    fn rect(&self, rect: Rect);
}

pub struct GlRenderer<'a> {
    window: &'a glutin::Window,
    size: Size,
    viewport: Rect,
}

impl<'a> GlRenderer<'a> {
    pub fn new(window: &'a glutin::Window, size: Size, viewport: Rect) -> GlRenderer<'a> {
        GlRenderer { window: window }
    }

    fn transform(&self, rect: Rect) {
        
    }
}

impl<'a> Renderer for GlRenderer<'a> {
    fn clear(&self: color: Color) {

    }
}