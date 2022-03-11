// impl trait
use std::fmt::Display;

fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

fn impl_function(input: impl Display) {
    println!("{input}")
}
fn main () {
//    generic_function::<u8>(8);
    impl_function::<u8>(8); 
}
