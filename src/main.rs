// It's trivial to copy the bytes

// Ownership and copy types
fn prints_number(number: i32) {
    println!("{}", number);
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);
}
