use primitives::*;
use glium;
use glium::glutin;

pub struct Renderer<'a> {
    surface: &'a mut glium::Frame,
    rendering_context: &'a RenderingContext,
    size: Size,
    viewport: Rect,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct RenderingContext {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl<'a> Renderer<'a> {
    pub fn new(surface: &'a mut glium::Frame, rendering_context: &'a RenderingContext, size: Size, viewport: Rect) -> Renderer<'a> {
        Renderer { 
            surface: surface,
            size: size,
            viewport: viewport,
            rendering_context: rendering_context,
        }
    }

    fn to_relative(&self, rect: Rect) ->((f32, f32), (f32, f32)) {
        let ((x, y), (w, h)) = self.viewport.transform_to_outer(rect).to_pos_size_tuple();
        ((x / self.size.w * 2.0 - 1.0, 1.0 - y / self.size.h * 2.0), (w / self.size.w * 2.0, h / self.size.h * 2.0))
    }

    pub fn sub_renderer<'b: 'a>(&'b mut self, rect: Rect) -> Renderer<'b> {
        Renderer::new(self.surface, self.rendering_context, self.size, self.viewport.transform_to_outer(rect))
    }

    pub fn clear(&mut self, color: Color) {
        let size = self.viewport.size.clone();
        self.rect(Rect::from_size(size), color);
    }

    pub fn rect(&mut self, rect: Rect, color: Color) {
        let (pos, size) = self.to_relative(rect);
        self.rendering_context.draw_rect(self.surface, pos, size, color);
    }
}

impl RenderingContext {
    pub fn new(display: &glium::Display) -> RenderingContext {
        let vertex_buffer = {        
            glium::VertexBuffer::new(display,
                &[
                    Vertex { position: [  0.0,  0.0] },
                    Vertex { position: [  1.0,  0.0] },
                    Vertex { position: [  1.0, -1.0] },
                    Vertex { position: [  0.0, -1.0] },
                ]
            ).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                                &[0u16, 1, 2, 0, 2, 3]).unwrap();
        let program = program!(display,
            140 => {
                vertex: "
                    #version 140
                    uniform vec2 pos;
                    uniform vec2 size;
                    uniform vec3 color;
                    in vec2 position;
                    out vec3 vColor;
                    void main() {
                        gl_Position = vec4(position.x * size.x + pos.x, position.y * size.y + pos.y, 0.0, 1.0);
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 140
                    in vec3 vColor;
                    out vec4 f_color;
                    void main() {
                        f_color = vec4(vColor, 1.0);
                    }
                "
            }/*,

            110 => {
                vertex: "
                    #version 110
                    uniform mat4 matrix;
                    attribute vec2 position;
                    attribute vec3 color;
                    varying vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 110
                    varying vec3 vColor;
                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },

            100 => {
                vertex: "
                    #version 100
                    uniform lowp mat4 matrix;
                    attribute lowp vec2 position;
                    attribute lowp vec3 color;
                    varying lowp vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 100
                    varying lowp vec3 vColor;
                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },*/
        ).unwrap();

        RenderingContext {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
        }
    }

    pub fn draw_rect(&self, surface: &mut glium::Frame, pos: (f32, f32), size: (f32, f32), color: Color) {
        use glium::Surface;

        info!("[Renderer] Drawing rect with pos: {:?} size: {:?}", &pos, &size);

        let uniforms = uniform! {
            pos: pos,
            size: size,
            color: color.to_tuple_rgb()
        };

        surface.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default()).unwrap();
    }
}