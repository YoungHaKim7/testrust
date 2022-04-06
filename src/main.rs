use std::num::ParseIntError;

fn try_to_make_number(int_input: &str, float_input: &str) -> Result<(), ParseIntError> {
    let my_number = int_input.parse::<i32>()?; // Ok(8) else return Err
    let other_number = float_input.parse::<f32>()?;
}

fn main() {}
