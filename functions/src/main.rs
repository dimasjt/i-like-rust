fn main() {
    println!("Hello, world!");
    another_func(30, 10);

    let plus = add(10, 10);
    println!("return value {}", plus);
}

fn another_func(x: i32, y: i32) {
    println!("Another func!");

    println!("Print x {}", x);
    println!("Print y {}", y);
}

// statement function, because return a value
fn add(x: i32, y: i32) -> i32 {
    x + y
}

