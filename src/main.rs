fn add(one: u8, two: u8) {
    match one.checked_add(two) {
        Some(num) => println!("Got a good number: {num}"),
        None => println!("Got an error : can't add {one} with {two}"),
    }
}

fn main() {
    add(100, 100);
}
