use std::ops::{Add, Sub};
 
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

    pub fn zero() -> Point {
        Point::new(0f32, 0f32)
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
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

    pub fn zero() -> Size {
        Size::new(0.0, 0.0)
    }

    pub fn from_tuple(t: (f32, f32)) -> Size {
        Size::new(t.0, t.1)
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.w, self.h)
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

    pub fn to_tuple_argb(&self) -> (f32, f32, f32, f32){
        (self.a, self.r, self.g, self.b)
    }

    pub fn to_tuple_rgb(&self) -> (f32, f32, f32){
        (self.r, self.g, self.b)
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

    pub fn from_size(size: Size) -> Rect {
        Rect {
            pos: Point::zero(),
            size: size,
        }
    }

    pub fn from_bounds(right: f32, top: f32, left: f32, bottom: f32) -> Rect {
        Rect {
            pos: Point::new(left, top),
            size: Size::new(right - left, bottom - top),
        }
    }

    pub fn zero() -> Rect {
        Rect {
            pos: Point::zero(),
            size: Size::zero(),
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

    pub fn to_pos_size_tuple(&self) -> ((f32, f32), (f32, f32)) {
        (self.pos.to_tuple(), self.size.to_tuple())
    }

    pub fn transform_to_outer(&self, rect: Rect) -> Rect {
        Rect::pos_size(self.pos + rect.pos, rect.size)
    }

    pub fn transform_to_inner(&self, rect: Rect) -> Rect {
        Rect::pos_size(rect.pos - self.pos, rect.size)
    }

    pub fn expand(&self, margin: Thickness) -> Rect {
        Rect::from_bounds(self.right() + margin.right, self.top() - margin.top,
                          self.left() - margin.left, self.bottom() + margin.bottom)
    }

    pub fn inset(&self, margin: Thickness) -> Rect {
        Rect::from_bounds(self.right() - margin.right, self.top() + margin.top,
                          self.left() + margin.left, self.bottom() - margin.bottom)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Thickness {
    pub right: f32,
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
}

impl Thickness {
    pub fn new(right: f32, top: f32, left: f32, bottom: f32) -> Thickness {
        Thickness {
            right: right,
            top: top,
            left: left,
            bottom: bottom,
        }
    }

    pub fn hv(horizontal: f32, vertical: f32) -> Thickness {
        Thickness::new(horizontal, vertical, horizontal, vertical)
    }

    pub fn rect_in(&self, container: Size) -> Rect {
        Rect::from_bounds(container.w - self.right, self.top, self.left, container.h - self.bottom)
    }
}