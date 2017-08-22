#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn square(size: f32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}

fn main() {

    let rect1 = Rectangle { width: 10.0, height: 10.0 };
    let square = Rectangle::square(20f32);

    println!("The area is {}", rect1.area());
    println!("rectangle: {:#?}", rect1);

    println!("Square area : {}", square.area());
    println!("Square : {:#?}", square);
}

