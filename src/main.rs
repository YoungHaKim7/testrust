use std::any::{type_name, Any};

struct MyType;

fn try_do_get_string(input: &dyn Any) {
    // trait object
    if let Some(a_string) = input.downcast_ref::<String>() {
        println!("We got a String! {a_string}");
    } else {
        println!("We got something else");
    }
}

fn main() {
    try_do_get_string(&9);

    try_do_get_string(&String::from("Hello there"));
}
