use std::ops::Add;
 
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn from_tuple(t: (f32, f32)) -> Point {
        Point::new(t.0, t.1)
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub w: f32,
    pub h: f32
}

impl Size {
    pub fn new(w: f32, h: f32) -> Size {
        Size { w: w, h: h }
    }

    pub fn from_tuple(t: (f32, f32)) -> Size {
        Size::new(t.0, t.1)
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.w, self.h)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color{
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn argb(a: f32, r: f32, g: f32, b: f32) -> Color {
        Color {
            a: a,
            r: r,
            g: g,
            b: b
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}

impl Rect {
    pub fn pos_size(pos: Point, size: Size) -> Rect {
        Rect {
            pos: pos,
            size: size
        }
    }

    pub fn left(&self) -> f32 {
        self.pos.x
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.size.w
    }

    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.h
    }

    pub fn top(&self) -> f32 {
        self.pos.y
    }
}