fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Print number {}", guess);

    // let name: f32 = 54.parse();
    // println!("Print name {}", name);

    let tup = (10, 10.4, "hello");
    let (x, y, z) = tup;

    println!("int {}", x);
    println!("float {}", y);
    println!("string {}", z);

    println!("int {}", tup.0);
    println!("float {}", tup.1);
    println!("string {}", tup.2);
}
