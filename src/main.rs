use std::any::{type_name, Any};

struct MyType;

fn do_stuff_depending(input: &dyn Any) {
    // trait object
    if input.is::<String>() {
        println!("We got a String");
    } else if input.is::<i32>() {
        println!("We have a number");
    } else {
        println!("Don't know what it is");
    }
}

fn main() {
    do_stuff_depending(&8);
    do_stuff_depending(&String::from("I am a String"));
    do_stuff_depending(&MyType);
}
