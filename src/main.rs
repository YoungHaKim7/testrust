// stcuct User // things
// enum Months // choices
// trait // verbs / adjectives

// traits   [power superpower]
#[derive(Debug)]
struct MyStruct {
    number: usize,
}

fn print_as_debug<T: std::fmt::Debug>(input: T) {
    println!("{input:?}")
}

fn main() {}
