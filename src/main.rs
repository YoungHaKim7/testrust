// boilerplate
// use anyhow; // 1.0.56
// use thiserror; // 1.0.30

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum CompanyError {
    CouldntConnect,
    NotEnoughData,
    UserTimeOut,
}

impl CompanyError {
    fn print_extra_detail(&self) {
        println!("Here is all the extra detail: blah blah blah blah");
    }
}
impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Got a CompanyError")
    }
}
#[derive(Debug)]
struct BaseError;

fn give_error(is_company_error: bool) -> Box<dyn Error> {
    if is_company_error {
        Box::new(CompanyError::CouldntConnect)
    } else {
        Box::new(BaseError)
    }
}

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

fn main() {
    let error_1 = give_error(true);
    let error_2 = give_error(false);

    if let Some(company_error) = error_1.downcast_ref::<CompanyError>() {
        company_error.print_extra_detail();
    } else {
        println!("{error_1}");
    }
}
