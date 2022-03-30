// typeof
// downcasting -> dynamically making concrete

// &dyn Any


use std::any::{Any, type_name};

struct MyType;

fn get_type_names<T: Any, U: Any>(_: T, _: U) {
    let type_of_t = type_name::<T>();
    let type_of_u = type_name::<U>();
    println!("First type: {type_of_t}");
    println!("second type: {type_of_u}");


}

fn main() {
    get_type_names(8, true);
    get_type_names(vec!['a'], MyType);
}
