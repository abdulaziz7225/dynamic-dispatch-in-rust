use crate::shape::Shape;

#[derive(Debug)]
pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Triangle {
        Triangle { a, b, c }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s: f64 = &self.perimeter() / 2.0;
        let result = s * (s - &self.a) * (s - &self.b) * (s - &self.c);
        result.sqrt().trunc()
    }

    fn perimeter(&self) -> f64 {
        &self.a + &self.b + &self.c
    }

    fn name(&self) -> &str {
        "triangle"
    }
}
