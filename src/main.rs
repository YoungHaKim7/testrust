struct Book;

use std::fmt::Display;

fn give_thing<T: Display>(input: T) -> T {
    //T
    println!("{}", input); // Display
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book);
    println!("{}", x);
    println!("{}", y);
}
