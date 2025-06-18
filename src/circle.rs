use crate::shape::Shape;

const PI: f64 = 3.14;

#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        (PI * &self.radius.powf(2.0)).trunc()
    }

    fn perimeter(&self) -> f64 {
        (2.0 * PI * &self.radius).trunc()
    }

    fn name(&self) -> &str {
        "circle"
    }
}
