fn main() {
    let mut user1 = User {
        username: String::from("dimasjt"),
        age: 21,
    };

    println!("username {}", user1.username);
    println!("age {}", user1.age);
}

struct User {
    username: String,
    age: u32,
}
