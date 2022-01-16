// stcuct User // things
// enum Months // choices
// trait // verbs / adjectives

// traits   [power superpower]
use std::fmt::Debug;

#[derive(Debug)]
struct MyStruct {
    number: usize,
}

fn print_as_debug<T>(input: T)
where
    T: Debug,
{
    println!("{:?}", input);
}

fn main() {}
