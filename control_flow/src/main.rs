fn main() {
    let num = 5;

    if num < 6 {
        println!("Less than 6");
    } else {
        println!("More than 6");
    }

    looping();

    reverse();
}

fn looping() {
    let a = [1, 2, 3, 4, 5, 6, 7];

    for element in a.iter() {
        println!("Print the element {}", element);
    }
}

fn reverse() {
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("END!");
}
