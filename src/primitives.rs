use std::ops::Add;
 
#[derive(Copy, Clone, Debug)]
pub struct Point(pub f32, pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Size(pub f32, pub f32);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}