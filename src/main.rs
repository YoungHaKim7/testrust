use thiserror::Error;

#[derive(Debug)]
struct User {
    points: u32,
    age: u8,
}

impl User {
    fn try_new() -> Result<Self, CompanyError> {
        todo!()
    }
}

#[derive(Error, Debug)]
enum CompanyError {
    #[error("Not enough data")]
    NotEnoughtData,
    #[error("Too old: {0} Can't be over 120")]
    TooOld(u8),
    #[error("God {0}, should be under 10,000")]
    TooBig(u32),
    #[error("Must be under 120 and 10,000 points, got{0:?} instead")]
    TooBigAndTooOld(User), // UserBuilder
}

fn main() {
    let some_error = CompanyError::TooBig(20000);
    let second_error = CompanyError::NotEnoughtData;
    println!("{some_error}");
    println!("{second_error}");
}
