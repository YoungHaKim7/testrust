// typeof
// downcasting -> dynamically making concrete

// &dyn Any


use std::any::{Any, type_name};

fn get_type_name<T: Any>(input: T) {
    let my_type = type_name::<T>();
    println!("{my_type}");

}

fn main() {}
