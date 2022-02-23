// Rc
use std::rc::Rc;

fn takes_a_string(input: Rc<String>) {
    println!("{input}");
}

fn also_takes_a_string(input: Rc<String>) {
    println!("{input}");
}

fn main() {
    let my_string = Rc::new("Hello there".to_string());
//    takes_a_string(Rc::clone(&my_string)); // Associated function syntex
//    also_takes_a_string(Rc::clone(&my_string));
    takes_a_string(my_string.clone());
    takes_a_string(my_string.clone());
}
