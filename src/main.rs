use std::error::Error;
use std::fmt::{Display, Formatter};

// anyhow 이 복잡한걸 편하게 만드는 extern-crate
#[derive(Debug)]
enum CompanyError {
    CouldntConnect,
    NotEnoughData,
    UserTimeOut,
}

impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Got a CompanyError")
    }
}

#[derive(Debug)]
struct BaseError;

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Got a BaseError")
    }
}

impl Error for CompanyError {}

impl Error for BaseError {}

// fn try_to_make_number(int_input: &str, float_input: &str) -> Result<(), ParseIntError> {
//     let my_number = int_input.parse::<i32>()?; // Ok(8) else return Err
//     let other_number = float_input.parse::<f32>()?;
//     Ok()
// }

fn main() {}
