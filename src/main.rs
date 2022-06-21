use std::env::args;

enum Letters {
    Capitalize,
    Lowerase,
    Nothing,
}

fn main() {
    let mut changes = Letters::Nothing;

    let input = args().collect()::<Vec<String>>();
    // let input: Vec<String> = args().collect();
}
