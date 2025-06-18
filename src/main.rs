mod circle;
mod rectangle;
mod shape;
mod triangle;

use circle::Circle;
use rectangle::Rectangle;
use shape::Shape;
use triangle::Triangle;

fn main() {
    let circle = Circle::new(10.0);
    let rectangle = Rectangle::new(5.0, 6.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0);

    let mut different_shapes: Vec<Box<dyn Shape>> = Vec::new();
    different_shapes.push(Box::new(circle));
    different_shapes.push(Box::new(rectangle));
    different_shapes.push(Box::new(triangle));

    for element in &different_shapes {
        println!("{element:?}");
        println!("Area of {0}: {1}", element.name(), element.area());
        println!("Perimeter of {0}: {1}\n", element.name(), element.perimeter());
    }
}
