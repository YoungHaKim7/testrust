use std::num::ParseIntError;

fn try_to_make_number(input: &str) -> Result<i32, ParseIntError> {
    let my_number = input.parse::<i32>()?; // Ok(8) else return Err
    let other_number = input.parse::<f32>()?;
}

fn main() {}
