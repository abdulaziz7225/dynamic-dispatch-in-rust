use crate::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        (&self.width * &self.height).trunc()
    }

    fn perimeter(&self) -> f64 {
        (2.0 * (&self.width + &self.height)).trunc()
    }

    fn name(&self) -> &str {
        "rectangle"
    }
}
