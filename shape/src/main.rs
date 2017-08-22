extern crate shape;

use shape::Rectangle;
use shape::Circle;

fn main() {
    let rect1 = Rectangle { width: 10, height: 10 };

    println!("Print rect1: {:?}", rect1);
    println!("Print rect1 area: {}", rect1.area());

    let circle1 = Circle { diameter: 14.0 };

    println!("Print circle1: {:?}", circle1);
    println!("Print circle1 area: {}", circle1.area());
}
