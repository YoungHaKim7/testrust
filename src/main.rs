// stcuct User // things
// enum Months // choices
// trait // verbs / adjectives

// traits   [power superpower]
use std::fmt::Debug;

#[derive(Debug)]
struct MyStruct {
    number: usize,
}

fn print_as_debug<T: Debug>(input: T) {
    println!("{:?}", input);
}

fn main() {}
