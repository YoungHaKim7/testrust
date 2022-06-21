use std::env::args;

enum Letters {
    Capitalize,
    Lowerase,
    Nothing,
}

fn main() {
    let mut changes = Letters::Nothing;
    let input: Vec<String> = args().collect();

    if input.len() > 1 {
        match input[1].as_str() {
            // PartialEq 랑 다른거임.
            "capital" => changes = Letters::Capitalize,
            "lowercase" => changes = Letters::Lowerase,
            _ => {}
        }
    }

    // programname - capitalize
    for word in args().skip(2) {
        match changes {
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowerase => println!("{}", word.to_lowercase()),
            Letters::Nothing => println!("{word}"),
        }
    }
}
