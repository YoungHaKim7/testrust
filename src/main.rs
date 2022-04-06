use std::num::ParseIntError;

fn try_to_make_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() {}
