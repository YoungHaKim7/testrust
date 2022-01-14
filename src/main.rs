// The question mark operator
// ?
use std::num::ParseIntError;

// gives the error to the caller
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // return Error
    Ok(parsed_number)
}

fn main() {
    for item in vec!["Senven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
