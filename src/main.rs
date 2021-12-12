// Collection types:
// ARRAYS     [ ] all the same type

// &str
fn main() {
    let array = ["One", "Two"]; // [&str; 2]
    let array2 = ["One", "Two", "Five"]; // [&str; 3]

    println!("Is array the same as array2? {}", array == array2); // true false
}
